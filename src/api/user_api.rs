use axum::{Router, routing::{post, get}, Json};
use sea_orm::DatabaseConnection;
use crate::service::user_service;
use crate::model::user_model::{CreateUserRequest, UpdateUserRequest, DeleteUserRequest, UserResponse};

pub fn routes(db: DatabaseConnection) -> Router {
    Router::new()
    .route("/user", get(select_user))
    .route("/user/create", post(create_user))
    .route("/user/update", post(update_user))
    .route("/user/delete", post(delete_user))
    .with_state(db)
}

#[axum::debug_handler]
async fn select_user(db: axum::extract::State<DatabaseConnection>) -> Result<Json<Vec<UserResponse>>, axum::http::StatusCode> {
    user_service::select_user(&db).await.map(Json).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[axum::debug_handler]
async fn create_user(db: axum::extract::State<DatabaseConnection>, Json(payload): Json<CreateUserRequest>) -> Result<Json<UserResponse>, axum::http::StatusCode> {
    user_service::create_user(&db, payload).await.map(Json).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[axum::debug_handler]
async fn update_user(db: axum::extract::State<DatabaseConnection>, Json(payload): Json<UpdateUserRequest>) -> Result<Json<UserResponse>, axum::http::StatusCode> {
    user_service::update_user(&db, payload).await.map(Json).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

#[axum::debug_handler]
async fn delete_user(db: axum::extract::State<DatabaseConnection>, Json(payload): Json<DeleteUserRequest>) -> Result<Json<UserResponse>, axum::http::StatusCode> {
    user_service::delete_user(&db, payload).await.map(Json).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}
