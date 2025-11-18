#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::gpio::{Level, Output};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

type TimeType = Mutex<ThreadModeRawMutex, Option<u64>>;
static TIME: TimeType = Mutex::new(None);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure the pin for output
    // We are going to give it a static lifetime (meaning it never dies)
    // because arguments to tasks should be static
    let led: Output<'static> = Output::new(p.PIN_10, Level::Low);

    // Configure input pins for input
    let down: Input<'static> = Input::new(p.PIN_2, Pull::Down);
    let up: Input<'static> = Input::new(p.PIN_3, Pull::Down);

    // Set our initial delay
    // We do this inside its own scope so that the lock is automatically dropped afterwards
    {
        *(TIME.lock().await) = Some(1000);
    }

    // Move the configured Output into the task
    spawner.spawn(lights(&TIME, led)).unwrap();
    spawner.spawn(input_reader(&TIME, down, 100)).unwrap();
    spawner.spawn(input_reader(&TIME, up, -100)).unwrap();

    loop {
        info!("Hello from main!");
        Timer::after_secs(10).await;
    }
}

#[embassy_executor::task]
async fn lights(time: &'static TimeType, mut led: Output<'static>) {
    loop {
        info!("led on!");
        led.set_high();

        let val = { time.lock().await.unwrap() };
        Timer::after_millis(val).await;

        info!("led off!");
        led.set_low();

        let val = { time.lock().await.unwrap() };
        Timer::after_millis(val).await;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn input_reader(time: &'static TimeType, mut button: Input<'static>, val: i32) {
    loop {
        // Do a debounced read
        button.wait_for_high().await;
        Timer::after_millis(5).await;
        button.wait_for_low().await;
        Timer::after_millis(5).await;
        // After the release, adjust the shared delay time variable
        {
            let mut time_unlocked = time.lock().await;
            let cur = time_unlocked.unwrap();
            let new = add_u64_i32_saturating(cur, val);
            *time_unlocked = Some(new);
            info!("New delay: {}", new);
        }
    }
}

fn add_u64_i32_saturating(a: u64, b: i32) -> u64 {
    if b >= 0 {
        a.saturating_add(b as u64)
    } else {
        // negate safely: convert the magnitude to u64, then saturating_sub
        // This safely ensures the result can never go below 0
        a.saturating_sub((-b) as u64)
    }
}
