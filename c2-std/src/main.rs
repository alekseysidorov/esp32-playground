use std::time::Duration;

use esp_idf_svc::log::EspLogger;
use esp_idf_sys::*;
use log::info;
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite, RGB8,
};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    EspLogger::initialize_default();

    let strip_len = 3;
    let led_pin = 26;
    let mut ws2812 = Ws2812Esp32Rmt::new(0, led_pin)?;

    info!("Start NeoPixel rainbow!");

    let mut hue = unsafe { esp_random() } as u8;
    loop {
        // let pixels = std::iter::repeat(hsv2rgb(Hsv {
        //     hue,
        //     sat: 255,
        //     val: 8,
        // }))
        // .take(strip_len);
        let pixels = [
            RGB8 { r: 64, g: 0, b: 0 },
            RGB8 { r: 0, g: 64, b: 0 },
            RGB8 { r: 0, g: 0, b: 64 },
        ];

        ws2812.write(pixels.into_iter())?;

        std::thread::sleep(Duration::from_millis(1_000));
        hue = hue.wrapping_add(10);

        info!("Next line");
    }
}
