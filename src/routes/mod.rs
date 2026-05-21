mod helper;
mod signup;
mod login;

use super::routes::login::login;
use super::routes::signup::signup;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{routing::get, Router};
use sqlx::SqlitePool;

// our router
pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/healthz", get(health_check))
        .route("/is_ready", get(readiness_check))
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .with_state(pool)
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Ok")
}

async fn readiness_check(State(pool): State<SqlitePool>) -> (StatusCode, &'static str) {
    // Try to execute a simple query (e.g., SELECT 1)
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => (StatusCode::OK, "ready"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "db unavailable"),
    }
}
