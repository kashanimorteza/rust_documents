//--------------------------------------------------------------------------------- Location
// src/handlers/user.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for User CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use crate::{orm::models::user::Model as UserModel, AppState};

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub username: String,
    pub password: String,
    pub key: String,
    pub email: String,
    pub phone: String,
    pub tg_id: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub key: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tg_id: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserModel>>, StatusCode> {
    match UserModel::select_all(&state.db).await {
        Ok(users) => Ok(Json(users)),
        Err(err) => {
            tracing::error!("Failed to fetch users: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserModel>, StatusCode> {
    match UserModel::select_by_id(&state.db, id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            tracing::error!("Failed to fetch user {}: {}", id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserModel>, StatusCode> {
    use crate::orm::models::user::ActiveModel as UserActiveModel;
    use sea_orm::Set;

    let active_user = UserActiveModel {
        name: Set(payload.name),
        username: Set(payload.username),
        password: Set(payload.password),
        key: Set(payload.key),
        email: Set(payload.email),
        phone: Set(payload.phone),
        tg_id: Set(payload.tg_id),
        enable: Set(payload.enable),
        ..Default::default()
    };

    match UserModel::insert(&state.db, active_user).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            tracing::error!("Failed to create user: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserModel>, StatusCode> {
    use crate::orm::models::user::{ActiveModel as UserActiveModel, Entity as UserEntity};
    use sea_orm::{ActiveValue::Set, ActiveModelTrait, EntityTrait};

    match UserEntity::find_by_id(id).one(&state.db).await {
        Ok(Some(existing_user)) => {
            let mut active_user: UserActiveModel = existing_user.into();
            
            if let Some(name) = payload.name { active_user.name = Set(name); }
            if let Some(username) = payload.username { active_user.username = Set(username); }
            if let Some(password) = payload.password { active_user.password = Set(password); }
            if let Some(key) = payload.key { active_user.key = Set(key); }
            if let Some(email) = payload.email { active_user.email = Set(email); }
            if let Some(phone) = payload.phone { active_user.phone = Set(phone); }
            if let Some(tg_id) = payload.tg_id { active_user.tg_id = Set(tg_id); }
            if let Some(enable) = payload.enable { active_user.enable = Set(enable); }

            match active_user.update(&state.db).await {
                Ok(updated_user) => Ok(Json(updated_user)),
                Err(err) => {
                    tracing::error!("Failed to update user {}: {}", id, err);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            tracing::error!("Failed to find user {}: {}", id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match UserModel::delete(&state.db, id).await {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(err) => {
            tracing::error!("Failed to delete user {}: {}", id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
