#![no_main]
#![no_std]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, delay::Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();
    log::info!("Starting…");

    loop {
        delay.delay(500.millis());
        log::info!("Hello world!");
    }
}
