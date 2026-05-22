use anyhow::{Context, Result};
use ems::config::Config;
use ems::db;
use ems::routes::app;
use ems::tracing::init_tracing;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Tracing initiallization
    let _guard = init_tracing();

    let config = Config::from_env();
    let pool = db::create_pool(&config.database_url)
        .await
        .context("Failed to create pool")?;

    // Run migrations
    db::run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;

    let socket = format!("{}:{}", config.ip_address, config.port);

    info!("server running on {:?}", &socket);
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    axum::serve(listener, app(pool))
        .await
        .context("Failed to start server.")
}
