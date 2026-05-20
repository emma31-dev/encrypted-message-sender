use axum::extract::Path;

use axum::{Router, http::StatusCode, routing::get};

// our router
pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(health_check))
        .route("/test/:id", get(get_user))
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "ok")
}

async fn get_user(Path(id): Path<String>) -> String {
    format!("User ID: {id}")
}