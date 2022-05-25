use migration::{Migrator, MigratorTrait};

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    let connection = sea_orm::Database::connect("sqlite://./db.db").await?;
    Migrator::up(&connection, None).await?;

    Ok(())
}
