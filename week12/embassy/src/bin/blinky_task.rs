#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure the pin for output
    // We are going to give it a static lifetime (meaning it never dies)
    // because arguments to tasks should be static
    let led: Output<'static> = Output::new(p.PIN_10, Level::Low);

    // Move the configured Output into the task
    spawner.spawn(blinky(led)).unwrap();

    loop {
        info!("Hello from main!");
        Timer::after_secs(10).await;
    }
}

#[embassy_executor::task]
async fn blinky(mut led: Output<'static>) {
    loop {
        info!("led on!");
        led.set_high();
        Timer::after_secs(1).await;

        info!("led off!");
        led.set_low();
        Timer::after_secs(1).await;
    }
}
