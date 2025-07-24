use crate::models::api_error::APIerror;
use crate::models::user::User;
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use crate::entity;
use std::sync::Arc;

pub async fn user_create(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Json(user_data): Json<User>,
) -> Result<Json<User>, APIerror> {
    let conn_db = db.as_ref();
    let hashpassword = hash(&user_data.password, DEFAULT_COST).map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    let user = entity::users::ActiveModel {
        username: Set(user_data.username.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(hashpassword.to_owned()),
        phone: Set(user_data.phone.to_owned()),
        updated_at: Set(Some(Utc::now().naive_local())),
        created_at: Set(Some(Utc::now().naive_local())),
        is_active: Set(Some(true)),
        ..Default::default()
    };

    let created_user = user.insert(conn_db).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(User {
        id: created_user.id,
        username: created_user.username,
        email: created_user.email,
        phone: created_user.phone,
        password: "".to_string(), // Don't return password
        created_at: created_user.created_at,
        updated_at: created_user.updated_at,
        is_active: created_user.is_active,
    }))
}


pub async fn user_update(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Path(id): Path<i32>,
    Json(user_data): Json<User>,
) -> Result<Json<User>, APIerror> { 
    let conn_db = db.as_ref();
    let mut user: entity::users::ActiveModel = entity::users::Entity::find()
        .filter(entity::users::Column::Id.eq(id))
        .one(conn_db)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or_else(|| APIerror {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?
        .into();

    user.email = Set(user_data.email.to_owned());
    user.username = Set(user_data.username.to_owned());
    user.phone = Set(user_data.phone.to_owned());
    user.updated_at = Set(Some(Utc::now().naive_local()));

    let updated_user = user.update(conn_db).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(User {
        id: updated_user.id,
        username: updated_user.username,
        email: updated_user.email,
        phone: updated_user.phone,
        password: "".to_string(),
        created_at: updated_user.created_at,
        updated_at: updated_user.updated_at,
        is_active: updated_user.is_active,
    }))
}

pub async fn user_delete(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Path(id): Path<i32>,
) -> Result<Json<bool>, APIerror> { 
    let conn_db = db.as_ref();
    let mut user: entity::users::ActiveModel = entity::users::Entity::find()
        .filter(entity::users::Column::Id.eq(id))
        .one(conn_db)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or_else(|| APIerror {
            message: "User doesn't exists".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?
        .into();
    user.is_active = Set(Some(false));

    user.update(conn_db).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    Ok(Json(true))
}

pub async fn user_index(
    Extension(db): Extension<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<User>>, APIerror> { 
    let conn_db = db.as_ref();
    let users = entity::users::Entity::find()
        .all(conn_db)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let user_list = users
        .into_iter()
        .map(|user| User {
            id: user.id,
            username: user.username,
            email: user.email,
            phone: user.phone,
            password: "".to_string(), // Don't return password
            created_at: user.created_at,
            updated_at: user.updated_at,
            is_active: user.is_active,
        })
        .collect();

    Ok(Json(user_list))
}