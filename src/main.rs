use esp_idf_hal::{
    self,
    gpio::{Output, PinDriver},
    io::Write,
    peripherals::Peripherals,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, http::server::EspHttpServer, nvs::EspDefaultNvsPartition,
    wifi::*,
};
use heapless::String;
use std::{
    net::Ipv4Addr,
    str::FromStr,
    thread::{self, sleep},
    time::Duration,
};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Started Testing!!");

    let peripherals = Peripherals::take().unwrap();

    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    blink_led_ms(&mut led, 300, 3)?;

    led.set_high()?;

    log::info!("Wifi");
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?;
    blink_led_ms(&mut led, 100, 3)?;

    /* let wifi_config = Configuration::Client(ClientConfiguration {
        ssid: String::try_from("Ap_287").unwrap(),
        password: String::try_from("@1CristoVive").unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }); */
    /* let wifi_config = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: String::try_from("ESP32-NICE-WIFI").unwrap(),
        password: String::try_from("123456789").unwrap(),
        channel: 0,
        auth_method: AuthMethod::WPA2Personal,
        max_connections: 4,
        ..Default::default()
    }); */

    // --------------------- WIFI CONFIG (AP + STA) -------------------------
    let wifi_config = Configuration::Mixed(
        ClientConfiguration {
            ssid: String::try_from("Ap_287").unwrap(),
            password: String::try_from("@1CristoVive").unwrap(),
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: String::try_from("ESP32-NET").unwrap(),
            password: String::try_from("12345678a").unwrap(),
            channel: 0, // IMPORTANT: auto channel
            auth_method: AuthMethod::WPA2Personal,
            max_connections: 4,
            ..Default::default()
        },
    );

    log::info!("Setting Wifi Configuration");
    wifi.set_configuration(&wifi_config)?;
    blink_led_ms(&mut led, 100, 5)?;

    log::info!("Starting Wifi");
    wifi.start()?;
    blink_led_ms(&mut led, 100, 5)?;

    log::info!("Connecting Wifi");
    wifi.connect()?;

    // ----------- WAIT FOR CONNECTION -----------
    loop {
        led.set_low()?;

        if wifi.is_connected()? && wifi.is_up()? {
            let ip_info = wifi.sta_netif().get_ip_info()?;

            if ip_info.ip != Ipv4Addr::new(0, 0, 0, 0) {
                log::info!("Wifi CONNECTED");
                break;
            }
            break;
        }

        blink_led_ms(&mut led, 100, 1)?;
        thread::sleep(Duration::from_millis(400));
    }

    let sta_netif = wifi.sta_netif();
    let ap_netif = wifi.ap_netif();

    let sta_ip_info = sta_netif.get_ip_info()?;
    log::info!("STA IP: {:?}", sta_ip_info);

    let ap_ip_info = ap_netif.get_ip_info()?;
    log::info!("AP IP: {:?}", ap_ip_info);

    // ----------------- WEB SERVER ---------------------
    let mut server = EspHttpServer::new(&Default::default())?;

    server.fn_handler("/", embedded_svc::http::Method::Get, |req| {
        let mut resp = req.into_ok_response()?;

        resp.write_all(b"<h1>Hey Brother</h1>").unwrap();

        Ok::<(), anyhow::Error>(())
    })?;

    log::info!("Server Running");

    loop {
        blink_led_ms(&mut led, 100, 5)?;

        led.set_high()?;
        sleep(Duration::from_millis(3000));
    }
}

fn blink_led_ms(led: &mut PinDriver<Output>, milliseconds: u64, times: u8) -> anyhow::Result<()> {
    for __i in 0..times {
        led.set_high()?;
        sleep(Duration::from_millis(milliseconds));
        led.set_low()?;
        sleep(Duration::from_millis(milliseconds));
    }

    return Ok(());
}
