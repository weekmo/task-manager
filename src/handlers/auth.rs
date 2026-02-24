use axum::{extract::State, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;

use crate::{
    errors::AppError,
    middleware::auth::create_jwt,
    models::user::{AuthResponse, LoginRequest, RegisterRequest, User},
};

pub async fn register(
    State(pool): State<PgPool>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let password_hash = hash(&body.password, DEFAULT_COST)
        .map_err(|_| AppError::BadRequest("Failed to hash password".into()))?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *",
    )
    .bind(&body.email)
    .bind(&password_hash)
    .fetch_one(&pool)
    .await?;

    let token = create_jwt(&user.id.to_string())?;
    Ok(Json(AuthResponse { token }))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::Auth("Invalid email or password".into()))?;

    let valid = verify(&body.password, &user.password_hash)
        .map_err(|_| AppError::Auth("Invalid email or password".into()))?;

    if !valid {
        return Err(AppError::Auth("Invalid email or password".into()));
    }

    let token = create_jwt(&user.id.to_string())?;
    Ok(Json(AuthResponse { token }))
}
