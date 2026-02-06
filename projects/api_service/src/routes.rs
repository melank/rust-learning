use crate::error::AppError;
use crate::models::{CreateNoteRequest, Note};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub fn build_router(pool: SqlitePool) -> Router {
    let state = AppState { pool };

    Router::new()
        .route("/health", get(health))
        .route("/notes", get(list_notes).post(create_note))
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn list_notes(State(state): State<AppState>) -> Result<Json<Vec<Note>>, AppError> {
    let notes =
        sqlx::query_as::<_, Note>("SELECT id, title, body, created_at FROM notes ORDER BY id DESC")
            .fetch_all(&state.pool)
            .await?;

    Ok(Json(notes))
}

async fn create_note(
    State(state): State<AppState>,
    Json(payload): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    if !payload.is_valid() {
        return Err(AppError::BadRequest("title/body must not be empty"));
    }

    let result = sqlx::query("INSERT INTO notes (title, body) VALUES (?1, ?2)")
        .bind(&payload.title)
        .bind(&payload.body)
        .execute(&state.pool)
        .await?;

    let inserted_id = result.last_insert_rowid();

    let note = sqlx::query_as::<_, Note>(
        "SELECT id, title, body, created_at FROM notes WHERE id = ?1 LIMIT 1",
    )
    .bind(inserted_id)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(note)))
}
