use embedded_svc::wifi::{AccessPointConfiguration, AuthMethod, Configuration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi::EspWifi};
use log::info;

pub struct Wifi<'a> {
    inner: EspWifi<'a>,
}

impl<'a> Wifi<'a> {
    pub fn new(modem: Modem, sysloop: EspSystemEventLoop) -> anyhow::Result<Self> {
        let inner = EspWifi::new(modem, sysloop, None)?;

        info!("Created esp-idf Wifi stack");

        Ok(Self { inner })
    }

    pub fn establish_softap(&mut self) -> anyhow::Result<()> {
        let conf = Configuration::AccessPoint(AccessPointConfiguration {
            ssid: heapless::String::from("esp32"),
            password: heapless::String::from("12345678"),
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        });
        self.inner.set_configuration(&conf)?;
        self.inner.start()?;

        info!("SoftAP started");

        Ok(())
    }
}
