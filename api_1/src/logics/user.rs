//--------------------------------------------------------------------------------- Location
// src/logics/user

//--------------------------------------------------------------------------------- Description
// This file contains logic to add some users using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::user::UserORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::user::ActiveModel as UserActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Users Logic
pub async fn add_sample_users(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let user_orm = UserORM::new(true, true);
    let sample_users = vec![
        UserActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            name: Set("Morteza".to_string()),
            username: Set("mkashanii".to_string()),
            password: Set("aaaaa".to_string()),
            key: Set("alice-api-key-001".to_string()),
            email: Set("alice@example.com".to_string()),
            phone: Set("+1-555-0101".to_string()),
            tg_id: Set("alice_tg_123".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample users...", sample_users.len());
    
    for (index, user) in sample_users.into_iter().enumerate() 
    {
        let user_name = match &user.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding user {}: {}", index + 1, user_name);
        
        let result = user_orm.add(db, user).await;
        if result.success 
        {
            if let Some(added_user) = result.data 
            {
                println!("✅ Successfully added user: {} (ID: {})", added_user.name, added_user.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add user: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample users!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Users Logic
pub async fn list_all_users(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let user_orm = UserORM::new(true, true); // verbose and log enabled
    let filters = std::collections::HashMap::new(); // No filters, get all users
    
    println!("📋 Fetching all users...");
    
    let result = user_orm.items(db, filters).await;
    
    if result.success {
        if let Some(users) = result.data {
            println!("👥 Found {} users:", users.len());
            println!("{:-<80}", "");
            for user in users {
                let status = if user.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Username: {} | Email: {} | Status: {}", 
                    user.id, user.name, user.username, user.email, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No users found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching users: {}", error);
        }
    }
    
    Ok(())
}
