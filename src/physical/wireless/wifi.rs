use std::{net::Ipv4Addr, time::Duration};

use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{AccessPointConfiguration, ClientConfiguration, Configuration, EspWifi},
};
use heapless::String;
use std::thread;

pub struct Wifi {}

impl Wifi {
    pub fn init_wifi(
        modem: Modem,
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> anyhow::Result<EspWifi> {
        log::info!("Setting Wifi Configuration");
        let wifi_configuration = Configuration::Mixed(
            ClientConfiguration {
                ssid: String::try_from("Ap_287").unwrap(),
                password: String::try_from("@1CristoVive").unwrap(),
                auth_method: esp_idf_svc::wifi::AuthMethod::WPA2Personal,
                ..Default::default()
            },
            AccessPointConfiguration {
                ssid: String::try_from("ESP-WIFI").unwrap(),
                password: String::try_from("12345678").unwrap(),
                channel: 0,
                auth_method: esp_idf_svc::wifi::AuthMethod::WPA2Personal,
                max_connections: 4,
                ..Default::default()
            },
        );

        let mut wifi = EspWifi::new(modem, sysloop, Some(nvs))?;
        wifi.set_configuration(&wifi_configuration)?;

        log::info!("Starting Wifi modem");
        wifi.start()?;

        log::info!("Connecting Wifi");
        wifi.connect()?;

        return Ok(wifi);
    }

    pub fn wait_connected(wifi: &EspWifi) -> anyhow::Result<()> {
        loop {
            if wifi.is_connected()? && wifi.is_up()? {
                let ip_info = wifi.sta_netif().get_ip_info()?;

                if ip_info.ip != Ipv4Addr::new(0, 0, 0, 0) {
                    log::info!("Wifi CONNECTED");
                    break;
                }
                break;
            }

            thread::sleep(Duration::from_millis(400));
        }

        Ok(())
    }
}
