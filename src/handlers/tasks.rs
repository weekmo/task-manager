use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
    models::task::{CreateTaskRequest, Task, UpdateTaskRequest},
};

pub async fn get_tasks(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(tasks))
}

pub async fn create_task(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    Json(body): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (user_id, title, description) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(user_id)
    .bind(&body.title)
    .bind(&body.description)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    Path(task_id): Path<Uuid>,
    Json(body): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            done = COALESCE($3, done)
         WHERE id = $4 AND user_id = $5
         RETURNING *",
    )
    .bind(&body.title)
    .bind(&body.description)
    .bind(body.done)
    .bind(task_id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Task not found".into()))?;

    Ok(Json(task))
}

pub async fn delete_task(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1 AND user_id = $2")
        .bind(task_id)
        .bind(user_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Task not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}