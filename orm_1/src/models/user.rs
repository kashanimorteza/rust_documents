//--------------------------------------------------------------------------------- Location
// src/models/user.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the User model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;

//--------------------------------------------------------------------------------- Action
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text")]
    pub key: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub phone: String,
    #[sea_orm(column_type = "Text")]
    pub tg_id: String,
    pub enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
