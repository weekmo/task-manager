use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthUser(pub Uuid);

pub fn create_jwt(user_id: &str) -> Result<String, AppError> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::Auth("Failed to create token".into()))
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let headers: &HeaderMap = &parts.headers;

        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Auth("Missing or invalid Authorization header".into()))?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Auth("Invalid or expired token".into()))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AppError::Auth("Invalid user ID in token".into()))?;

        Ok(AuthUser(user_id))
    }
}
