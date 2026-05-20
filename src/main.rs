use axum::Server;
use ems::routes::app;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let address = "0.0.0.0:3000".parse().expect("Failed to open socket");

    println!("server running on {:?}", &address);
    Server::bind(&address)
        .serve(app().into_make_service())
        .await
        .context("Failed to start server.")
}
