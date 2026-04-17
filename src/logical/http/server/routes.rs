use esp_idf_hal::io::Write;
use esp_idf_svc::http::server::EspHttpServer;

pub fn setup_routes(server: &mut EspHttpServer) -> anyhow::Result<()> {
    server.fn_handler("/", esp_idf_svc::http::Method::Get, |req| {
        let mut res = req.into_ok_response()?;
        res.write_all(b"<h1>ESP-32 Working !</h1")
    })?;

    Ok(())
}
