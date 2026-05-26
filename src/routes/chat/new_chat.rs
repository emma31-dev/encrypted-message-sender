use crate::extractors::AuthenticatedUser;
use crate::routes::structures::{CreateChatRequest, CreateChatResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use sqlx;
use sqlx::SqlitePool;
use tracing::error;
use uuid::Uuid;

pub async fn new_chat(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateChatRequest>,
) -> Result<Json<CreateChatResponse>, (StatusCode, String)> {
    if payload.other_user_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Empty payload".to_string()));
    }

    if user_id == payload.other_user_id {
        return Err((
            StatusCode::BAD_REQUEST,
            "Cannot create chat with yourself".to_string(),
        ));
    }

    // 2. Check that other_user exists (not deleted)
    let exists = sqlx::query("SELECT 1 FROM users WHERE id = ? AND deleted_at IS NULL")
        .bind(&payload.other_user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|error| {
            error!(?error, "Db error");
            (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string())
        })?;
    if exists.is_none() {
        return Err((StatusCode::NOT_FOUND, "Other user not found".to_string()));
    }

    // // 3. Start transaction (to ensure consistency)
    // let mut tx = pool.await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

    // 4. Insert into chats
    let chat_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query("INSERT INTO chats (id, created_at) VALUES (?, ?)")
        .bind(chat_id)
        .bind(now)
        .execute(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

    // 5. Insert participants
    for participant in [&user_id, &payload.other_user_id] {
        let _result =
            sqlx::query("INSERT INTO chat_participants (chat_id, user_id) VALUES (?1, ?2)")
                .bind(&chat_id)
                .bind(&participant)
                .execute(&pool)
                .await;
    }

    // tx.commit().await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?;

    Ok(Json(CreateChatResponse {
        id: chat_id.to_string(),
        participants: vec![user_id.to_string(), payload.other_user_id.to_string()],
        created_at: now.to_rfc2822(),
    }))
}
