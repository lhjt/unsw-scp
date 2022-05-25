use actix_web::{web::Data, App, HttpServer};
use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use routes::get_roles;

mod routes;

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
