#![warn(clippy::pedantic)]

use std::env;

use actix_web::{middleware::Logger, web, App, HttpServer};
use awc::Client;
use middleware::handle_client_cert;
use once_cell::sync::Lazy;
use tracing::info;

use crate::tls::create_tls_server_config;

mod env_util;
mod middleware;
mod routes;
mod tls;

const PORT: u16 = 8080;

// env declarations
static BASE_DOMAIN: Lazy<String> = env_util::lazy_env!("BASE_DOMAIN", "local.host:8443");
static REGISTRY_URL: Lazy<String> = env_util::lazy_env!("REGISTRY_URL", "registry");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();
    tls::initialise_key_pem();

    info!("Launching SCP proxy version {}", env!("CARGO_PKG_VERSION"));

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Client::default()))
            .wrap(middleware::CheckCertificate)
            .wrap(Logger::new("%a %{Host}i %r %s %t (%T)"))
            .default_service(web::route().to(routes::route_whoami))
    })
    .on_connect(handle_client_cert)
    .bind(("0.0.0.0", PORT))?
    .bind_rustls(("0.0.0.0", 8443), create_tls_server_config()?)?
    .run()
    .await
}
