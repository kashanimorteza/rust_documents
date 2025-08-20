//--------------------------------------------------------------------------------- Location
// src/handlers/user.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for User CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{orm::models::user::Model as UserModel, orm::models::general::ModelOutput, AppState};
use crate::api::services::user::UserService;

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
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<UserModel>>>, StatusCode> {
    let service = UserService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<UserModel>>, StatusCode> {
    let service = UserService::new();
    // For now, we'll use items with empty filters and then filter by id
    // In a real implementation, you'd add a get_by_id method to the service
    let mut filters = std::collections::HashMap::new();
    filters.insert("id".to_string(), id.to_string());
    let result = service.items(&state.db, filters).await;
    
    // Convert Vec<UserModel> result to single UserModel result
    match result.data {
        Some(users) if !users.is_empty() => {
            Ok(Json(ModelOutput::success(users[0].clone(), "User found".to_string())))
        }
        _ => Ok(Json(ModelOutput::error("User not found".to_string())))
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ModelOutput<UserModel>>, StatusCode> {
    let service = UserService::new();
    let user_model = UserModel {
        id: 0, // Will be auto-generated
        name: payload.name,
        username: payload.username,
        password: payload.password,
        key: payload.key,
        email: payload.email,
        phone: payload.phone,
        tg_id: payload.tg_id,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, user_model).await;
    Ok(Json(result))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ModelOutput<UserModel>>, StatusCode> {
    let service = UserService::new();
    
    // Create a user model with the provided data
    // For simplicity, we'll use default values for required fields that aren't provided
    let user_model = UserModel {
        id,
        name: payload.name.unwrap_or_default(),
        username: payload.username.unwrap_or_default(),
        password: payload.password.unwrap_or_default(),
        key: payload.key.unwrap_or_default(),
        email: payload.email.unwrap_or_default(),
        phone: payload.phone.unwrap_or_default(),
        tg_id: payload.tg_id.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, user_model).await;
    Ok(Json(result))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = UserService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
