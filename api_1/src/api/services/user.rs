//--------------------------------------------------------------------------------- Location
// src/api/services/user.rs

//--------------------------------------------------------------------------------- Description
// This is service for user - Rust equivalent of Python service layer



//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::user::{Model as UserModel, ActiveModel as UserActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::user::UserORM;

//--------------------------------------------------------------------------------- Service
pub struct UserService {
    pub logic: UserORM,
}

impl UserService {
    //-------------------------- [Init]
    pub fn new() -> Self 
    {
        Self 
        {
            logic: UserORM::new(true, true), // verbose=true, log=true
        }
    }

    //-------------------------- [Add]
    pub async fn add(&self, db: &DatabaseConnection, item: UserModel) -> ModelOutput<UserModel> 
    {
        // Convert UserModel to ActiveModel (equivalent to item.dict() and del item['id'])
        let active_user = UserActiveModel 
        {
            id: Default::default(), // Auto-increment (equivalent to del item['id'])
            name: Set(item.name),
            username: Set(item.username),
            password: Set(item.password),
            key: Set(item.key),
            email: Set(item.email),
            phone: Set(item.phone),
            tg_id: Set(item.tg_id),
            enable: Set(item.enable),
        };

        // Call logic layer
        self.logic.add(db, active_user).await
    }

    //-------------------------- [Items]
    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<UserModel>> 
    {
        let output = self.logic.items(db, filters).await;
        
        // Convert data to dictionary format (equivalent to Python's item.toDict())
        if output.success 
        {
            // In Rust, the serialization is handled by serde automatically
            // The models already have Serialize derive, so they convert to JSON properly
            output
        } 
        else 
        {
            output
        }
    }

    //-------------------------- [Item]
    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<UserModel> 
    {
        // Call logic layer
        self.logic.item(db, id).await
    }

    //-------------------------- [Update]
    pub async fn update(&self, db: &DatabaseConnection, item: UserModel) -> ModelOutput<UserModel> 
    {
        // Convert UserModel to ActiveModel (equivalent to model_db(**item.dict()))
        let active_user = UserActiveModel 
        {
            id: Set(item.id),
            name: Set(item.name),
            username: Set(item.username),
            password: Set(item.password),
            key: Set(item.key),
            email: Set(item.email),
            phone: Set(item.phone),
            tg_id: Set(item.tg_id),
            enable: Set(item.enable),
        };

        // Call logic layer
        self.logic.update(db, active_user).await
    }

    //-------------------------- [Delete]
    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }

    //-------------------------- [Disable]
    pub async fn disable(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<UserModel> 
    {
        self.logic.disable(db, id).await
    }

    //-------------------------- [Enable]
    pub async fn enable(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<UserModel> 
    {
        self.logic.enable(db, id).await
    }

    //-------------------------- [Dead]
    pub async fn dead(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.dead(db, id).await
    }
}
