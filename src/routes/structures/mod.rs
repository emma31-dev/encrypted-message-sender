use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,  // expiration
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub other_user_id: String,
}

#[derive(Serialize)]
pub struct CreateChatResponse {
    pub id: String,
    pub participants: Vec<String>,
    pub created_at: String,
}
