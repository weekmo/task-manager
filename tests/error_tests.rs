// Unit tests for error handling
use task_manager::errors::AppError;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[test]
fn test_app_error_display() {
    let error = AppError::Auth("Invalid token".to_string());
    assert_eq!(error.to_string(), "Authentication error: Invalid token");

    let error = AppError::NotFound("Task not found".to_string());
    assert_eq!(error.to_string(), "Not found: Task not found");

    let error = AppError::BadRequest("Invalid input".to_string());
    assert_eq!(error.to_string(), "Bad request: Invalid input");
}

#[test]
fn test_error_status_codes() {
    let error = AppError::Auth("test".to_string());
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let error = AppError::NotFound("test".to_string());
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let error = AppError::BadRequest("test".to_string());
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
