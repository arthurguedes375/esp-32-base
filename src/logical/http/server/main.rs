use esp_idf_svc::http::server::EspHttpServer;

use crate::logical::http::server::routes::setup_routes;

pub struct HttpServer<'a> {
    pub server: EspHttpServer<'a>,
}

impl<'a> HttpServer<'a> {
    pub fn new() -> anyhow::Result<Self> {
        log::info!("Starting HTTP Server");
        let mut server = EspHttpServer::new(&Default::default())?;

        setup_routes(&mut server)?;

        log::info!("HTTP Server Running");

        return Ok(Self { server });
    }
}
