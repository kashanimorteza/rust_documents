# api_1
    this is api_1



<!--------------------------------------------------------------------------------- API -->
<br><br>

# API
<!------------------------- Install -->
Install
```bash
cargo add dotenvy
cargo add tokio --features full
cargo add sea-orm --no-default-features --features sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-async-std-rustls,runtime-tokio-rustls,macros,debug-print,seaography,with-uuid,with-chrono,with-json,with-bigdecimal,with-time
cargo add axum
cargo add serde
cargo add serde_json
cargo add thiserror
```



<!--------------------------------------------------------------------------------- API -->
<br><br>

# Axum API Documentation

## Overview
This is a complete Axum API implementation following the structure: **Routes | Handlers | Extractors | Middlewares | State Management**

## Project Structure
```
src/
├── main.rs              # Main server with state management and middleware
├── handlers/            # Request handlers
│   ├── mod.rs
│   └── user.rs         # User CRUD handlers
├── routes/              # Route definitions
│   ├── mod.rs
│   └── user.rs         # User routes
├── middleware/          # Custom middleware
│   └── mod.rs          # Logging middleware
├── models/              # Data models
│   ├── mod.rs
│   ├── prelude.rs
│   └── user.rs         # User model with SeaORM
└── logics/              # Business logic
    ├── mod.rs
    └── user.rs         # User CRUD operations
```

## Features Implemented

### 1. Routes
- **GET /users** - List all users
- **GET /users/{id}** - Get user by ID
- **POST /users** - Create new user
- **PUT /users/{id}** - Update user
- **DELETE /users/{id}** - Delete user

### 2. Handlers
- Proper error handling with HTTP status codes
- JSON request/response handling
- Path parameter extraction
- State injection for database access

### 3. Extractors
- `State<AppState>` - Database connection injection
- `Path<i32>` - URL path parameter extraction
- `Json<T>` - JSON request body parsing

### 4. Middlewares
- **CORS** - Cross-origin resource sharing
- **Tracing** - HTTP request tracing
- **Custom Logging** - Request/response logging with timing

### 5. State Management
- `AppState` struct containing database connection
- Shared state across all handlers
- Database connection pooling via SeaORM

## API Endpoints

### List Users
```bash
GET /users
Response: Array of user objects
```

### Get User
```bash
GET /users/{id}
Response: Single user object or 404
```

### Create User
```bash
POST /users
Content-Type: application/json

{
  "name": "string",
  "username": "string", 
  "password": "string",
  "key": "string",
  "email": "string",
  "phone": "string",
  "tg_id": "string",
  "enable": boolean
}

Response: Created user object
```

### Update User
```bash
PUT /users/{id}
Content-Type: application/json

{
  "name": "string (optional)",
  "username": "string (optional)",
  "password": "string (optional)", 
  "key": "string (optional)",
  "email": "string (optional)",
  "phone": "string (optional)",
  "tg_id": "string (optional)",
  "enable": "boolean (optional)"
}

Response: Updated user object or 404
```

### Delete User
```bash
DELETE /users/{id}
Response: 204 No Content or 404
```

## Running the Server

1. Set environment variable (optional):
   ```bash
   export DATABASE_URL="sqlite::memory:"
   ```

2. Start the server:
   ```bash
   cargo run
   ```

3. Server will listen on `http://0.0.0.0:3000`

## Dependencies
- **axum** - Web framework
- **sea-orm** - ORM for database operations
- **serde** - JSON serialization/deserialization
- **tokio** - Async runtime
- **tower** - Middleware framework
- **tower-http** - HTTP middleware (CORS, tracing)
- **tracing** - Structured logging
- **dotenvy** - Environment variable loading

## Database
Uses SeaORM with SQLite in-memory database by default. Can be configured via `DATABASE_URL` environment variable to use PostgreSQL, MySQL, or file-based SQLite.
