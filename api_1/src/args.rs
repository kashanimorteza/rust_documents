//--------------------------------------------------------------------------------- Location
// src/args.rs

//--------------------------------------------------------------------------------- Description
// This file contains argument handling logic for the application

//--------------------------------------------------------------------------------- Import
use crate::logics;
use sea_orm::DatabaseConnection;

//--------------------------------------------------------------------------------- Argument Handler
pub async fn handle_arguments(db: &DatabaseConnection) -> Result<bool, Box<dyn std::error::Error>> 
{
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 
    {
        match args[1].as_str() 
        {
            "--add-users" => 
            {
                println!("🔧 Adding sample users...");
                if let Err(e) = logics::user::add_sample_users(db).await 
                {
                    eprintln!("❌ Error adding sample users: {}", e);
                }
            
                println!("\n📋 Listing all users:");
                if let Err(e) = logics::user::list_all_users(db).await 
                {
                    eprintln!("❌ Error listing users: {}", e);
                }
                
                println!("\n✅ Sample users added successfully!");
                return Ok(true); // Return true to indicate we should exit
            }
            "--add-samples" => 
            {
                let admin = logics::admin::Admin::new();
                if let Err(e) = admin.add_samples(db).await {
                    eprintln!("❌ Error adding samples: {}", e);
                }
                
                println!("\n📋 Listing all users:");
                if let Err(e) = logics::user::list_all_users(db).await {
                    eprintln!("❌ Error listing users: {}", e);
                }
                
                return Ok(true); // Return true to indicate we should exit
            }
            "--help" | "-h" => 
            {
                println!("🚀 Axum API Server");
                println!("Usage: {} [OPTIONS]", args[0]);
                println!();
                println!("Options:");
                println!("  --add-users    Add sample users to the database");
                println!("  --add-samples  Add sample data via admin module");
                println!("  --help, -h     Show this help message");
                println!();
                println!("If no options are provided, the server will start normally on port 3000.");
                return Ok(true); // Return true to indicate we should exit
            }
            _ => 
            {
                eprintln!("❌ Unknown argument: {}", args[1]);
                eprintln!("Use --help for available options.");
                return Ok(true); // Return true to indicate we should exit
            }
        }
    }
    
    Ok(false) // Return false to continue with normal server startup
}
