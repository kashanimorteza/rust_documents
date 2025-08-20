//--------------------------------------------------------------------------------- Location
// src/api/routes/user.rs

//--------------------------------------------------------------------------------- Description
// This is route for user

//--------------------------------------------------------------------------------- Import
use serde::{Deserialize};
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::user::Model as UserModel;
use crate::orm::models::general::ModelOutput;
use crate::api::services::user::UserService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddUserRequest 
{
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
pub struct UpdateUserRequest 
{
    pub id: i32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub key: String,
    pub email: String,
    pub phone: String,
    pub tg_id: String,
    pub enable: bool,
}



//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddUserRequest>) -> Json<ModelOutput<UserModel>> 
{
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
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<UserModel>>> 
{
    let service = UserService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateUserRequest>, ) -> Json<ModelOutput<UserModel>> 
{
    let service = UserService::new();
    
    let user_model = UserModel 
    {
        id: payload.id,
        name: payload.name,
        username: payload.username,
        password: payload.password,
        key: payload.key,
        email: payload.email,
        phone: payload.phone,
        tg_id: payload.tg_id,
        enable: payload.enable,
    };
    
    let result = service.update(&state.db, user_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_user(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = UserService::new();
    let result = service.delete(&state.db, id).await;
    Json(result)
}

//-------------------------- [Disable]
async fn disable(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<UserModel>> 
{
    let service = UserService::new();
    let result = service.disable(&state.db, id).await;
    Json(result)
}

//-------------------------- [Dead]
async fn dead(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = UserService::new();
    let result = service.dead(&state.db, id).await;
    Json(result)
}



//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/add", post(add))
        .route("/items", get(items))
        .route("/update", put(update))
        .route("/delete/{id}", delete(delete_user))
        .route("/disable/{id}", get(disable))
        .route("/dead/{id}", get(dead))
}