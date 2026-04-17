pub mod logical;
pub mod physical;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use esp_idf_hal::{self, gpio::PinDriver, io::Write, peripherals::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

use crate::{logical::http::server::HttpServer, physical::wireless::wifi::Wifi};
use std::thread;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Started Testing!!");

    let peripherals = Peripherals::take().unwrap();

    let led_mutex = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio2)?));

    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // --------------------- WIFI CONFIG (AP + STA) -------------------------
    let main_led = led_mutex.clone();
    let mut led = main_led.lock().unwrap();
    led.set_high()?;
    let wifi = Wifi::init_wifi(peripherals.modem, sysloop, nvs)?;
    Wifi::wait_connected(&wifi)?;

    let sta_netif = wifi.sta_netif();
    let ap_netif = wifi.ap_netif();

    let sta_ip_info = sta_netif.get_ip_info()?;
    log::info!("STA IP: {:?}", sta_ip_info);

    let ap_ip_info = ap_netif.get_ip_info()?;
    log::info!("AP IP: {:?}", ap_ip_info);
    led.set_low()?;
    drop(led);

    // ----------------- WEB SERVER ---------------------
    let mut http_server = HttpServer::new()?;
    let http_led = led_mutex.clone();

    http_server
        .server
        .fn_handler("/toggle", esp_idf_svc::http::Method::Get, move |req| {
            let mut led = http_led.lock().unwrap();
            if led.is_set_high() {
                led.set_low()?;
                req.into_ok_response()?.write_all(b"<h1>LED Off</h1>")
            } else {
                led.set_high()?;
                req.into_ok_response()?.write_all(b"<h1>LED On</h1>")
            }
        })?;

    loop {
        thread::sleep(Duration::from_millis(2_000));
    }
}
