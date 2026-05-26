use anyhow::{Context, Result};
use ems::config::Config;
use ems::db;
use ems::routes::app;
use ems::tracing::init_tracing;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Tracing initiallization
    let _guard = init_tracing();
    let config = Config::from_env();

    let pool = db::create_pool(&config.database_url)
        .await
        .context("Failed to create pool")?;
    db::run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;

    let socket = format!("{}:{}", config.ip_address, config.port);

    info!("server running on {:?}", socket);
    let app = app(pool).into_make_service_with_connect_info::<SocketAddr>();
    let listener = TcpListener::bind(socket)
        .await
        .context("Failed to create new TCP listener")?;
    axum::serve(listener, app)
        .await
        .context("Failed to start server.")
}
