
# API : Axum 
Resource : [crates] | [docs] | [github]

Structure :

    Middlewares | Route | Extractors | Handler | Responses

Flow

- TCP → HTTP Request (Tokio + hyper)
- Middleware stack (Trace, CORS, Auth, Logging, …)
- Router → Match route by path + method
- Extractors → State, Json, Path, Query, Headers …
- Handler (calls Service layer, does DB ops, returns data)
- Response wrapped & sent back → Middleware → hyper → Client



<!--------------------------------------------------------------------------------- Install -->
<br><br>

## Install
Cargo
```bash
cargo add axum
```


<!--------------------------------------------------------------------------------- Handler -->
<br><br>

## Middleware

TraceLayer
TraceLayer
Custom Middleware



<!--------------------------------------------------------------------------------- Route -->
<br><br>

## Route

Init
```bash
```


<!--------------------------------------------------------------------------------- Extractor -->
<br><br>

## Extractor
- An extractor is a mechanism that tells the framework how to take parts of an incoming HTTP request and turn them into strongly typed Rust values that you can use directly in your handler functions
- Extractors "extract" data from the request and inject it into your handler’s function parameters
- Extractors are the bridge between raw HTTP requests and strongly typed Rust values in Axum handlers

```rust
Path<T> → extract route parameters, like /users/:id.
Query<T> → extract query string parameters, e.g. /search?term=rust.
Json<T> → parse request bodies as JSON into a struct.
Form<T> → parse application/x-www-form-urlencoded bodies.
State<T> → get shared application state (like a database pool).
TypedHeader<T> → extract typed HTTP headers (e.g., Content-Type).
Extension<T> (older, replaced by State in modern Axum).
```


<!--------------------------------------------------------------------------------- Handler -->
<br><br>

## Handler

Init
```bash
```



<!--------------------------------------------------------------------------------- Service -->
<br><br>

## Service

Init
```bash
```











<!--------------------------------------------------------------------------------- Model -->
<br><br>

## Model

Init
```bash
```



<!--------------------------------------------------------------------------------- Links -->
[crates]: https://crates.io/crates/axum
[docs]: https://docs.rs/axum/0.8.4/axum/
[github]: https://github.com/tokio-rs/axum