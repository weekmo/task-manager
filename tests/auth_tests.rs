// Unit tests for JWT authentication
use jsonwebtoken::{decode, DecodingKey, Validation};
use task_manager::middleware::auth::{create_jwt, Claims};

#[test]
fn test_create_jwt_success() {
    std::env::set_var("JWT_SECRET", "test_secret_key");
    let user_id = "550e8400-e29b-41d4-a716-446655440000";

    let result = create_jwt(user_id);
    assert!(result.is_ok());

    let token = result.unwrap();
    assert!(!token.is_empty());
}

#[test]
fn test_jwt_contains_valid_claims() {
    std::env::set_var("JWT_SECRET", "test_secret_key");
    let user_id = "550e8400-e29b-41d4-a716-446655440000";

    let token = create_jwt(user_id).unwrap();

    // Decode and verify the token
    let secret = std::env::var("JWT_SECRET").unwrap();
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    assert!(token_data.is_ok());
    let claims = token_data.unwrap().claims;
    assert_eq!(claims.sub, user_id);
    assert!(claims.exp > chrono::Utc::now().timestamp() as usize);
}

#[test]
fn test_invalid_jwt_fails() {
    std::env::set_var("JWT_SECRET", "test_secret_key");
    let invalid_token = "invalid.token.here";

    let secret = std::env::var("JWT_SECRET").unwrap();
    let result = decode::<Claims>(
        invalid_token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    assert!(result.is_err());
}
