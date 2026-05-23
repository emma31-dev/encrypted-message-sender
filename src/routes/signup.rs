use super::helper::create_jwt;
use super::structures::{AuthResponse, SignupRequest};
use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use sqlx::SqlitePool;
use tracing::{debug, error};
use uuid::Uuid;

pub async fn signup(
    State(pool): State<SqlitePool>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    debug!("Signup attempt detected");

    if payload.username.is_empty() || payload.password.is_empty() {
        debug!("Empty payload received for signup");
        return Err((StatusCode::BAD_REQUEST, "Missing fields".to_string()));
    }

    let hashed = hash(payload.password, DEFAULT_COST)
        .map_err(|e| error!(?e))
        .expect("Failed to hash user password");
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
            debug!(user_id, "Created user successfully");

            let token = create_jwt(&user_id)?;
            Ok(Json(AuthResponse { token, user_id }))
        }
        Err(sqlx::Error::Database(ref e)) if e.is_unique_violation() => {
            debug!(
                payload.username,
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

#[cfg(test)]
mod test {
    use crate::routes::structures::{SignupRequest};
    use crate::db;
    use crate::config::Config;
    use sqlx::SqlitePool;
    use anyhow::{Context,Result};
    use axum::{Json, extract::State};
    use super::signup;

    #[tokio::test]
    #[ignore = "only runs locally (use `cargo test -- --ignored` to run this test)"]
    async fn test_signup() -> Result<()> {
        let payload = SignupRequest { 
            username: "test_user_1".to_string(), 
            password: "secret123".to_string()
        };
        let config = Config::from_env();
        let pool = db::create_pool(&config.database_url)
            .await
            .context("Failed to create pool")?;
        db::run_migrations(&pool)
            .await
            .context("Failed to run migrations")?;

        let response = signup(State(pool), Json(payload)).await;
        assert!(!response.is_ok());
        Ok(())
    }
    
    #[tokio::test]
    async fn test_signup_of_existing_account() -> Result<()> {
        let payload = SignupRequest { 
            username: "test_user_1".to_string(), 
            password: "secret123".to_string()
        };
        
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();

        let response = signup(State(pool), Json(payload)).await;
        assert!(response.is_ok());
        Ok(())
    }
}
