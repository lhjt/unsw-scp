#![warn(clippy::pedantic)]
#![allow(clippy::unused_async)]

use std::env;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use migration::{Migrator, MigratorTrait};
use once_cell::sync::Lazy;

mod gaia_utils;
mod handler_utils;
mod registry;
mod routes;

static JWT_PEM: Lazy<String> = once_cell::sync::Lazy::new(|| match env::var("JWT_PEM_LOC") {
    Ok(v) => std::fs::read_to_string(v).unwrap_or_else(|_| panic!("JWT PEM missing")),
    Err(_) => {
        std::fs::read_to_string("/certs/jwt-key.pem").unwrap_or_else(|_| panic!("JWT PEM missing"))
    },
});

/// HMAC key used for authenticating flags
#[allow(clippy::match_wild_err_arm)]
static HMAC_KEY: Lazy<String> = once_cell::sync::Lazy::new(|| match env::var("HMAC_KEY") {
    Ok(v) => v,
    Err(_) => {
        panic!("HMAC_KEY env var missing")
    },
});

static DB_URI: Lazy<String> = env_utils::lazy_env!("DB_URI", "sqlite://./db.db");
static GAIA_ADDR: Lazy<String> = env_utils::lazy_env!("GAIA_ADDR", "gaia-backend:8081");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    let connection = sea_orm::Database::connect(DB_URI.as_str()).await?;
    Migrator::up(&connection, None).await?;

    Ok(HttpServer::new(move || {
        App::new().app_data(Data::new(connection.clone())).service(
            web::scope("/api")
                .service(routes::evaluation::evaluate)
                .service(routes::create_service::create_service)
                .service(web::scope("/flags").service(routes::flags::generate_flag))
                .service(web::scope("/challenges").service(routes::challenges::get_all)),
        )
    })
    .bind(("0.0.0.0", 8082))?
    .run()
    .await?)
}
