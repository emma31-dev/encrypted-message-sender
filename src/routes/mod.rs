mod helper;
mod login;
mod signup;
mod structures;

use super::routes::login::login;
use super::routes::signup::signup;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{routing::get, Router};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::{Any, CorsLayer};

// our router
pub fn app(pool: SqlitePool) -> Router {
    let governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(5)
            .finish()
            .unwrap(),
    );
    let rate_limiter = GovernorLayer::new(governor_config);
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/healthz", get(health_check))
        .route("/is_ready", get(readiness_check))
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .with_state(pool)
        .layer(cors)
        .layer(rate_limiter)
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
