// Integration tests for the task manager API
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

// Helper function to create test app
async fn create_test_app(pool: PgPool) -> axum::Router {
    use axum::routing::{delete, get, post, put};
    use tower_http::trace::TraceLayer;

    axum::Router::new()
        .route(
            "/auth/register",
            post(task_manager::handlers::auth::register),
        )
        .route("/auth/login", post(task_manager::handlers::auth::login))
        .route("/tasks", get(task_manager::handlers::tasks::get_tasks))
        .route("/tasks", post(task_manager::handlers::tasks::create_task))
        .route(
            "/tasks/:id",
            put(task_manager::handlers::tasks::update_task),
        )
        .route(
            "/tasks/:id",
            delete(task_manager::handlers::tasks::delete_task),
        )
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}

// Helper to setup test database
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/task_manager".to_string());

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Clean up existing data
    sqlx::query("TRUNCATE TABLE tasks, users CASCADE")
        .execute(&pool)
        .await
        .ok();

    pool
}

// Helper to create a test user and return token
async fn create_test_user_with_token(app: &axum::Router, email: &str) -> String {
    let register_body = json!({
        "email": email,
        "password": "testpassword123"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&register_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    json["token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_register_success() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let body = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["token"].is_string());
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let body = json!({
        "email": "duplicate@example.com",
        "password": "password123"
    });

    // First registration should succeed
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Second registration with same email should fail
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_login_success() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    // Register user first
    let register_body = json!({
        "email": "login@example.com",
        "password": "password123"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&register_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now login
    let login_body = json!({
        "email": "login@example.com",
        "password": "password123"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&login_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["token"].is_string());
}

#[tokio::test]
async fn test_login_wrong_password() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    // Register user
    let register_body = json!({
        "email": "test_wrong_pwd@example.com",
        "password": "correctpassword"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&register_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Login with wrong password
    let login_body = json!({
        "email": "test_wrong_pwd@example.com",
        "password": "wrongpassword"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&login_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_task_success() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let token = create_test_user_with_token(&app, "task_creator@example.com").await;

    let task_body = json!({
        "title": "Test Task",
        "description": "Test Description"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&task_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["title"], "Test Task");
    assert_eq!(json["description"], "Test Description");
    assert_eq!(json["done"], false);
}

#[tokio::test]
async fn test_create_task_without_auth() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let task_body = json!({
        "title": "Test Task",
        "description": "Test Description"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&task_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_tasks() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let token = create_test_user_with_token(&app, "task_getter@example.com").await;

    // Create a task first
    let task_body = json!({
        "title": "Get Test Task",
        "description": "For testing get endpoint"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&task_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now get all tasks
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/tasks")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.is_array());
    assert!(json.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn test_update_task() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let token = create_test_user_with_token(&app, "task_updater@example.com").await;

    // Create a task
    let task_body = json!({
        "title": "Original Title",
        "description": "Original Description"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&task_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let created_task: Value = serde_json::from_slice(&body).unwrap();
    let task_id = created_task["id"].as_str().unwrap();

    // Update the task
    let update_body = json!({
        "title": "Updated Title",
        "done": true
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/tasks/{}", task_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&update_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["title"], "Updated Title");
    assert_eq!(json["done"], true);
}

#[tokio::test]
async fn test_delete_task() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let token = create_test_user_with_token(&app, "task_deleter@example.com").await;

    // Create a task
    let task_body = json!({
        "title": "Task to Delete",
        "description": "Will be deleted"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&task_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let created_task: Value = serde_json::from_slice(&body).unwrap();
    let task_id = created_task["id"].as_str().unwrap();

    // Delete the task
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/tasks/{}", task_id))
                .header(header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_task_isolation_between_users() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;

    let token1 = create_test_user_with_token(&app, "user1@example.com").await;
    let token2 = create_test_user_with_token(&app, "user2@example.com").await;

    // User 1 creates a task
    let task_body = json!({
        "title": "User 1 Task",
        "description": "Private task"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {}", token1))
                .body(Body::from(serde_json::to_string(&task_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // User 2 gets their tasks
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/tasks")
                .header(header::AUTHORIZATION, format!("Bearer {}", token2))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // User 2 should not see User 1's tasks
    assert_eq!(json.as_array().unwrap().len(), 0);
}
