// Unit tests for Task model
use chrono::Utc;
use task_manager::models::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use uuid::Uuid;

#[test]
fn test_create_task_request_deserialization() {
    let json = r#"{"title": "Test Task", "description": "Test Description"}"#;
    let request: CreateTaskRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.title, "Test Task");
    assert_eq!(request.description, Some("Test Description".to_string()));
}

#[test]
fn test_create_task_request_without_description() {
    let json = r#"{"title": "Test Task"}"#;
    let request: CreateTaskRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.title, "Test Task");
    assert_eq!(request.description, None);
}

#[test]
fn test_update_task_request_partial() {
    let json = r#"{"done": true}"#;
    let request: UpdateTaskRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.title, None);
    assert_eq!(request.description, None);
    assert_eq!(request.done, Some(true));
}

#[test]
fn test_update_task_request_all_fields() {
    let json = r#"{"title": "Updated", "description": "New desc", "done": true}"#;
    let request: UpdateTaskRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.title, Some("Updated".to_string()));
    assert_eq!(request.description, Some("New desc".to_string()));
    assert_eq!(request.done, Some(true));
}

#[test]
fn test_task_serialization() {
    let task = Task {
        id: Uuid::nil(),
        user_id: Uuid::nil(),
        title: "Test Task".to_string(),
        description: Some("Description".to_string()),
        done: false,
        created_at: Utc::now(),
    };

    let json = serde_json::to_value(&task).unwrap();
    assert_eq!(json["title"], "Test Task");
    assert_eq!(json["description"], "Description");
    assert_eq!(json["done"], false);
    assert!(json["id"].is_string());
    assert!(json["user_id"].is_string());
}

#[test]
fn test_task_with_null_description() {
    let task = Task {
        id: Uuid::nil(),
        user_id: Uuid::nil(),
        title: "Test Task".to_string(),
        description: None,
        done: false,
        created_at: Utc::now(),
    };

    let json = serde_json::to_value(&task).unwrap();
    assert_eq!(json["title"], "Test Task");
    assert!(json["description"].is_null());
}
