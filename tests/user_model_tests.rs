// Unit tests for User model
use chrono::Utc;
use task_manager::models::user::{AuthResponse, LoginRequest, RegisterRequest, User};
use uuid::Uuid;

#[test]
fn test_register_request_deserialization() {
    let json = r#"{"email": "test@example.com", "password": "password123"}"#;
    let request: RegisterRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.email, "test@example.com");
    assert_eq!(request.password, "password123");
}

#[test]
fn test_login_request_deserialization() {
    let json = r#"{"email": "test@example.com", "password": "password123"}"#;
    let request: LoginRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.email, "test@example.com");
    assert_eq!(request.password, "password123");
}

#[test]
fn test_auth_response_serialization() {
    let response = AuthResponse {
        token: "test_token_123".to_string(),
    };

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("test_token_123"));
    assert!(json.contains("token"));
}

#[test]
fn test_user_serialization() {
    let user = User {
        id: Uuid::nil(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        created_at: Utc::now(),
    };

    let json = serde_json::to_value(&user).unwrap();
    assert_eq!(json["email"], "test@example.com");
    assert!(json["id"].is_string());
    assert!(json["created_at"].is_string());
}
