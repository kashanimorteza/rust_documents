# User API - Rust Axum Server

A REST API built with Rust and Axum framework for user management operations.


## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo

### Install
```bash
cargo add dotenvy
cargo add tokio --features full
cargo add sea-orm --no-default-features --features sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-async-std-rustls,runtime-tokio-rustls,macros,debug-print,seaography,with-uuid,with-chrono,with-json,with-bigdecimal,with-time
cargo add axum
cargo add serde
cargo add serde_json
cargo add thiserror
```

### Running the Server
```bash
cargo run
```

The server will start on `http://0.0.0.0:3000`

## 📋 API Endpoints

### 1. Get All Users
**Endpoint:** `GET /users/items`

```bash
curl -X GET http://localhost:3000/users/items
```

**Response:**
```json
{
  "success": true,
  "message": "Users retrieved successfully",
  "data": [
    {
      "id": 1,
      "name": "John Doe",
      "username": "johndoe",
      "password": "secret",
      "key": "api-key-1",
      "email": "john@example.com",
      "phone": "+123456789",
      "tg_id": "tg-123",
      "enable": true
    }
  ],
  "error": null
}
```

### 2. Get User by ID
**Endpoint:** `GET /users/{id}`

```bash
curl -X GET http://localhost:3000/users/1
```

**Response:**
```json
{
  "success": true,
  "message": "User found",
  "data": {
    "id": 1,
    "name": "John Doe",
    "username": "johndoe",
    "password": "secret",
    "key": "api-key-1",
    "email": "john@example.com",
    "phone": "+123456789",
    "tg_id": "tg-123",
    "enable": true
  },
  "error": null
}
```

### 3. Add New User
**Endpoint:** `POST /users/add`

```bash
curl -X POST http://localhost:3000/users/add \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Smith",
    "username": "janesmith",
    "password": "securepass123",
    "key": "api-key-jane",
    "email": "jane@example.com",
    "phone": "+987654321",
    "tg_id": "tg-jane",
    "enable": true
  }'
```

**Response:**
```json
{
  "success": true,
  "message": "User added successfully",
  "data": {
    "id": 2,
    "name": "Jane Smith",
    "username": "janesmith",
    "password": "securepass123",
    "key": "api-key-jane",
    "email": "jane@example.com",
    "phone": "+987654321",
    "tg_id": "tg-jane",
    "enable": true
  },
  "error": null
}
```

### 4. Update User
**Endpoint:** `PUT /users/update`

```bash
curl -X PUT http://localhost:3000/users/update \
  -H "Content-Type: application/json" \
  -d '{
    "id": 19,
    "name": "vvvvvvv",
    "username": "janesmith_updated",
    "password": "newpassword123",
    "key": "api-key-jane-updated",
    "email": "jane.updated@example.com",
    "phone": "+987654321",
    "tg_id": "tg-jane-updated",
    "enable": true
  }'
```

**Response:**
```json
{
  "success": true,
  "message": "User updated successfully",
  "data": {
    "id": 2,
    "name": "Jane Smith Updated",
    "username": "janesmith_updated",
    "password": "newpassword123",
    "key": "api-key-jane-updated",
    "email": "jane.updated@example.com",
    "phone": "+987654321",
    "tg_id": "tg-jane-updated",
    "enable": true
  },
  "error": null
}
```

### 5. Delete User
**Endpoint:** `DELETE /users/delete/{id}`

```bash
curl -X DELETE http://localhost:3000/users/delete/2
```

**Response:**
```json
{
  "success": true,
  "message": "User deleted successfully",
  "data": "deleted",
  "error": null
}
```

### 6. Disable User
**Endpoint:** `GET /users/disable/{id}`

```bash
curl -X GET http://localhost:3000/users/disable/1
```

**Response:**
```json
{
  "success": true,
  "message": "User disabled successfully",
  "data": {
    "id": 1,
    "name": "John Doe",
    "username": "johndoe",
    "password": "secret",
    "key": "api-key-1",
    "email": "john@example.com",
    "phone": "+123456789",
    "tg_id": "tg-123",
    "enable": false
  },
  "error": null
}
```

### 7. Enable User
**Endpoint:** `GET /users/enable/{id}`

```bash
curl -X GET http://localhost:3000/users/enable/1
```

**Response:**
```json
{
  "success": true,
  "message": "User enabled successfully",
  "data": {
    "id": 1,
    "name": "John Doe",
    "username": "johndoe",
    "password": "secret",
    "key": "api-key-1",
    "email": "john@example.com",
    "phone": "+123456789",
    "tg_id": "tg-123",
    "enable": true
  },
  "error": null
}
```

### 8. Mark User as Dead
**Endpoint:** `GET /users/dead/{id}`

```bash
curl -X GET http://localhost:3000/users/dead/1
```

**Response:**
```json
{
  "success": true,
  "message": "User deleted successfully",
  "data": "deleted",
  "error": null
}
```

## 📊 Response Format

All API responses follow this consistent format:

```json
{
  "success": boolean,
  "message": "string",
  "data": object | array | string | null,
  "error": string | null
}
```

### Success Response
- `success`: `true`
- `message`: Descriptive success message
- `data`: The actual response data
- `error`: `null`

### Error Response
- `success`: `false`
- `message`: "Error"
- `data`: `null`
- `error`: Error description

## 🏗️ Architecture

The API follows a clean layered architecture:

```
API Layer (Routes/Handlers) → Service Layer → ORM/Logic Layer → Database
```

### Key Components:
- **Routes**: HTTP endpoint definitions
- **Handlers**: Request/response processing
- **Services**: Business logic coordination
- **ORM/Logic**: Database operations and business rules
- **Models**: Data structures and entities

## 🔧 Configuration

### Database
By default, the API uses SQLite in-memory database. You can configure it using the `DATABASE_URL` environment variable:

```bash
export DATABASE_URL="sqlite::memory:"
# or for file-based SQLite:
export DATABASE_URL="sqlite:database.db"
# or for PostgreSQL:
export DATABASE_URL="postgresql://user:password@localhost/dbname"
```

### Logging
The API includes comprehensive logging with different levels:
- Request/response logging
- Business logic logging
- Error logging
- Debug information

## 🛠️ Development

### Build
```bash
cargo build
```

### Run in Development Mode
```bash
cargo run
```

### Run Tests
```bash
cargo test
```

## 📝 Notes

- All endpoints return JSON responses
- The API uses SeaORM for database operations
- Verbose logging is enabled by default
- CORS is enabled for all origins
- The server includes middleware for tracing and logging