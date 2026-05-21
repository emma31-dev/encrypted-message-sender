use super::signup::{AuthResponse};
use axum::extract::State;
use axum::Json;
use axum::http::StatusCode;
use sqlx::SqlitePool;
use serde::Deserialize;
use tracing::{info, warn, error};
use bcrypt::verify;
use super::helper::create_jwt;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    info!("Login attempt detected");

    if payload.username.is_empty() || payload.password.is_empty() {
        warn!("Empty payload received for login");
        return Err((StatusCode::BAD_REQUEST, "Missing fields".to_string()));
    }
    let row = sqlx::query!(
            "SELECT id, password_hash FROM users WHERE username = ? AND deleted_at IS NULL",
            payload.username
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error!("DB error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string())
        })?;
    
    let (id, stored_hash) = match row {
        Some(r) => (r.id, r.password_hash),
        None => {
            warn!("Failed to find credentials for login attempt of user");
            return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
        },
    };
    
    let user_id = id.unwrap_or_default();
    info!("Record found for login of User: {}", &user_id);
    
    // 3. Verify password
    let is_valid = verify(&payload.password, &stored_hash)
        .map_err(|_| {
            warn!("Failed to verify password for login attempt of user: {}", &user_id);
            (StatusCode::INTERNAL_SERVER_ERROR, "Hash error".to_string())
        })?;

    if !is_valid {
        warn!("Invalid credentials provided for login attempt of user: {}", &user_id);
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // 4. Generate and return JWT
    let token = create_jwt(&user_id)?;
    info!("Authentication response returned to user: {}", &user_id);
    Ok(Json(AuthResponse { token, user_id }))
}