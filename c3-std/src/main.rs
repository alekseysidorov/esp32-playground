use std::{net::TcpListener, time::Duration};

use c3_std::Wifi;
use esp_idf_hal::{gpio::PinDriver, prelude::*};
use esp_idf_svc::{eventloop::EspSystemEventLoop, log::EspLogger, wifi::WifiEvent};

// If using the `binstart` feature of `esp-idf-sys`, always keep this module importe
use esp_idf_sys as _;
use log::info;

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let sysloop = EspSystemEventLoop::take()?;
    let mut wifi = Wifi::new(peripherals.modem, sysloop.clone())?;
    wifi.establish_softap()?;

    let _handle = sysloop.subscribe(|event: &WifiEvent| {
        info!("Got event: {:?}", event);
    })?;

    std::thread::spawn(|| -> anyhow::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:80")?;
        info!("Bound TCP on: {:?}", listener.local_addr());

        for stream in listener.incoming() {
            let stream = stream?;
            info!("Got connection: {:?}", stream.peer_addr());
        }

        Ok(())
    });

    let mut led = PinDriver::output(peripherals.pins.gpio7)?;
    loop {
        std::thread::sleep(Duration::from_millis(200));
        led.set_high()?;
        std::thread::sleep(Duration::from_millis(200));
        led.set_low()?;
    }
}
