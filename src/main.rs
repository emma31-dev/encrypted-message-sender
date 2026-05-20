use anyhow::{Context, Result};
use axum::Server;
use ems::config::Config;
use ems::db;
use ems::routes::app;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env();
    // Create connection pool
    let pool = db::create_pool(&config.database_url).await?;
    // Run migrations (creates tables if not present)
    db::run_migrations(&pool).await?;

    let address = format!("0.0.0.0:{}", config.port)
        .parse()
        .expect("Failed to open socket");

    println!("server running on {:?}", &address);
    Server::bind(&address)
        .serve(app(pool).into_make_service())
        .await
        .context("Failed to start server.")
}
