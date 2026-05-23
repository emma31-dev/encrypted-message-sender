use super::helper::create_jwt;
use super::structures::{AuthResponse, LoginRequest};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::verify;
use sqlx::SqlitePool;
use tracing::{debug, error, info, warn};

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    debug!("Login attempt detected");

    if payload.username.is_empty() || payload.password.is_empty() {
        debug!("Empty payload received for login");
        return Err((StatusCode::BAD_REQUEST, "Missing fields".to_string()));
    }
    let row = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = ? AND deleted_at IS NULL",
        payload.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!(
            error = ?e,
            "DB failed to initialize"
        );
        (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string())
    })?;

    let (id, stored_hash) = match row {
        Some(r) => (r.id, r.password_hash),
        None => {
            warn!(
                username = payload.username,
                "Failed to find record for login attempt of user"
            );
            return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
        }
    };

    let user_id = id.unwrap_or_default();
    debug!(user_id = user_id, "Record found for login of User");

    // 3. Verify password
    let is_valid = verify(&payload.password, &stored_hash).map_err(|_| {
        warn!(
            user_id = user_id,
            "Failed to verify password for login attempt of user"
        );
        (StatusCode::INTERNAL_SERVER_ERROR, "Hash error".to_string())
    })?;

    if !is_valid {
        warn!(
            user_id = user_id,
            "Invalid credentials provided for login attempt of user"
        );
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // 4. Generate and return JWT
    let token = create_jwt(&user_id)?;
    info!(
        user_id = user_id,
        "Authentication response returned to user"
    );
    Ok(Json(AuthResponse { token, user_id }))
}

#[cfg(test)]
mod test {
    use crate::routes::structures::LoginRequest;
    use crate::db;
    use crate::config::Config;
    use anyhow::{Context, Result};
    use axum::{Json, extract::State};
    use super::login;

    #[tokio::test]
    async fn test_login() -> Result<()> {
        let payload = LoginRequest { 
            username: "test_user_1".to_string(), 
            password: "secret123".to_string()
        };
        let config = Config::from_env();
        let pool = db::create_pool(&config.database_url)
            .await
            .context("Failed to create pool")?;

        let response = login(State(pool), Json(payload)).await;
        assert!(response.is_ok());
        Ok(())
    }
}