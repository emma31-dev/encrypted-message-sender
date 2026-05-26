use crate::config::Config;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub struct AuthenticatedUser(pub String);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Get Authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing authorization header".to_string(),
                )
            })?
            .to_str()
            .map_err(|_| {
                (StatusCode::UNAUTHORIZED, "Invalid header".to_string())
            })?;

        // 2. Expect "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid auth scheme".to_string()))?;

        // 3. Decode JWT using your secret (from Config)
        let config = Config::from_env(); // or get from state (better)
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<serde_json::Value>(&token, &decoding_key, &validation)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        // 4. Extract user_id (the `sub` claim you stored in login/signup)
        let user_id = token_data
            .claims
            .get("sub")
            .map(|v| v.to_string())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing user_id".to_string()))?;

        Ok(AuthenticatedUser(user_id))
    }
}

// // 1. Import the necessary traits
// use axum::{extract::FromRequestParts, http::{request::Parts, StatusCode}};
// use jsonwebtoken::{decode, DecodingKey, Validation};
// use uuid::Uuid;

// // 2. Define the extractor struct
// pub struct AuthenticatedUser(pub Uuid);

// // 3. Implement the FromRequestParts trait (no macro needed!)
// impl<S> FromRequestParts<S> for AuthenticatedUser
// where
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, &'static str);

//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         // 1. Extract the JWT from the Authorization header
//         let auth_header = parts
//             .headers
//             .get("authorization")
//             .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header"))?
//             .to_str()
//             .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid authorization header"))?;

//         // 2. Validate the bearer token format
//         let token = auth_header
//             .strip_prefix("Bearer ")
//             .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization scheme"))?;

//         // 3. Decode and validate the JWT
//         let decoding_key = DecodingKey::from_secret(b"your-secret-key"); // ⚠️ Store this properly
//         let validation = Validation::default();
//         let token_data = decode::<serde_json::Value>(token, &decoding_key, &validation)
//             .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

//         // 4. Extract the user_id from the `sub` claim
//         let user_id = token_data
//             .claims
//             .get("sub")
//             .and_then(|v| v.as_str())
//             .and_then(|id| Uuid::parse_str(id).ok())
//             .ok_or((StatusCode::UNAUTHORIZED, "Invalid token claims"))?;

//         Ok(AuthenticatedUser(user_id))
//     }
// }
