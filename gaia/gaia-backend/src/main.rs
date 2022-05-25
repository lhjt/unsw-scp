use std::env;

use actix_web::{web::Data, App, HttpServer};
use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use once_cell::sync::Lazy;
use routes::get_roles;

mod routes;
mod utils;

static JWT_PEM: Lazy<String> = once_cell::sync::Lazy::new(|| match env::var("JWT_PEM_LOC") {
    Ok(v) => std::fs::read_to_string(v).unwrap_or_else(|_| panic!("JET PEM missing")),
    Err(_) => std::fs::read_to_string("../../proxy/certs/jwt-key.pem")
        .unwrap_or_else(|_| panic!("JET PEM missing")),
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    let connection = sea_orm::Database::connect("sqlite://./db.db").await?;
    Migrator::up(&connection, None).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(connection.clone()))
            .service(get_roles)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
    .context("failed to run and bind the server")
}
