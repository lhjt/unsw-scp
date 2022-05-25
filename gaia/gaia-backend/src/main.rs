use std::env;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use once_cell::sync::Lazy;

mod env_util;
mod routes;
mod utils;

static JWT_PEM: Lazy<String> = once_cell::sync::Lazy::new(|| match env::var("JWT_PEM_LOC") {
    Ok(v) => std::fs::read_to_string(v).unwrap_or_else(|_| panic!("JET PEM missing")),
    Err(_) => std::fs::read_to_string("../../proxy/certs/jwt-key.pem")
        .unwrap_or_else(|_| panic!("JET PEM missing")),
});

static DB_URI: Lazy<String> = env_util::lazy_env!("DB_URI", "sqlite://./db.db");
static PUBLIC_ADDR: Lazy<String> = env_util::lazy_env!("PUBLIC_ADDR", "login.local.host:8443");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    let connection = sea_orm::Database::connect(DB_URI.as_str()).await?;
    Migrator::up(&connection, None).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(connection.clone()))
            .wrap(Logger::new("%a %{Host}i %r %s %t (%T)"))
            .service(
                web::scope("/api")
                    .service(routes::set_user_roles)
                    .service(routes::get_user_roles)
                    .service(routes::get_users)
                    .service(routes::self_service::get_roles)
                    .service(routes::self_service::get_id),
            )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
    .context("failed to run and bind the server")
}
