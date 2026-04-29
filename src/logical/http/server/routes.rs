use esp_idf_hal::io::Write;
use esp_idf_svc::http::server::EspHttpServer;

pub fn setup_routes(server: &mut EspHttpServer) -> anyhow::Result<()> {
    server.fn_handler("/", esp_idf_svc::http::Method::Get, |req| {
        let mut res = req.into_ok_response()?;
        res.write_all(b"<h1>ESP-32 Working !</h1")
    })?;

    // curl -X POST http://<ESP_IP>/save_data -d "hello esp32"
    server.fn_handler("/save_data", esp_idf_svc::http::Method::Post, |mut req| {
        // The limit for each request is payloads with 512 bytes
        let mut buf = [0u8; 512];

        let len = req.read(&mut buf)?;

        let received = std::str::from_utf8(&buf).unwrap();

        log::info!("Received ({}): {}", len, received);

        let mut resp = req.into_ok_response()?;

        resp.write_all(b"Ok.")
    })?;

    Ok(())
}
