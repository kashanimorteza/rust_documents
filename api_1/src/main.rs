//--------------------------------------------------------------------------------- Location
// src/main.rs

//--------------------------------------------------------------------------------- Description
// Main Axum server with proper structure: Routes | Handlers | Extractors | Middlewares | State Management

//--------------------------------------------------------------------------------- Import
pub use axum::{middleware::from_fn, Router};
pub use dotenvy::dotenv;
pub use sea_orm::{Database, DatabaseConnection};
pub use std::net::SocketAddr;
pub use tower::ServiceBuilder;
pub use tower_http::{cors::CorsLayer, trace::TraceLayer};
pub use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub mod api;
pub mod orm;

//--------------------------------------------------------------------------------- State Management
#[derive(Clone)]
pub struct AppState 
{
    pub db: DatabaseConnection,
}

//--------------------------------------------------------------------------------- Main
#[tokio::main]
async fn main() 
{
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "api_1=debug,tower_http=debug".into()),)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    let db: DatabaseConnection = Database::connect(&database_url).await.expect("Failed to connect to database");

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
    let api_host = std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let api_port = std::env::var("API_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("{}:{}", api_host, api_port).parse().expect("invalid host:port");
    tracing::info!("🚀 Server listening on {}", addr);
    
    // Server Start
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}