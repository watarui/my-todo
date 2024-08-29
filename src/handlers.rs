use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use std::sync::Arc;

use crate::repositories::{CreateTodo, TodoRepository};

pub async fn create_todo<T: TodoRepository>(
    State(repository): State<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo))
}
