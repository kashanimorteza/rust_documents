//--------------------------------------------------------------------------------- Location
// src/middleware/mod.rs

//--------------------------------------------------------------------------------- Description
// Middleware configuration

//--------------------------------------------------------------------------------- Import
use axum::{http::Request, middleware::Next, response::Response};
use std::time::Instant;

//--------------------------------------------------------------------------------- Middleware
pub async fn logging_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    tracing::info!(
        method = %method,
        uri = %uri,
        status = %status,
        duration = ?duration,
        "Request processed"
    );
    
    response
}
