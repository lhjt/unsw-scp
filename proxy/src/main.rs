#![warn(clippy::pedantic)]

use std::env;

use actix_web::{web, App, HttpServer};
use middleware::handle_client_cert;
use tracing::info;

mod middleware;
mod routes;
mod tls;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    info!("Launching server on port {}", PORT);

    HttpServer::new(|| App::new().default_service(web::route().to(routes::route_whoami)))
        .on_connect(handle_client_cert)
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}
