use std::time::Duration;

use esp_idf_hal::{prelude::Peripherals, gpio::PinDriver};
use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut led = PinDriver::output(peripherals.pins.gpio19)?;
    loop {
        std::thread::sleep(Duration::from_millis(200));
        led.set_high()?;
        std::thread::sleep(Duration::from_millis(200));
        led.set_low()?;
    }
}
