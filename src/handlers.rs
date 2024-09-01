use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, Path, State},
    response::IntoResponse,
    Json,
};
use hyper::{Request, StatusCode};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use validator::Validate;

use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let message = format!("Json parse error: [{}]", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

pub async fn create_todo<T: TodoRepository>(
    State(repository): State<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todos = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(todos)))
}

pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    State(repository): State<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    State(repository): State<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
