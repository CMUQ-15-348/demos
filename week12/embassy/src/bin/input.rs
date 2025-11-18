#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure the pin for output
    // We are going to give it a static lifetime (meaning it never dies)
    // because arguments to tasks should be static
    let blinking_led: Output<'static> = Output::new(p.PIN_10, Level::Low);
    let changing_led: Output<'static> = Output::new(p.PIN_13, Level::Low);

    // Configure the input button
    let button: Input<'static> = Input::new(p.PIN_2, Pull::Down);

    // Spawn both tasks
    spawner.spawn(blinky(blinking_led)).unwrap();
    spawner.spawn(input_reader(button, changing_led)).unwrap();
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

#[embassy_executor::task]
async fn input_reader(mut button: Input<'static>, mut led: Output<'static>) {
    loop {
        // Do a debounced read
        button.wait_for_high().await;
        Timer::after_millis(5).await;
        button.wait_for_low().await;
        Timer::after_millis(5).await;
        // After the release, toggle the LED
        led.toggle();
    }
}
