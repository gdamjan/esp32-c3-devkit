#![no_main]
#![no_std]

use esp_backtrace as _;
use esp_println::println;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        // Configure the CPU to run at the maximum frequency.
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();
    println!("Starting…");

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO0, Level::High);

    let delay = Delay::new();
    loop {
        delay.delay(500.millis());
        println!("Hello world!");
        led.toggle();
    }
}
