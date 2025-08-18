//--------------------------------------------------------------------------------- Location
// src/main.rs

//--------------------------------------------------------------------------------- Description
// This is main

//--------------------------------------------------------------------------------- Import
pub use sea_orm::entity::prelude::*;
pub use sea_orm::Database;
pub mod models;
pub mod logics;
pub use sea_orm::{EntityTrait, DbConn, DbErr};
pub use models::user::{Entity as UserEntity, Model as UserModel, ActiveModel as UserActiveModel};
use sea_orm::ActiveValue::Set;
use dotenvy::dotenv;
use std::env;

//--------------------------------------------------------------------------------- Action
#[tokio::main]
async fn main() -> Result<(), DbErr> 
{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let db = Database::connect(&database_url).await?;

    // Insert
    let created = UserModel::insert(
        &db,
        UserActiveModel {
            name: Set("John Doe".to_owned()),
            username: Set("jdoe".to_owned()),
            password: Set("secret".to_owned()),
            key: Set("api-key-1".to_owned()),
            email: Set("john@example.com".to_owned()),
            phone: Set("+123456789".to_owned()),
            tg_id: Set("tg-123".to_owned()),
            enable: Set(true),
            ..Default::default()
        },
    ).await?;
    println!("Inserted: {:?}", created);

    // Update
    let mut to_update: UserActiveModel = created.clone().into();
    to_update.name = Set("John Doe Jr.".to_owned());
    to_update.username = Set("johnny".to_owned());
    to_update.email = Set("johnny@example.com".to_owned());
    let updated = UserModel::update(&db, to_update).await?;
    println!("Updated: {:?}", updated);

    // Select by id
    let fetched = UserModel::select_by_id(&db, updated.id).await?;
    println!("Selected by id: {:?}", fetched);

    // Select all
    let users = UserModel::select_all(&db).await?;
    println!("Select all ({} rows)", users.len());
    for u in users {
        println!("{:?}", u);
    }

    // Delete
    // let rows_affected = UserModel::delete(&db, updated.id).await?;
    // println!("Deleted rows: {}", rows_affected);

    Ok(())
}