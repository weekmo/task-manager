mod db;
mod errors;
mod handlers;
mod middleware;
mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url).await;

    let app = Router::new()
        // Auth routes
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        // Task routes (protected)
        .route("/tasks", get(handlers::tasks::get_tasks))
        .route("/tasks", post(handlers::tasks::create_task))
        .route("/tasks/:id", put(handlers::tasks::update_task))
        .route("/tasks/:id", delete(handlers::tasks::delete_task))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
