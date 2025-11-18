#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    // Move the configured Output into the task
    spawner.spawn(task1()).unwrap();
    spawner.spawn(task2()).unwrap();

    loop {
        info!("Hello from main!");
        Timer::after_secs(10).await;
    }
}

#[embassy_executor::task]
async fn task1() {
    loop {
        info!("Hello from task 1");
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn task2() {
    loop {
        info!("Hello from task 2");
        Timer::after_secs(3).await;
    }
}
