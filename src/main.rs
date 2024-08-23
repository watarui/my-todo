use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use hyper::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use std::env;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = create_app();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

async fn root() -> &'static str {
    tracing::debug!("Handling GET request to /");
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    tracing::debug!("Handling POST request to /users");
    let user = User {
        id: 1337,
        username: payload.username,
    };
    tracing::info!("Created user: {:?}", user);
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Debug)]
struct User {
    id: u64,
    username: String,
}
