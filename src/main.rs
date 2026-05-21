use anyhow::{Context, Result};
use axum::Server;
use ems::config::Config;
use ems::db;
use ems::routes::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env();
    // Create connection pool
    let pool = db::create_pool(&config.database_url).await?;
    // Run migrations (creates tables if not present)
    db::run_migrations(&pool).await?;

    let socket = format!("{}:{}", config.ip_address, config.port)
        .parse()
        .expect("Failed to open socket");

    println!("server running on {:?}", &socket);
    Server::bind(&socket)
        .serve(app(pool).into_make_service_with_connect_info::<SocketAddr>())
        .await
        .context("Failed to start server.")
}
