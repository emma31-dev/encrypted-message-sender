use super::structures::Claims;
use crate::config::Config;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

pub fn create_jwt(user_id: &str) -> Result<String, (StatusCode, String)> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };
    let config = Config::from_env();
    let secret = config.jwt_secret;
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Token creation failed".to_string(),
        )
    })
}
