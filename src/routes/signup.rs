use super::helper::*;
use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{debug, error};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
}

pub async fn signup(
    State(pool): State<SqlitePool>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    debug!("Signup attempt detected");

    if payload.username.is_empty() || payload.password.is_empty() {
        debug!("Empty payload received for signup");
        return Err((StatusCode::BAD_REQUEST, "Missing fields".to_string()));
    }

    let hashed = hash(payload.password, DEFAULT_COST).expect("Failed to hash password");
    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        "INSERT INTO users (id, username, password_hash, date_joined) VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(&user_id)
    .bind(&payload.username)
    .bind(&hashed)
    .bind(&now)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            debug!(user_id = user_id, "Created user successfully");
            // 5. Generate JWT
            let token = create_jwt(&user_id)?; // implement this helper
            Ok(Json(AuthResponse { token, user_id }))
        }
        Err(sqlx::Error::Database(ref e)) if e.is_unique_violation() => {
            debug!(
                username = payload.username,
                "Creating user failed. Username already exist"
            );
            Err((StatusCode::CONFLICT, "Username already taken".to_string()))
        }
        Err(_) => {
            error!("Creating user failed. Failed to load database.");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))
        }
    }
}
