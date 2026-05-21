use super::helper::*;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SignupRequest {
    username: String,
    password: String,
    // optional public_key for future E2EE
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String, // JWT
    user_id: String,
}

pub async fn signup(
    State(pool): State<SqlitePool>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // 1. Validate input (username not empty, password strength, etc.)
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing fields".to_string()));
    }

    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    // 2. Hash password
    let hashed = cipher
        .encrypt(&nonce, payload.password.as_ref())
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Hash error".to_string()))?;

    // 3. Generate user ID
    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // 4. Insert into database
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
            // 5. Generate JWT
            let token = create_jwt(&user_id)?; // implement this helper
            Ok(Json(AuthResponse { token, user_id }))
        }
        Err(sqlx::Error::Database(ref e)) if e.is_unique_violation() => {
            Err((StatusCode::CONFLICT, "Username already taken".to_string()))
        }
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string())),
    }
}
