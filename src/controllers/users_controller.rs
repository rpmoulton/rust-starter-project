use crate::models::api_error::APIerror;
use crate::models::user::User;
use crate::services::user_service::{user_create, user_delete, user_index, user_update};
use axum::extract::{Path, Extension};
use axum::{Json};
use axum_macros::debug_handler;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[debug_handler]
pub async fn user_create_controller(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Json(user_data): Json<User>,
) -> Result<Json<User>, APIerror> {
    user_create(Extension(db), Json(user_data)).await
}

#[debug_handler]
pub async fn user_update_controller(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Path(id): Path<i32>,
    Json(user_data): Json<User>,
) -> Result<Json<User>, APIerror> {
    user_update(Extension(db), Path(id), Json(user_data)).await
}

#[debug_handler]
pub async fn user_delete_controller(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Path(id): Path<i32>,
) -> Result<Json<bool>, APIerror> {
    user_delete(Extension(db), Path(id)).await
}

#[debug_handler]
pub async fn user_index_controller(
    Extension(db): Extension<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<User>>, APIerror> {
    user_index(Extension(db)).await
}