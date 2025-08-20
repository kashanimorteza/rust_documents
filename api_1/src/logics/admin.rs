//--------------------------------------------------------------------------------- Location
// src/logics/admin.rs

//--------------------------------------------------------------------------------- Description
// This file contains admin logic for managing the system



//--------------------------------------------------------------------------------- Import
use crate::logics::user;
use sea_orm::DatabaseConnection;



//--------------------------------------------------------------------------------- Admin Logic
pub struct Admin;

impl Admin 
{
    pub fn new() -> Self 
    {
        Self
    }

    //-------------------------- [Add Samples]
    pub async fn add_samples(&self, db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 Admin: Starting to add sample data...");
        
        // Call the user module's add_sample_users function
        user::add_sample_users(db).await?;
        
        println!("✅ Admin: Sample data added successfully!");
        Ok(())
    }
}
