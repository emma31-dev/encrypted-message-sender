mod helper;
mod signup;

use super::routes::signup::signup;
use axum::{http::StatusCode, routing::get, Router};
use sqlx::SqlitePool;

// our router
pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/healthz", get(health_check))
        .route("/signup", get(signup))
        .with_state(pool)
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "ok")
}
