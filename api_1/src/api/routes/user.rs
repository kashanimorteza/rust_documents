use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;

use crate::AppState;

// از مدل‌ها و منطق خودت استفاده می‌کنیم:
use crate::models::user::{
    ActiveModel as UserActiveModel,
    Model as UserModel,
};
// Use the UserModel directly and call the logic methods on it

// ---------- DTO ها ----------
#[derive(Deserialize)]
pub struct CreateUser {
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
pub struct UpdateUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub key: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tg_id: Option<String>,
    pub enable: Option<bool>,
}

#[derive(Serialize)]
pub struct ListResponse<T> {
    pub total: usize,
    pub items: Vec<T>,
}

// ---------- هندلرها ----------
async fn list_users(State(st): State<AppState>) -> axum::response::Result<Json<ListResponse<UserModel>>> {
    let items = UserModel::select_all(&st.db).await.map_err(to_ise)?;
    Ok(Json(ListResponse { total: items.len(), items }))
}

async fn get_user(Path(id): Path<i32>, State(st): State<AppState>) -> axum::response::Result<Json<UserModel>> {
    match UserModel::select_by_id(&st.db, id).await.map_err(to_ise)? {
        Some(m) => Ok(Json(m)),
        None => Err(axum::http::StatusCode::NOT_FOUND.into()),
    }
}

async fn create_user(State(st): State<AppState>, Json(payload): Json<CreateUser>) -> axum::response::Result<Json<UserModel>> {
    let active = UserActiveModel {
        // اگر PK auto-increment است، NotSet بگذار (Default::default())
        id: Default::default(),
        name: Set(payload.name),
        username: Set(payload.username),
        password: Set(payload.password),
        key: Set(payload.key),
        email: Set(payload.email),
        phone: Set(payload.phone),
        tg_id: Set(payload.tg_id),
        enable: Set(payload.enable),
    };
    let inserted = UserModel::insert(&st.db, active).await.map_err(to_ise)?;
    Ok(Json(inserted))
}

async fn update_user(Path(id): Path<i32>, State(st): State<AppState>, Json(payload): Json<UpdateUser>)
-> axum::response::Result<Json<UserModel>>
{
    // ابتدا رکورد فعلی را بگیریم
    let Some(current) = UserModel::select_by_id(&st.db, id).await.map_err(to_ise)? else {
        return Err(axum::http::StatusCode::NOT_FOUND.into());
    };

    // به ActiveModel تبدیل کنیم تا فیلدها را Set/NotSet کنیم
    let mut active: UserActiveModel = current.into();

    if let Some(v) = payload.name     { active.name     = Set(v); }
    if let Some(v) = payload.username { active.username = Set(v); }
    if let Some(v) = payload.password { active.password = Set(v); }
    if let Some(v) = payload.key      { active.key      = Set(v); }
    if let Some(v) = payload.email    { active.email    = Set(v); }
    if let Some(v) = payload.phone    { active.phone    = Set(v); }
    if let Some(v) = payload.tg_id    { active.tg_id    = Set(v); }
    if let Some(v) = payload.enable   { active.enable   = Set(v); }

    let updated = UserModel::update(&st.db, active).await.map_err(to_ise)?;
    Ok(Json(updated))
}

async fn delete_user(Path(id): Path<i32>, State(st): State<AppState>) -> axum::response::Result<()> {
    let rows = UserModel::delete(&st.db, id).await.map_err(to_ise)?;
    if rows == 0 {
        return Err(axum::http::StatusCode::NOT_FOUND.into());
    }
    Ok(())
}

// ---------- Router ----------
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}

// ---------- کمکی ----------
fn to_ise<E: std::error::Error>(e: E) -> axum::http::StatusCode {
    eprintln!("DB error: {e}");
    axum::http::StatusCode::INTERNAL_SERVER_ERROR
}
