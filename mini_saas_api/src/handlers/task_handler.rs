use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::query_as;

use crate::{errors::AppError, models::task::Task, AppState};

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub completed: Option<String>,
}

pub async fn get_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = query_as("SELECT * FROM tasks")
        .fetch_all(&state.db)
        .await
        .map_err(|_| AppError::DatabaseError)?;

    Ok(Json(tasks))
}

pub async fn get_task_by_id(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Task>, AppError> {
    let task = query_as("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| AppError::DatabaseError)?;

    task.map(Json).ok_or(AppError::NotFound)
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError> {
    let task = query_as(
        "INSERT INTO tasks (title,completed) VALUES(?,FALSE);
        SELECT * FROM tasks WHERE id= LAST_INSERT_ID();",
    )
    .bind(payload.title)
    .fetch_one(&state.db)
    .await
    .map_err(|_| AppError::DatabaseError)?;

    Ok(Json(task))
}

pub async fn update_task(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    sqlx::query(
        "UPDATE tasks SET
         title = COALESCE(?, title),
         completed = COALESCE(?, completed)
         WHERE id = ?",
    )
    .bind(payload.title)
    .bind(payload.completed)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::DatabaseError)?;

    get_task_by_id(Path(id), State(state)).await
}

pub async fn delete_task(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<&'static str>, AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json("Deleted successfully"))
}
