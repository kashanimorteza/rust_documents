//--------------------------------------------------------------------------------- Location
// src/main.rs

//--------------------------------------------------------------------------------- Description
// Main Axum server with proper structure: Routes | Handlers | Extractors | Middlewares | State Management

//--------------------------------------------------------------------------------- Import
use axum::{middleware::from_fn, Router};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod models;
mod orm;

//--------------------------------------------------------------------------------- State Management
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

//--------------------------------------------------------------------------------- Main
#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_1=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite::memory:".to_string());
    
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // State management
    let state = AppState { db };

    // Middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(from_fn(api::middleware::logging_middleware));

    // Routes configuration
    let app = Router::new()
        .nest("/users", api::routes::user::router())
        .layer(middleware_stack)
        .with_state(state);

    // Server configuration
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!("🚀 Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}