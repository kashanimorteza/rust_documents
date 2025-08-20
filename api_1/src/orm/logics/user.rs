//--------------------------------------------------------------------------------- Location
// src/orm/user.rs

//--------------------------------------------------------------------------------- Description
// This is orm for user



//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::user::{ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel};
use crate::logics::general::ModelOutput;



//--------------------------------------------------------------------------------- Class
pub struct UserORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl UserORM
{
    //-------------------------- [Init]
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "UserORM".to_string(),
            module: "user".to_string(),
        }
    }

    //-------------------------- [Add]
    pub async fn add(&self, db: &DbConn, item: UserActiveModel) -> ModelOutput<UserModel> 
    {
        let this_method = "add";
        
        if self.verbose 
        {
            debug!("{}::{} - Starting add operation", self.this_class, this_method);
        }

        match item.insert(db).await 
        {
            Ok(user) => {
                let output = ModelOutput::success(user, "User added successfully".to_string());
                if self.verbose 
                {
                    info!("{}::{} - Success: {:?}", self.this_class, this_method, output);
                }
                if self.log {
                    info!("LOG: {}::{} - User added", self.this_class, this_method);
                }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log 
                {
                    error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg);
                }
                output
            }
        }
    }

    //-------------------------- [Items]
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<UserModel>> 
    {
        let this_method = "items";
        
        if self.verbose 
        {
            debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters);
        }

        // For now, ignore filters and return all users
        // In a real implementation, you'd apply filters here
        match UserEntity::find().all(db).await 
        {
            Ok(users) => 
            {
                let output = ModelOutput::success(users, "Users retrieved successfully".to_string());
                if self.verbose 
                {
                    info!("{}::{} - Success: Retrieved {} users", self.this_class, this_method, output.data.as_ref().map_or(0, |d| d.len()));
                }
                if self.log 
                {
                    info!("LOG: {}::{} - Users retrieved", self.this_class, this_method);
                }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log 
                {
                    error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg);
                }
                output
            }
        }
    }

    //-------------------------- [Item]
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<UserModel> 
    {
        let this_method = "item";
        
        if self.verbose 
        {
            debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id);
        }

        match UserEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(user)) => 
            {
                let output = ModelOutput::success(user, "User retrieved successfully".to_string());
                if self.verbose 
                {
                    info!("{}::{} - Success: User {} found", self.this_class, this_method, id);
                }
                if self.log 
                {
                    info!("LOG: {}::{} - User {} retrieved", self.this_class, this_method, id);
                }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("User not found".to_string());
                if self.verbose 
                {
                    info!("{}::{} - User {} not found", self.this_class, this_method, id);
                }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log 
                {
                    error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg);
                }
                output
            }
        }
    }

    //-------------------------- [Update]
    pub async fn update(&self, db: &DbConn, item: UserActiveModel) -> ModelOutput<UserModel> 
    {
        let this_method = "update";
        
        if self.verbose 
        {
            debug!("{}::{} - Starting update operation", self.this_class, this_method);
        }

        match item.update(db).await 
        {
            Ok(user) => 
            {
                let output = ModelOutput::success(user, "User updated successfully".to_string());
                if self.verbose 
                {
                    info!("{}::{} - Success: {:?}", self.this_class, this_method, output);
                }
                if self.log 
                {
                    info!("LOG: {}::{} - User updated", self.this_class, this_method);
                }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log 
                {
                    error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg);
                }
                output
            }
        }
    }

    //-------------------------- [Delete]
    pub async fn delete(&self, db: &DbConn, id: i32) -> ModelOutput<String> 
    {
        let this_method = "delete";
        
        if self.verbose 
        {
            debug!("{}::{} - Starting delete operation for id: {}", self.this_class, this_method, id);
        }

        match UserEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "User deleted successfully".to_string());
                    if self.verbose 
                    {
                        info!("{}::{} - Success: User {} deleted", self.this_class, this_method, id);
                    }
                    if self.log 
                    {
                        info!("LOG: {}::{} - User {} deleted", self.this_class, this_method, id);
                    }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("User not found".to_string());
                    if self.verbose 
                    {
                        info!("{}::{} - User {} not found", self.this_class, this_method, id);
                    }
                    output
                }
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log 
                {
                    error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg);
                }
                output
            }
        }
    }

    //-------------------------- [Disable]
    pub async fn disable(&self, db: &DbConn, id: i32) -> ModelOutput<UserModel> 
    {
        let this_method = "disable";
        
        if self.verbose 
        {
            debug!("{}::{} - Starting disable operation for id: {}", self.this_class, this_method, id);
        }

        match UserEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(existing)) => 
            {
                let mut active: UserActiveModel = existing.into();
                active.enable = sea_orm::Set(false);
                
                match active.update(db).await 
                {
                    Ok(updated_user) => 
                    {
                        let output = ModelOutput::success(updated_user, "User disabled successfully".to_string());
                        if self.verbose 
                        {
                            info!("{}::{} - Success: User {} disabled", self.this_class, this_method, id);
                        }
                        if self.log 
                        {
                            info!("LOG: {}::{} - User {} disabled", self.this_class, this_method, id);
                        }
                        output
                    }
                    Err(e) => 
                    {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        let output = ModelOutput::error(error_msg.clone());
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        output
                    }
                }
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("User not found".to_string());
                if self.verbose 
                {
                    info!("{}::{} - User {} not found", self.this_class, this_method, id);
                }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                output
            }
        }
    }

    //-------------------------- [Enable]
    pub async fn enable(&self, db: &DbConn, id: i32) -> ModelOutput<UserModel> 
    {
        let this_method = "enable";
        
        if self.verbose 
        {
            debug!("{}::{} - Starting enable operation for id: {}", self.this_class, this_method, id);
        }

        match UserEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(existing)) => 
            {
                let mut active: UserActiveModel = existing.into();
                active.enable = sea_orm::Set(true);
                
                match active.update(db).await 
                {
                    Ok(updated_user) => 
                    {
                        let output = ModelOutput::success(updated_user, "User enabled successfully".to_string());
                        if self.verbose 
                        {
                            info!("{}::{} - Success: User {} enabled", self.this_class, this_method, id);
                        }
                        if self.log 
                        {
                            info!("LOG: {}::{} - User {} enabled", self.this_class, this_method, id);
                        }
                        output
                    }
                    Err(e) => 
                    {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        let output = ModelOutput::error(error_msg.clone());
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        output
                    }
                }
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("User not found".to_string());
                if self.verbose 
                {
                    info!("{}::{} - User {} not found", self.this_class, this_method, id);
                }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log 
                {
                    error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg);
                }
                output
            }
        }
    }

    //-------------------------- [Dead]
    pub async fn dead(&self, db: &DbConn, id: i32) -> ModelOutput<String> 
    {
        let this_method = "dead";
        
        if self.verbose 
        {
            debug!("{}::{} - Starting dead operation for id: {}", self.this_class, this_method, id);
        }

        // "Dead" operation - delete the user
        self.delete(db, id).await
    }
}