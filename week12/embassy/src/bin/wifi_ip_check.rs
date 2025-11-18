//! This example uses the RP Pico W board Wifi chip (cyw43).
//! Connects to Wifi network and makes a web request to get the internet facing IP

#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

use core::str::from_utf8;

use cyw43::JoinOptions;
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::dns::DnsSocket;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::{Config, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use panic_probe as _;
use reqwless::client::{HttpClient, TlsConfig, TlsVerify};
use reqwless::request::Method;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const WIFI_NETWORK: &str = ""; // change to your network SSID
const WIFI_PASSWORD: &str = ""; // change to your network password

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");

    let p = embassy_rp::init(Default::default());
    let mut rng = RoscRng;

    let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");
    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download 43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download 43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(cyw43_task(runner)).unwrap();

    control.init(clm).await;

    let mac = control.address().await;
    info!(
        "Pico W MAC address: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    );

    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());
    // Use static IP configuration instead of DHCP
    //let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 69, 2), 24),
    //    dns_servers: Vec::new(),
    //    gateway: Some(Ipv4Address::new(192, 168, 69, 1)),
    //});

    // Generate random seed
    let seed = rng.next_u64();

    // Init network stack
    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(net_task(runner)).unwrap();

    // Connect to the WIFI network
    let mut attempts = 0u8;
    loop {
        match control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => break,
            Err(err) => {
                attempts = attempts.saturating_add(1);
                warn!(
                    "join failed ({}), attempt {}. Retrying…",
                    err.status, attempts
                );
                Timer::after(Duration::from_millis(500)).await;
            }
        }
    }

    info!("waiting for link...");
    stack.wait_link_up().await;

    info!("waiting for DHCP...");
    stack.wait_config_up().await;

    // And now we can use it!
    info!("Stack is up!");

    if let Some(cfg) = stack.config_v4() {
        info!("IPv4: {}", cfg.address);
        if let Some(gw) = cfg.gateway {
            info!("GW: {}", gw);
        }
        if !cfg.dns_servers.is_empty() {
            info!("DNS: {:?}", cfg.dns_servers);
        }
    }

    let mut rx_buffer = [0; 8192];
    let mut tls_read_buffer = [0; 16640];
    let mut tls_write_buffer = [0; 16640];

    let client_state = TcpClientState::<1, 1024, 1024>::new();
    let tcp_client = TcpClient::new(stack, &client_state);
    let dns_client = DnsSocket::new(stack);
    let tls_config = TlsConfig::new(
        seed,
        &mut tls_read_buffer,
        &mut tls_write_buffer,
        TlsVerify::None,
    );

    let mut http_client = HttpClient::new_with_tls(&tcp_client, &dns_client, tls_config);
    let url = "https://ifconfig.co/ip";

    info!("connecting to {}", &url);

    let mut request = match http_client.request(Method::GET, &url).await {
        Ok(req) => req,
        Err(e) => {
            error!("HTTP request build failed: {:?}", e);
            return;
        }
    };

    let resp = match request.send(&mut rx_buffer).await {
        Ok(r) => r,
        Err(e) => {
            error!("HTTP send failed: {:?}", e);
            return;
        }
    };

    info!("HTTP status = {}", resp.status);

    let body_bytes = match resp.body().read_to_end().await {
        Ok(b) => b,
        Err(e) => {
            error!("Body read err: {:?}", e);
            return;
        }
    };

    match from_utf8(body_bytes) {
        Ok(text) => info!("Public IP: {}", text.trim()),
        Err(_) => warn!("Non-UTF8 body ({} bytes)", body_bytes.len()),
    }

    loop {
        Timer::after(Duration::from_secs(5)).await;
    }
}
