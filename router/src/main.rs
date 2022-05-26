use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use migration::{Migrator, MigratorTrait};
use once_cell::sync::Lazy;
use std::env;

mod gaia_utils;
mod registry;
mod routes;

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
                .service(routes::evaluate)
                .service(routes::create_service::create_service),
        )
    })
    .bind(("0.0.0.0", 8082))?
    .run()
    .await?)
}