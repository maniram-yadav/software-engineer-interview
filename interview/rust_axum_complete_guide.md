# The Complete Rust + Axum Guide
### Interview Questions with Detailed Answers + Full Theory + Inner Architecture + Complete Tutorial

---

## Table of Contents

**Part A — Interview Questions**
1. [Axum Fundamentals](#1-axum-fundamentals)
2. [Routing & Handlers](#2-routing--handlers)
3. [Extractors](#3-extractors)
4. [Application State](#4-application-state)
5. [Middleware & Tower Layers](#5-middleware--tower-layers)
6. [Error Handling](#6-error-handling)
7. [Database Connections & Pooling](#7-database-connections--pooling)
8. [ORM Integration (SQLx, SeaORM, Diesel)](#8-orm-integration-sqlx-seaorm-diesel)
9. [Authentication & Security](#9-authentication--security)
10. [WebSockets, SSE & Streaming](#10-websockets-sse--streaming)
11. [File Uploads & Multipart](#11-file-uploads--multipart)
12. [Testing Axum Applications](#12-testing-axum-applications)
13. [Deployment & Production](#13-deployment--production)
14. [Advanced / Architecture Questions](#14-advanced--architecture-questions)

**Part B — Complete Theory**
15. [Axum Theoretical Deep Dive & Inner Architecture](#15-axum-theoretical-deep-dive--inner-architecture)

**Part C — Full Tutorial**
16. [Complete Tutorial: Building a Production-Style Task Manager API](#16-complete-tutorial-building-a-production-style-task-manager-api)

---

# Part A — Interview Questions

## 1. Axum Fundamentals

### Q1. What is Axum, and how does it fit into the Rust web ecosystem?
Axum is an ergonomic, modular web framework built by the Tokio team on top of three foundational crates:
- **Tokio** — the async runtime (executor, timers, async I/O).
- **Hyper** — the low-level HTTP/1.1 and HTTP/2 implementation.
- **Tower** — a library of reusable, composable middleware abstractions (`Service` and `Layer` traits).

Axum's pitch is that it adds **no new middleware system** — it reuses Tower's `Service`/`Layer` traits directly, meaning any middleware written for Tower (or `tower-http`) works with Axum out of the box, and Axum handlers themselves compile down to Tower `Service`s. This is fundamentally different from frameworks like Actix-web, which have their own bespoke middleware/service abstractions.

### Q2. How does Axum compare to other Rust web frameworks (Actix-web, Rocket, Warp)?
```
Actix-web:  Own actor-based runtime historically, own middleware system, extremely mature, highest raw throughput in some benchmarks
Rocket:     Very ergonomic macro-driven DX, historically slower to adopt async, own request guard system
Warp:       Also Tower/Hyper based, but uses Filter combinators (composable but can produce hard-to-read type errors)
Axum:       Tower/Hyper/Tokio based, extractor + handler model (like FastAPI's function-signature style), no macros required for routing, excellent type-checked ergonomics, backed/maintained by the Tokio team itself
```
Axum's key differentiators: no proc-macros required for basic routing (routes are plain functions), extractors validate/parse the request via the function signature (similar mental model to FastAPI's dependency injection), and full interoperability with the broader Tower/Hyper ecosystem (rate limiting, tracing, compression, retries — all reusable `tower` crates).

### Q3. What is the minimal Axum application, and how do you run it?
```rust
// Cargo.toml
// [dependencies]
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
```
`#[tokio::main]` expands into code that creates a multi-threaded Tokio runtime and blocks on the async `main` body. `axum::serve` (0.7+) binds a `Router` (which is itself a Tower `Service`) to a `TcpListener` and drives the accept loop.

### Q4. What is Tokio, and why does Axum require an async runtime?
Rust's standard library deliberately ships **no async runtime** — `async fn` and `.await` are language/compiler features, but something has to actually poll the resulting `Future`s to completion, manage a task scheduler, and provide async I/O primitives (non-blocking sockets, timers). **Tokio** provides all of this: a work-stealing multi-threaded task scheduler, async TCP/UDP, timers, channels, and synchronization primitives. Axum, Hyper, and virtually the entire async Rust web ecosystem are built specifically on top of Tokio (as opposed to alternative runtimes like `async-std` or `smol`).

### Q5. What is the `Service` trait, and why is it central to Axum's design?
```rust
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}
```
Tower's `Service` trait is an abstraction over "something that takes a request and asynchronously produces a response" — it's generic enough to represent an HTTP handler, a database connection, a load balancer, or a middleware layer. Axum handlers are converted into `Service<Request>` implementations under the hood via the `Handler` trait, and a `Router` itself implements `Service`, which is precisely why routers can be nested, merged, and wrapped in middleware uniformly.

### Q6. What is the `Layer` trait, and how does it relate to middleware?
```rust
pub trait Layer<S> {
    type Service;
    fn layer(&self, inner: S) -> Self::Service;
}
```
A `Layer` wraps an existing `Service` to produce a new `Service` — this is the middleware composition pattern in Tower. Applying `.layer(SomeLayer)` to a `Router` wraps every route's underlying `Service` with `SomeLayer`'s behavior (e.g., logging before/after calling the inner service). Middleware stacks are just nested `Service`s, each decorating the one beneath it — conceptually identical to the decorator pattern.

---

## 2. Routing & Handlers

### Q7. How do you define routes for different HTTP methods and path parameters?
```rust
use axum::{
    routing::{get, post, put, delete},
    extract::Path,
    Router,
};

async fn list_items() -> &'static str { "list" }
async fn get_item(Path(id): Path<u32>) -> String { format!("item {id}") }
async fn create_item() -> &'static str { "created" }
async fn update_item(Path(id): Path<u32>) -> String { format!("updated {id}") }
async fn delete_item(Path(id): Path<u32>) -> String { format!("deleted {id}") }

fn routes() -> Router {
    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/{id}", get(get_item).put(update_item).delete(delete_item))
}
```
Chaining `.get().post()` etc. on the same `.route()` call registers multiple methods for the same path without duplicating the path string. (Axum 0.7 uses `{id}` path-param syntax; pre-0.7 used `:id`.)

### Q8. What is a "handler" in Axum, and what makes a function eligible to be one?
```rust
async fn handler_a() -> &'static str { "ok" }                      // valid: no args, returns something IntoResponse
async fn handler_b(Path(id): Path<u32>) -> String { id.to_string() } // valid: extractors as args
async fn handler_c(body: String) -> String { body }                 // valid: String implements FromRequest
```
A handler is any `async fn` whose:
- **Arguments** all implement `FromRequestParts` (for non-body extractors like `Path`, `Query`, headers) or the *last* argument implements `FromRequest` (consumes the body — e.g., `Json<T>`, `String`, `Bytes`).
- **Return type** implements `IntoResponse`.

Axum implements the `Handler` trait for functions up to a fixed arity (currently 16 arguments) via a macro, which is why the compiler can validate your route signatures entirely at compile time — a mismatched extractor or a non-`IntoResponse` return type is a compile error, not a runtime surprise.

### Q9. How do you organize routes across modules and nest/merge routers?
```rust
// routes/items.rs
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_items))
        .route("/{id}", get(get_item))
}

// routes/users.rs
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_users))
}

// main.rs
let app = Router::new()
    .nest("/items", items::router())
    .nest("/users", users::router())
    .merge(health::router());       // merge = combine at the SAME path level, no prefix
```
`.nest("/prefix", router)` mounts a sub-router under a path prefix (analogous to FastAPI's `APIRouter(prefix=...)` / Express's sub-routers). `.merge()` combines two routers' routes into one without adding a prefix — useful for splitting a flat route list across files.

### Q10. How do you define fallback handlers (404s) and route-specific error pages?
```rust
async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

let app = Router::new()
    .route("/", get(root))
    .fallback(fallback);
```
`.fallback()` registers a handler invoked when no route matches — without it, Axum returns a bare `404` with an empty body. `.method_not_allowed_fallback()` similarly customizes the `405` response when a path matches but the HTTP method doesn't.

### Q11. How do route matching precedence and wildcards work?
```rust
Router::new()
    .route("/users/{id}", get(get_user))          // matches /users/42
    .route("/users/me", get(get_current_user))     // static segments take precedence over params
    .route("/files/{*path}", get(serve_file));      // {*path} = catch-all wildcard, matches /files/a/b/c
```
Axum's router (built on `matchit`, a radix-tree-based matcher) prioritizes more specific/static segments over dynamic ones, so `/users/me` is matched before falling through to `/users/{id}` regardless of declaration order — this is different from frameworks that match top-to-bottom.

### Q12. What return types can a handler produce, and how does `IntoResponse` work?
```rust
use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;

#[derive(Serialize)]
struct Item { id: u32, name: String }

async fn plain() -> &'static str { "text" }                              // 200, text/plain
async fn json_resp() -> Json<Item> { Json(Item { id: 1, name: "x".into() }) } // 200, application/json
async fn status_and_body() -> (StatusCode, Json<Item>) {                  // custom status + body
    (StatusCode::CREATED, Json(Item { id: 1, name: "x".into() }))
}
async fn custom() -> Response {                                           // full manual control
    Response::builder()
        .status(StatusCode::IM_A_TEAPOT)
        .body("I'm a teapot".into())
        .unwrap()
}
```
`IntoResponse` is implemented for tuples of `(StatusCode, T)`, `(HeaderMap, T)`, `Json<T>` (where `T: Serialize`), `String`, `&str`, `StatusCode` alone, `Result<T, E>` (where both `T` and `E` implement `IntoResponse`), and more — this trait-based conversion is what lets handlers return ergonomic, varied types while Axum uniformly converts them into an HTTP response.

---

## 3. Extractors

### Q13. What is an "extractor," and what's the mental model for using them?
Extractors are types implementing `FromRequestParts` (headers/metadata only) or `FromRequest` (can consume the body) that pull typed data out of an incoming request directly into your handler's function parameters — Axum's equivalent of FastAPI's dependency-injected parameters. The framework parses/validates the request *before* your handler body ever runs; if extraction fails, the extractor's `Rejection` type is converted straight into an error response without your handler code executing at all.

```rust
async fn handler(
    Path(id): Path<u32>,           // from the URL path
    Query(params): Query<Params>,   // from ?query=string
    headers: HeaderMap,               // request headers
    Json(body): Json<CreateItem>,    // request body as JSON (must be LAST — consumes the body)
) -> impl IntoResponse { ... }
```

### Q14. What is the critical ordering rule for extractors, and why does it exist?
```rust
// COMPILE ERROR: only the LAST extractor may consume the body
async fn bad(Json(a): Json<A>, Json(b): Json<B>) { }

// CORRECT: body-consuming extractor must be last
async fn good(Path(id): Path<u32>, Json(body): Json<CreateItem>) { }
```
The HTTP request body is a single-consumption async stream — only one extractor can read it, and it must be the *last* parameter. All extractors before it may only inspect parts (`FromRequestParts` — path, query, headers, extensions, state) without touching the body. Axum enforces this at compile time via its blanket trait implementations, so getting the order wrong fails to compile rather than panicking at runtime.

### Q15. How do `Path` and `Query` extraction work, including multiple path params?
```rust
use axum::extract::{Path, Query};
use serde::Deserialize;

// Single path param
async fn get_item(Path(id): Path<u32>) -> String { id.to_string() }

// Multiple path params via a tuple
async fn get_comment(Path((post_id, comment_id)): Path<(u32, u32)>) -> String {
    format!("{post_id}/{comment_id}")
}

// Query params deserialized into a struct via serde
#[derive(Deserialize)]
struct Pagination { page: Option<u32>, limit: Option<u32> }

async fn list(Query(p): Query<Pagination>) -> String {
    format!("page={:?} limit={:?}", p.page, p.limit)
}
```
Both rely on `serde` for deserialization — `Path` deserializes from route segments, `Query` from the URL-encoded query string (via `serde_urlencoded` internally). Failed extraction (wrong type, missing required field) yields an automatic `400 Bad Request` rejection.

### Q16. How does the `Json` extractor work, and what happens on invalid input?
```rust
use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser { username: String, email: String }

#[derive(Serialize)]
struct UserOut { id: u32, username: String }

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<UserOut>) {
    let user = UserOut { id: 1, username: payload.username };
    (StatusCode::CREATED, Json(user))
}
```
`Json<T>` requires `T: DeserializeOwned` for extraction and `T: Serialize` when used as a response wrapper. It checks the `Content-Type: application/json` header and deserializes via `serde_json`; malformed JSON or a type mismatch automatically produces a `400 Bad Request` with a descriptive error body — no manual validation boilerplate, directly analogous to FastAPI's Pydantic-driven body parsing.

### Q17. How do you write a custom extractor?
```rust
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    async_trait,
};

struct ApiKey(String);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ApiKey {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .map(|s| ApiKey(s.to_owned()))
            .ok_or((StatusCode::UNAUTHORIZED, "Missing X-API-Key header"))
    }
}

async fn protected_handler(ApiKey(key): ApiKey) -> String {
    format!("authenticated with key: {key}")
}
```
Custom extractors are Axum's primary extension point for cross-cutting request logic (auth, request IDs, tenant resolution) — implement `FromRequestParts` (headers/metadata only, can run before other extractors) or `FromRequest` (needs body access, must be the terminal extractor) and Axum wires it in automatically wherever the type appears in a handler signature.

### Q18. What are `Extension`, `State`, and when should you use each?
```rust
use axum::{extract::{State, Extension}, Router};

#[derive(Clone)]
struct AppState { db_pool: sqlx::PgPool }

async fn handler_with_state(State(state): State<AppState>) -> String {
    format!("pool size: {}", state.db_pool.size())
}

async fn handler_with_extension(Extension(req_id): Extension<RequestId>) -> String {
    req_id.0
}

let app = Router::new()
    .route("/", get(handler_with_state))
    .with_state(AppState { db_pool });
```
`State<T>` is the **preferred** mechanism for typed, compile-time-checked application state (DB pools, config) — injected once via `.with_state()`, and Axum verifies at compile time that the state type matches. `Extension<T>` is a more dynamic, type-map-based mechanism (originally the only option pre-0.6) typically reserved for values inserted per-request by middleware (e.g., a request ID or an authenticated user attached by an auth layer) rather than global app state.

### Q19. How do you make an extractor optional or capture extraction failures explicitly?
```rust
use axum::extract::{Query, Path};

// Option<T> - None if extraction fails instead of short-circuiting with an error response
async fn handler(Query(params): Query<Option<MyParams>>) { }

// Result<T, T::Rejection> - inspect the rejection yourself
async fn handler2(result: Result<Path<u32>, axum::extract::rejection::PathRejection>) {
    match result {
        Ok(Path(id)) => { /* ... */ }
        Err(rejection) => { /* custom handling */ }
    }
}
```
Wrapping an extractor in `Option<T>` or `Result<T, Rejection>` opts out of the automatic rejection-to-error-response behavior, letting the handler itself decide what to do when extraction fails — useful for optional query parameters or custom error formatting.

---

## 4. Application State

### Q20. What's the idiomatic pattern for sharing a database pool / config across handlers?
```rust
use axum::{extract::State, Router};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    config: Arc<Config>,   // Arc for cheap-clone immutable shared data
}

#[tokio::main]
async fn main() {
    let db = sqlx::PgPool::connect("postgres://...").await.unwrap();
    let state = AppState { db, config: Arc::new(Config::load()) };

    let app = Router::new()
        .route("/items", get(list_items))
        .with_state(state);   // state is cloned (cheaply) per request internally
}

async fn list_items(State(state): State<AppState>) -> impl IntoResponse {
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(items)
}
```
`Router<S>` is generic over a state type `S`; `.with_state(s)` finalizes it into `Router<()>` (or rather removes the state parameter), ready to be served. State must implement `Clone` — for a `PgPool` this is cheap because it internally wraps an `Arc`-based connection pool, so cloning just bumps a reference count rather than duplicating connections. This is the same pattern used for any shared resource: HTTP clients, Redis connections, in-memory caches.

### Q21. How do you compose state from multiple sub-modules that each need only part of the state (`FromRef`)?
```rust
use axum::extract::{FromRef, State};

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    redis: deadpool_redis::Pool,
}

// Allows extracting JUST the PgPool via State<PgPool> even though AppState is the router's state
impl FromRef<AppState> for sqlx::PgPool {
    fn from_ref(state: &AppState) -> Self { state.db.clone() }
}

async fn handler(State(db): State<sqlx::PgPool>) -> impl IntoResponse {
    // handler only depends on the DB pool, not the whole AppState -> more testable, decoupled
}
```
`FromRef` lets a large composite `AppState` be decomposed so individual handlers/modules only declare a dependency on the specific slice of state they actually need (analogous to narrowing a dependency injection scope) — improves testability and avoids handlers being coupled to fields they never touch.

### Q22. When should you use `Arc<Mutex<T>>` vs a connection pool vs `RwLock` for shared mutable state?
```rust
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::Mutex as TokioMutex;

// In-memory counter, low contention, short critical sections -> std Mutex is fine
struct AppState { counter: Arc<Mutex<u64>> }

// Frequent reads, rare writes (e.g., cached config) -> RwLock allows concurrent readers
struct AppState2 { cache: Arc<RwLock<HashMap<String, String>>> }

// If you must hold the lock ACROSS an .await point -> use tokio::sync::Mutex, not std::sync::Mutex
struct AppState3 { session: Arc<TokioMutex<SomeAsyncResource>> }
```
- **`std::sync::Mutex`/`RwLock`**: fine for short, synchronous critical sections (never held across an `.await`) — cheaper than the async variants.
- **`tokio::sync::Mutex`**: required if the lock must be held while awaiting (e.g., an in-progress async operation) — holding a `std::sync::Mutex` guard across `.await` risks blocking the executor thread and is flagged by clippy (`await_holding_lock`).
- **Connection pools (`sqlx::PgPool`, `deadpool`)**: the correct choice for database/external-resource access — they manage a bounded set of connections with internal async-aware locking, rather than you hand-rolling synchronization around a single shared connection.

---

## 5. Middleware & Tower Layers

### Q23. What are the different ways to write middleware in Axum?
```rust
// 1. axum::middleware::from_fn - simplest, function-based middleware
use axum::{middleware::{self, Next}, extract::Request, response::Response};

async fn log_middleware(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let start = std::time::Instant::now();
    let response = next.run(req).await;      // call the rest of the stack
    println!("{path} took {:?}", start.elapsed());
    response
}

let app = Router::new()
    .route("/", get(root))
    .layer(middleware::from_fn(log_middleware));

// 2. tower::Layer / tower::Service - full custom Tower middleware (more boilerplate, most flexible)
// 3. tower-http prebuilt layers - TraceLayer, CorsLayer, CompressionLayer, TimeoutLayer, etc.
```
`from_fn` is the most common entry point for custom middleware — it wraps an async function matching a specific signature (`Request` in, calls `next.run()`, returns a `Response`), sidestepping the need to hand-implement the `Service`/`Layer` traits directly for simple cases.

### Q24. How do you apply middleware from `tower-http` (CORS, tracing, compression, timeouts)?
```rust
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
};
use std::time::Duration;
use axum::http::Method;

let app = Router::new()
    .route("/", get(root))
    .layer(TraceLayer::new_for_http())                    // structured request/response logging via `tracing`
    .layer(CompressionLayer::new())                          // gzip/br response compression
    .layer(TimeoutLayer::new(Duration::from_secs(10)))         // abort requests exceeding the timeout
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST]),
    );
```
`tower-http` provides a large library of production-grade, battle-tested middleware as composable `Layer`s — reusable across any Tower-based framework, not just Axum. Order matters: layers are applied outermost-to-innermost in the order `.layer()` is called relative to route registration (later `.layer()` calls wrap *outside* earlier ones when reading top-to-bottom in the typical builder chain).

### Q25. How does per-route vs global middleware application work, and how do you scope middleware to only some routes?
```rust
let public_routes = Router::new().route("/login", post(login));

let protected_routes = Router::new()
    .route("/profile", get(profile))
    .route_layer(middleware::from_fn(require_auth));   // applies ONLY to routes registered so far in THIS router

let app = Router::new()
    .merge(public_routes)
    .merge(protected_routes);
```
`.route_layer()` (as opposed to `.layer()`) applies middleware only to the routes already registered in the router at the point it's called — and, critically, does *not* run for unmatched routes (so it won't interfere with 404 handling), unlike `.layer()` which wraps the whole router including its fallback.

### Q26. How do you attach per-request data (like a request ID or authenticated user) via middleware so downstream handlers can extract it?
```rust
use axum::{Extension, middleware::Next, extract::Request, response::Response};
use uuid::Uuid;

#[derive(Clone)]
struct RequestId(String);

async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let id = RequestId(Uuid::new_v4().to_string());
    req.extensions_mut().insert(id.clone());
    let mut response = next.run(req).await;
    response.headers_mut().insert("x-request-id", id.0.parse().unwrap());
    response
}

// any downstream handler:
async fn handler(Extension(req_id): Extension<RequestId>) -> String {
    req_id.0
}
```
Middleware inserts arbitrary typed values into the request's `Extensions` type-map; any handler (or further middleware) downstream extracts it via `Extension<T>`. This is the standard mechanism for auth middleware to attach an "authenticated user" struct that route handlers can then simply declare as a parameter.

### Q27. What is `Next`, and what can middleware do before/after calling it?
```rust
async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    // BEFORE: inspect/reject the request before it reaches the handler
    let token = req.headers().get("authorization").ok_or(StatusCode::UNAUTHORIZED)?;
    validate_token(token)?;

    let response = next.run(req).await;   // hands off to the rest of the middleware stack + handler

    // AFTER: inspect/modify the response before it goes to the client
    Ok(response)
}
```
`Next` represents "the rest of the request-handling pipeline" — calling `next.run(req).await` invokes whatever is next in the stack (more middleware, then eventually the matched handler) and yields the resulting `Response`, which the current middleware can further inspect or modify. Not calling `next.run()` at all short-circuits the request entirely (useful for auth rejection, rate limiting, or serving a cached response).

---

## 6. Error Handling

### Q28. What is the idiomatic pattern for handler error handling using a custom `AppError` + `IntoResponse`?
```rust
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

enum AppError {
    NotFound,
    Validation(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(err) => {
                tracing::error!(%err, "internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

// handlers now return Result<T, AppError> and use `?` freely
async fn get_item(Path(id): Path<u32>, State(state): State<AppState>) -> Result<Json<Item>, AppError> {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1")
        .bind(id as i32)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or(AppError::NotFound)?;
    Ok(Json(item))
}
```
This is the standard Axum error-handling pattern: define one enum representing every error your handlers can produce, implement `IntoResponse` for it once, and every handler's return type becomes `Result<T, AppError>` — letting `?` propagate errors naturally while guaranteeing consistent, centrally controlled error response formatting (directly analogous to FastAPI's global exception handlers, but resolved at compile time via the type system instead of a runtime registry).

### Q29. How do you convert errors from external crates (e.g., `sqlx::Error`, `anyhow::Error`) into your `AppError` ergonomically?
```rust
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::Internal(other.into()),
        }
    }
}

// now `?` auto-converts sqlx::Error -> AppError without .map_err()
async fn get_item(Path(id): Path<u32>, State(state): State<AppState>) -> Result<Json<Item>, AppError> {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1")
        .bind(id as i32)
        .fetch_one(&state.db)
        .await?;   // sqlx::Error -> AppError via From, thanks to `?`
    Ok(Json(item))
}
```
Implementing `From<SourceError> for AppError` for every error type your handlers encounter lets `?` perform the conversion automatically (Rust's `?` operator calls `.into()` on the error under the hood) — this is idiomatic Rust error handling (the same pattern used outside web contexts) applied directly to Axum handlers, and libraries like `thiserror` are commonly used to reduce the boilerplate of writing these `From` impls and `Display` messages.

### Q30. How do you handle panics inside handlers so they don't crash the whole server?
```rust
use tower_http::catch_panic::CatchPanicLayer;

let app = Router::new()
    .route("/", get(root))
    .layer(CatchPanicLayer::new());
```
By default, Hyper/Tokio isolate a panic inside one request-handling task — it does not crash the whole server process, but it *does* abort that one connection with no response sent, which looks like a dropped connection to the client. `tower-http`'s `CatchPanicLayer` catches panics and converts them into a proper `500 Internal Server Error` response instead, which is standard practice for production services.

### Q31. How do you validate request bodies with structured, field-level error messages (like Pydantic's 422 responses)?
```rust
use validator::Validate;
use axum::{extract::rejection::JsonRejection, Json};

#[derive(serde::Deserialize, Validate)]
struct CreateUser {
    #[validate(length(min = 3, max = 20))]
    username: String,
    #[validate(email)]
    email: String,
}

async fn create_user(payload: Result<Json<CreateUser>, JsonRejection>) -> Result<Json<UserOut>, AppError> {
    let Json(payload) = payload.map_err(|e| AppError::Validation(e.to_string()))?;
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    // ... proceed
    Ok(Json(UserOut { /* ... */ }))
}
```
Unlike Pydantic, `serde` alone only validates *shape/types*, not business constraints (min length, email format, ranges) — the `validator` crate (via `#[derive(Validate)]` and field attributes) is the common addition for declarative field-level validation, producing a structured `ValidationErrors` you map into your `AppError`.

---

## 7. Database Connections & Pooling

### Q32. What are the main options for talking to a database from Axum, and how do you choose?
```
sqlx      - async, compile-time checked raw SQL, no ORM abstraction, supports Postgres/MySQL/SQLite
SeaORM    - full async ORM built ON TOP of sqlx, ActiveRecord-style models, migrations, relations
Diesel    - mature, powerful query builder/ORM, historically SYNC (diesel-async now exists for async use)
tokio-postgres - lower-level async Postgres driver (sqlx/deadpool-postgres build on similar foundations)
```
`sqlx` is by far the most common choice in the Axum ecosystem because it's async-native (built on Tokio), has zero runtime ORM overhead, and its standout feature — **compile-time verified SQL** — catches typos and type mismatches in your queries at `cargo build` time (via macros that connect to a real database or an offline query cache during compilation). Teams wanting a fuller ORM experience (migrations, model relations, ActiveRecord patterns similar to Django/Rails) typically reach for **SeaORM**, which itself uses `sqlx` under the hood.

### Q33. How do you set up an `sqlx` connection pool and wire it into Axum's state?
```rust
use sqlx::postgres::{PgPoolOptions, PgPool};

async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(20)              // upper bound on concurrent DB connections
        .min_connections(5)                // keep-warm floor, avoids cold-start latency spikes
        .acquire_timeout(std::time::Duration::from_secs(5))   // fail fast instead of hanging forever
        .idle_timeout(std::time::Duration::from_secs(600))     // recycle idle connections
        .connect(database_url)
        .await
        .expect("failed to connect to Postgres")
}

#[tokio::main]
async fn main() {
    let pool = create_pool(&std::env::var("DATABASE_URL").unwrap()).await;
    let app = Router::new().route("/health", get(health)).with_state(pool);
}

async fn health(State(pool): State<PgPool>) -> &'static str {
    sqlx::query("SELECT 1").execute(&pool).await.unwrap();
    "ok"
}
```
`PgPool` is a cheap-to-`Clone` handle (internally `Arc`-wrapped) around a managed set of connections — you create it **once** at startup and share it via `State`, never opening a fresh connection per request. Key pool-sizing considerations: `max_connections` should generally correlate with `(Postgres max_connections / number of app instances)`, not be set arbitrarily high (each connection has real memory/resource cost on the DB server); `acquire_timeout` prevents a request from hanging indefinitely if the pool is exhausted, converting it into a fast, visible error instead.

### Q34. What is `sqlx`'s compile-time query checking, and how does the offline mode work?
```rust
// Requires DATABASE_URL to be set (or a cached .sqlx/ directory) at COMPILE time
let row = sqlx::query_as!(
    Item,
    "SELECT id, name, price FROM items WHERE id = $1",
    item_id
)
.fetch_one(&pool)
.await?;
```
```bash
# Generate an offline query cache so CI/other devs don't need a live DB to compile:
cargo install sqlx-cli
cargo sqlx prepare        # writes .sqlx/*.json capturing each query's verified shape
# committed to git; `cargo build` then uses SQLX_OFFLINE=true to check against the cache instead of a live DB
```
The `query!`/`query_as!` macros connect to an actual database schema at compile time (or read a cached `.sqlx` directory) to verify that your SQL is syntactically valid *and* that the columns you're selecting match the Rust struct's field names/types — a class of bugs (typo'd column names, type mismatches between DB and Rust) is caught before the code ever runs, which is a distinctive Rust-ecosystem feature with no real equivalent in most dynamically-typed ORMs.

### Q35. How do you run multiple queries in a transaction with `sqlx`?
```rust
async fn transfer_funds(pool: &PgPool, from: i32, to: i32, amount: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("UPDATE accounts SET balance = balance - $1 WHERE id = $2")
        .bind(amount).bind(from)
        .execute(&mut *tx)
        .await?;

    sqlx::query("UPDATE accounts SET balance = balance + $1 WHERE id = $2")
        .bind(amount).bind(to)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;   // if this line is never reached (error propagated via `?`),
                            // `tx` is dropped and the transaction is automatically ROLLED BACK
    Ok(())
}
```
`sqlx`'s `Transaction` leverages Rust's RAII/`Drop` semantics: if you return early via `?` (an error) without calling `.commit()`, the transaction guard is dropped and automatically issues a `ROLLBACK` — you cannot forget to roll back on an error path, unlike manual try/catch/rollback patterns in other languages.

### Q36. What connection pooling strategies exist for non-SQL datastores (Redis, MongoDB) in Axum?
```rust
// Redis via deadpool-redis
use deadpool_redis::{Config, Runtime, Pool as RedisPool};

let cfg = Config::from_url("redis://127.0.0.1/");
let redis_pool: RedisPool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

// MongoDB - the official driver has its OWN internal connection pooling; you just clone the Client
use mongodb::Client;
let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
```
`deadpool` is a generic async pooling library used for Redis and other resources that don't ship their own pool (`deadpool-redis`, `deadpool-postgres`). MongoDB's official Rust driver manages its own internal connection pool per `Client` — you create one `Client` at startup, clone it (cheap, `Arc`-backed) into `State` just like an `sqlx::PgPool`, and never construct it per-request.

### Q37. How should database errors surface to API consumers without leaking internal details?
```rust
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Validation("resource already exists".into())
            }
            _ => {
                tracing::error!(error = ?err, "database error");   // full detail logged server-side
                AppError::Internal(err.into())                        // generic message sent to client
            }
        }
    }
}
```
Map specific, expected DB error conditions (not found, unique constraint violations, foreign key violations) to precise HTTP semantics (`404`, `409 Conflict`), while ensuring unexpected DB errors are logged in full detail server-side but returned to the client only as a generic `500` — never leak raw SQL error strings, table names, or query text to API consumers (an information-disclosure risk).

---

## 8. ORM Integration (SQLx, SeaORM, Diesel)

### Q38. How do you define and query entities with SeaORM inside Axum?
```rust
// entity/item.rs - generated via `sea-orm-cli generate entity` from an existing DB, or hand-written
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub price: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// handler
use sea_orm::{DatabaseConnection, EntityTrait};

async fn get_item(
    Path(id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<item::Model>, AppError> {
    let item = item::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(item))
}
```
SeaORM's `DatabaseConnection` (itself pool-backed) is shared via `State` exactly like a raw `sqlx::PgPool`. Entities are defined declaratively (either hand-written or generated from an existing schema via `sea-orm-cli`), and querying uses a fluent, ActiveRecord-adjacent builder API (`Entity::find()`, `.filter()`, `.one()`/`.all()`) rather than raw SQL strings.

### Q39. How do SeaORM migrations work, and how do you run them alongside an Axum app?
```rust
// migration/src/m20240101_000001_create_items.rs
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create().table(Items::Table).if_not_exists()
                .col(ColumnDef::new(Items::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Items::Name).string().not_null())
                .to_owned()
        ).await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Items::Table).to_owned()).await
    }
}
```
```bash
sea-orm-cli migrate up      # applies pending migrations
sea-orm-cli migrate down    # rolls back the last migration
```
SeaORM migrations are written as Rust structs implementing `up`/`down`, compiled into a separate `migration` crate/binary — analogous to Alembic (Python) or Rails migrations, but type-checked at compile time like the rest of the Rust codebase. For raw `sqlx` projects without an ORM, `sqlx-cli`'s simpler SQL-file-based migrations (`sqlx migrate add`, `sqlx migrate run`) are the standard alternative.

### Q40. How does Diesel differ from sqlx/SeaORM, and what does `diesel-async` add?
```rust
// Diesel: schema defined via macros (often generated from the DB by `diesel print-schema`)
table! {
    items (id) {
        id -> Int4,
        name -> Text,
        price -> Numeric,
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = items)]
struct Item { id: i32, name: String, price: bigdecimal::BigDecimal }

// diesel-async required to use Diesel from an async Axum handler without blocking the runtime
use diesel_async::{RunQueryDsl, AsyncPgConnection, pooled_connection::deadpool::Pool};

async fn get_item(State(pool): State<Pool<AsyncPgConnection>>, Path(id): Path<i32>) -> Result<Json<Item>, AppError> {
    let mut conn = pool.get().await?;
    let item = items::table.find(id).select(Item::as_select()).first(&mut conn).await?;
    Ok(Json(item))
}
```
Diesel's query builder is famous for being extremely type-safe (the schema's Rust types are baked into every query at compile time, catching mismatched joins/columns as compiler errors) and historically was **synchronous only** — using it from Axum required offloading calls to a blocking thread pool (`tokio::task::spawn_blocking`). `diesel-async` (a newer, actively maintained addition) provides genuinely async connections/pools so Diesel can be used directly from `async fn` handlers without the blocking-thread-pool indirection, closing the gap with sqlx/SeaORM for async-native codebases.

### Q41. What's the recommended repository/service-layer pattern for structuring DB access in an Axum app?
```rust
// repository.rs - isolates ALL sql/orm calls behind a trait
#[async_trait::async_trait]
pub trait ItemRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<Item>, sqlx::Error>;
    async fn create(&self, item: NewItem) -> Result<Item, sqlx::Error>;
}

pub struct PgItemRepository { pool: PgPool }

#[async_trait::async_trait]
impl ItemRepository for PgItemRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<Item>, sqlx::Error> {
        sqlx::query_as!(Item, "SELECT * FROM items WHERE id = $1", id).fetch_optional(&self.pool).await
    }
    async fn create(&self, item: NewItem) -> Result<Item, sqlx::Error> {
        sqlx::query_as!(Item, "INSERT INTO items (name) VALUES ($1) RETURNING *", item.name)
            .fetch_one(&self.pool).await
    }
}

// handlers depend on `Arc<dyn ItemRepository>` in AppState -> trivially mockable in tests
```
Wrapping the ORM/SQL layer behind a trait decouples handlers from the concrete database technology and — critically — makes handlers unit-testable with an in-memory fake implementation of the trait, without spinning up a real database. This mirrors the "repository pattern" common across many stacks (and is the Rust-idiomatic equivalent of FastAPI's dependency-override-based DB mocking, achieved here via trait objects instead of runtime DI).

---

## 9. Authentication & Security

### Q42. How do you implement JWT-based authentication in Axum?
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Claims { sub: String, exp: usize }

fn create_token(user_id: &str, secret: &[u8]) -> String {
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap()
}

// Custom extractor that validates the Authorization: Bearer <token> header
struct AuthUser(String);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization").ok_or(StatusCode::UNAUTHORIZED)?;
        let token = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?
            .strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;

        let secret = std::env::var("JWT_SECRET").unwrap();
        let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser(data.claims.sub))
    }
}

async fn protected(AuthUser(user_id): AuthUser) -> String {
    format!("Hello, user {user_id}")
}
```
Implementing auth as a **custom extractor** (rather than middleware) is a common and idiomatic Axum pattern: the handler simply declares `AuthUser` as a parameter, and Axum guarantees it's populated (validated) or the request never reaches the handler body at all — the type system documents which routes require authentication. Middleware (`from_fn`) is the alternative when you need auth logic to run before other extractors or to attach data via `Extension` for multiple downstream consumers.

### Q43. How do you hash and verify passwords securely in Rust?
```rust
use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use argon2::password_hash::{SaltString, rand_core::OsRng};

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
```
**Argon2** (winner of the Password Hashing Competition) is the current recommended default for the Rust ecosystem (`argon2` crate), analogous to `bcrypt`/`argon2` in Python — deliberately slow and memory-hard to resist brute-force and GPU/ASIC cracking. Never store plaintext passwords or use general-purpose fast hashes (SHA-256/MD5) for credentials.

### Q44. How do you implement session-based (cookie) authentication instead of JWTs?
```rust
use axum_extra::extract::cookie::{CookieJar, Cookie};
use tower_sessions::{Session, SessionManagerLayer, MemoryStore};   // or a Redis-backed store in production

async fn login(jar: CookieJar, /* ... */) -> (CookieJar, &'static str) {
    let jar = jar.add(Cookie::new("session_id", "abc123").http_only(true).secure(true));
    (jar, "logged in")
}

// tower-sessions layer for server-side session state (recommended over hand-rolled cookies)
let session_store = MemoryStore::default();   // swap for tower-sessions-sqlx-store / redis-store in prod
let app = Router::new()
    .route("/login", post(login))
    .layer(SessionManagerLayer::new(session_store));

async fn handler(session: Session) -> String {
    session.insert("user_id", 42).await.unwrap();
    let user_id: Option<i32> = session.get("user_id").await.unwrap();
    format!("{user_id:?}")
}
```
`tower-sessions` (a Tower-ecosystem crate) provides server-side session storage with pluggable backends (in-memory for dev, Redis/Postgres-backed for production/multi-instance deployments), issuing a session-ID cookie to the client while keeping the actual session data server-side — a common alternative to stateless JWTs when you need instant server-side session revocation or want to avoid embedding claims in a client-held token.

### Q45. What security middleware/headers should a production Axum app set, and what does `tower-http` provide?
```rust
use tower_http::{
    set_header::SetResponseHeaderLayer,
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
};
use axum::http::{header, HeaderValue};

let app = Router::new()
    .route("/", get(root))
    .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))   // cap request body size (DoS mitigation)
    .layer(SetResponseHeaderLayer::if_not_present(
        header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"),
    ))
    .layer(CorsLayer::new().allow_origin("https://myapp.com".parse::<HeaderValue>().unwrap()));
```
Beyond CORS, standard production hardening: cap request body size (`RequestBodyLimitLayer`) to prevent memory-exhaustion DoS from oversized uploads, set security headers (`X-Content-Type-Options`, `X-Frame-Options`, `Strict-Transport-Security` — often better handled at a reverse proxy like nginx/Caddy), rate-limit sensitive endpoints (`tower::limit::RateLimitLayer` or a Redis-backed custom layer for multi-instance deployments), and always terminate TLS in front of the app (Axum itself serves plain HTTP; TLS is typically handled by a load balancer, or via `axum-server`'s `rustls` support if termination must happen in-process).

---

## 10. WebSockets, SSE & Streaming

### Q46. How do you implement a WebSocket endpoint in Axum?
```rust
use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use axum::response::IntoResponse;

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            if socket.send(Message::Text(format!("echo: {text}"))).await.is_err() {
                break;   // client disconnected
            }
        }
    }
}

let app = Router::new().route("/ws", get(ws_handler));
```
`WebSocketUpgrade` is an extractor that handles the HTTP upgrade handshake; `.on_upgrade()` takes a closure that receives the live `WebSocket` (a bidirectional stream of `Message`s) once the upgrade completes. This is built directly on Hyper's upgrade mechanism and Tokio's async I/O — no separate WebSocket server/process needed.

### Q47. How do you broadcast messages to multiple connected WebSocket clients (a chat-room pattern)?
```rust
use tokio::sync::broadcast;
use axum::extract::State;

#[derive(Clone)]
struct AppState { tx: broadcast::Sender<String> }

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() { break; }
        }
    });

    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx.send(text);   // rebroadcast to all subscribers
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}
```
`tokio::sync::broadcast` is a multi-producer, multi-consumer channel ideal for fan-out pub/sub within a single process — each connected client subscribes with `.subscribe()`, and any message sent via `.send()` reaches every subscriber. `socket.split()` separates the WebSocket into independent send/receive halves so both directions can be driven concurrently via separate spawned tasks, coordinated with `tokio::select!` so either task finishing (e.g., on disconnect) cancels the other.

### Q48. How do you implement Server-Sent Events (SSE) for one-way streaming?
```rust
use axum::response::sse::{Event, Sse};
use futures::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| Event::default().data("tick"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
```
SSE is a simpler one-directional alternative to WebSockets (server → client only, over plain HTTP, auto-reconnecting on the browser's `EventSource` API) — well suited for live dashboards, progress updates, or notification feeds where the client never needs to send data back over the same connection.

---

## 11. File Uploads & Multipart

### Q49. How do you handle `multipart/form-data` file uploads?
```rust
use axum::extract::Multipart;

async fn upload(mut multipart: Multipart) -> Result<String, AppError> {
    while let Some(field) = multipart.next_field().await.map_err(|_| AppError::Validation("bad multipart".into()))? {
        let name = field.name().unwrap_or("").to_string();
        let file_name = field.file_name().map(|s| s.to_string());
        let data = field.bytes().await.map_err(|_| AppError::Validation("read error".into()))?;

        if let Some(file_name) = file_name {
            tokio::fs::write(format!("uploads/{file_name}"), &data).await
                .map_err(|e| AppError::Internal(e.into()))?;
        }
        println!("field `{name}` = {} bytes", data.len());
    }
    Ok("uploaded".into())
}
```
`Multipart` streams fields one at a time via `.next_field()` rather than buffering the entire request in memory up front — important for large file uploads. Always pair this with `RequestBodyLimitLayer` (Q45) to cap the maximum accepted upload size, since an unbounded multipart body is a straightforward DoS vector.

### Q50. How do you stream a large file as a response without loading it entirely into memory?
```rust
use tokio_util::io::ReaderStream;
use axum::body::Body;

async fn download() -> Result<Response, AppError> {
    let file = tokio::fs::File::open("large_file.zip").await.map_err(|e| AppError::Internal(e.into()))?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Response::builder()
        .header("Content-Type", "application/octet-stream")
        .header("Content-Disposition", "attachment; filename=\"large_file.zip\"")
        .body(body)
        .map_err(|e| AppError::Internal(e.into()))
}
```
`tokio_util::io::ReaderStream` adapts any `AsyncRead` (a file, a network stream) into a `Stream` of byte chunks, which `Body::from_stream` turns into a streaming HTTP response body — Hyper sends chunks as they're read rather than requiring the full file in memory first, essential for serving large files efficiently.

---

## 12. Testing Axum Applications

### Q51. How do you write integration tests against an Axum router without starting a real TCP server?
```rust
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;   // for `.oneshot()`

#[tokio::test]
async fn test_health_check() {
    let app = create_app();   // your Router builder function

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```
`tower::ServiceExt::oneshot` sends a single request directly into the `Router` (which is a `Service`) in-process, without binding a real socket — fast, isolated tests that exercise the full middleware/routing/extraction stack exactly as production would, analogous to FastAPI's `TestClient` calling into the ASGI app directly.

### Q52. How do you test JSON request/response bodies end-to-end?
```rust
use serde_json::{json, Value};
use http_body_util::BodyExt;   // for `.collect()`

#[tokio::test]
async fn test_create_item() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/items")
                .header("content-type", "application/json")
                .body(Body::from(json!({ "name": "Widget", "price": 9.99 }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["name"], "Widget");
}
```

### Q53. How do you set up a real test database for integration tests (transactional rollback pattern)?
```rust
async fn setup_test_db() -> PgPool {
    let pool = PgPoolOptions::new()
        .connect(&std::env::var("TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();   // ensure schema is current
    pool
}

#[sqlx::test]   // sqlx's own test macro: spins up an isolated, migrated DB per test automatically
async fn test_insert_item(pool: PgPool) {
    let item = sqlx::query_as::<_, Item>("INSERT INTO items (name) VALUES ($1) RETURNING *")
        .bind("Widget")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(item.name, "Widget");
}
```
`#[sqlx::test]` (from `sqlx`'s `macros` feature) is the recommended approach for Postgres/MySQL integration tests: it automatically creates a fresh, migrated, isolated database per test function and tears it down afterward — avoiding cross-test pollution without hand-rolled transaction-rollback plumbing. For lighter unit tests that don't need a real DB, prefer the repository-trait pattern (Q41) with an in-memory fake implementation.

### Q54. How do you test middleware and authenticated routes?
```rust
#[tokio::test]
async fn test_protected_route_requires_auth() {
    let app = create_app();

    // No Authorization header -> expect 401
    let response = app.clone()
        .oneshot(Request::builder().uri("/profile").body(Body::empty()).unwrap())
        .await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // With a valid token -> expect 200
    let token = create_token("user-1", b"test-secret");
    let response = app
        .oneshot(
            Request::builder()
                .uri("/profile")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```
Because `Router` implements `Clone` (cheaply — it's `Arc`-backed internally) and `Service`, you can call `.oneshot()` multiple times against clones of the same app instance within one test to exercise different scenarios (unauthenticated vs authenticated) without rebuilding the router each time.

---

## 13. Deployment & Production

### Q55. How do you build an optimized release binary and containerize an Axum app?
```dockerfile
# Multi-stage build: compile in a full Rust image, ship only the tiny final binary
FROM rust:1.79 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/myapp /usr/local/bin/myapp
EXPOSE 3000
CMD ["myapp"]
```
```toml
# Cargo.toml - further shrink/optimize the release binary
[profile.release]
opt-level = 3
lto = true              # link-time optimization, smaller/faster binary, slower compile
codegen-units = 1        # more optimization opportunities across the whole crate
strip = true             # strip debug symbols from the final binary
```
A key Rust/Axum deployment advantage over interpreted-language frameworks: the final artifact is a **single, statically-mostly-linked native binary** with no runtime/interpreter dependency and typically far lower memory footprint and faster cold-start than a Python/Node equivalent — multi-stage Docker builds keep the final image minimal by discarding the entire Rust toolchain after compilation.

### Q56. How many worker threads should the Tokio runtime use, and how do you configure it?
```rust
// #[tokio::main] defaults to a multi-threaded runtime with worker threads = number of CPU cores
#[tokio::main]
async fn main() { /* ... */ }

// Explicit configuration:
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main());
}
```
Unlike the Python/Node model of scaling via multiple OS **processes** (each with its own GIL/event loop), a single Tokio multi-threaded runtime already spreads work across all available CPU cores within one process using a work-stealing scheduler — so a single Axum process, correctly written (no blocking calls inside `async fn`), can often saturate all cores without needing a process-manager-per-core setup like Gunicorn+Uvicorn workers. Horizontal scaling (multiple container replicas behind a load balancer) is still the standard approach for redundancy/deployment-scale reasons, just not strictly required for CPU utilization the way it is in Python.

### Q57. Why is blocking code inside an `async fn` dangerous in Axum, and how do you offload it?
```rust
// BAD: std::fs / a CPU-heavy loop / a sync DB driver call blocks the Tokio worker thread executing it,
// starving every other task scheduled on that thread
async fn bad_handler() -> String {
    let data = std::fs::read_to_string("big_file.txt").unwrap();   // blocking syscall!
    data
}

// GOOD: offload blocking work to Tokio's dedicated blocking-thread pool
async fn good_handler() -> Result<String, AppError> {
    let data = tokio::task::spawn_blocking(|| std::fs::read_to_string("big_file.txt"))
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(data)
}
```
Exactly the same underlying principle as FastAPI's async event-loop-blocking pitfall: Tokio's worker threads cooperatively multitask many async tasks, so a synchronous/blocking call inside one task monopolizes that OS thread and stalls every other task queued on it. `tokio::task::spawn_blocking` moves the blocking work onto a separate, larger thread pool reserved specifically for this purpose, keeping the async worker threads free.

### Q58. What does a production observability setup look like (structured logging, tracing, metrics)?
```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info,myapp=debug"))
        .with(tracing_subscriber::fmt::layer().json())   // structured JSON logs for log aggregators
        .init();
}

let app = Router::new()
    .route("/", get(root))
    .layer(TraceLayer::new_for_http());   // auto-instruments every request with a tracing span

async fn health_check() -> &'static str {
    tracing::info!("health check hit");
    "ok"
}
```
The `tracing` crate (not the standard `log` crate) is the ecosystem standard for structured, span-based observability in async Rust — spans correctly track causality across `.await` points (something the plain `log` crate cannot do reliably in concurrent async code), `tower-http::TraceLayer` auto-generates a span per HTTP request, and `tracing-opentelemetry` bridges these spans into distributed tracing backends (Jaeger, Honeycomb, Datadog) for cross-service request tracing in a microservices deployment.

### Q59. How do you add health/readiness checks for orchestrators like Kubernetes?
```rust
async fn liveness() -> &'static str { "ok" }   // "is the process alive" - cheap, no dependencies checked

async fn readiness(State(state): State<AppState>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1").execute(&state.db).await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok("ok")
}

let app = Router::new()
    .route("/healthz", get(liveness))
    .route("/readyz", get(readiness));
```
Separate **liveness** (should Kubernetes restart this pod?) from **readiness** (should the load balancer send it traffic right now?) — a slow/unavailable database should fail readiness (temporarily removing the pod from the load-balancing rotation) without necessarily failing liveness (which would trigger an unnecessary, disruptive pod restart).

---

## 14. Advanced / Architecture Questions

### Q60. How do you structure a large, production-grade Axum project?
```
myapp/
├── src/
│   ├── main.rs                # entry point: builds Router, starts server
│   ├── config.rs                # env-based configuration struct
│   ├── state.rs                  # AppState definition
│   ├── error.rs                   # AppError + IntoResponse impl
│   ├── routes/
│   │   ├── mod.rs                  # combines all sub-routers
│   │   ├── items.rs                 # /items routes
│   │   └── users.rs                  # /users routes
│   ├── extractors/                # custom extractors (AuthUser, etc.)
│   ├── middleware/                 # custom middleware
│   ├── models/                      # DB row structs / SeaORM entities
│   ├── schemas/                      # request/response DTOs (serde structs, separate from DB models)
│   └── repository/                    # DB access trait + implementations (Q41)
├── migrations/                          # sqlx/sea-orm migration files
├── tests/                                # integration tests (black-box, via oneshot/real server)
├── Cargo.toml
└── Dockerfile
```
As with the FastAPI equivalent, separating **schemas** (API contracts) from **models** (DB representation) from **repository** (data access) from **routes** (HTTP layer) keeps each concern independently testable — the Rust compiler additionally enforces these boundaries more rigidly than dynamic languages, since crossing them incorrectly (e.g., leaking a DB-only field into a response type) requires explicit, visible mapping code rather than accidental attribute leakage.

### Q61. How do you version an Axum API?
```rust
let v1 = Router::new().route("/items", get(items_v1::list));
let v2 = Router::new().route("/items", get(items_v2::list));

let app = Router::new()
    .nest("/api/v1", v1)
    .nest("/api/v2", v2);
```
URL-path versioning via `.nest()` is the simplest and most common approach — identical in spirit to FastAPI's `APIRouter(prefix=...)` versioning. Header-based versioning (a custom extractor reading an `Accept` or custom version header) is also possible but less discoverable for API consumers.

### Q62. How do you avoid the N+1 query problem in Axum + sqlx/SeaORM?
```rust
// BAD: N+1 - one query per article to fetch its author
let articles = sqlx::query_as::<_, Article>("SELECT * FROM articles").fetch_all(&pool).await?;
for article in &articles {
    let author = sqlx::query_as::<_, Author>("SELECT * FROM authors WHERE id = $1")
        .bind(article.author_id).fetch_one(&pool).await?;   // N extra round trips!
}

// GOOD: single JOIN query
#[derive(sqlx::FromRow)]
struct ArticleWithAuthor { title: String, author_name: String }

let rows = sqlx::query_as::<_, ArticleWithAuthor>(
    "SELECT a.title, u.name AS author_name FROM articles a JOIN authors u ON a.author_id = u.id"
).fetch_all(&pool).await?;

// SeaORM equivalent: `.find_also_related()` / `.find_with_related()` batches related loads
let articles_with_authors = article::Entity::find()
    .find_also_related(author::Entity)
    .all(&db)
    .await?;
```
Exactly the same underlying problem and solution as in any ORM-based stack: prefer explicit `JOIN`s or an ORM's eager-loading API over looping and issuing one query per row — SeaORM's `find_also_related`/`find_with_related` batch related-entity fetches into a bounded number of queries instead of one-per-row.

### Q63. How do you implement pagination idiomatically in Axum?
```rust
#[derive(Deserialize)]
struct Pagination {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_page_size")]
    page_size: u32,
}
fn default_page() -> u32 { 1 }
fn default_page_size() -> u32 { 20 }

#[derive(Serialize)]
struct PaginatedResponse<T> { items: Vec<T>, total: i64, page: u32, page_size: u32 }

async fn list_items(Query(p): Query<Pagination>, State(db): State<PgPool>) -> Result<Json<PaginatedResponse<Item>>, AppError> {
    let offset = ((p.page.max(1) - 1) * p.page_size) as i64;
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items ORDER BY id LIMIT $1 OFFSET $2")
        .bind(p.page_size as i64).bind(offset)
        .fetch_all(&db).await?;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM items").fetch_one(&db).await?;
    Ok(Json(PaginatedResponse { items, total, page: p.page, page_size: p.page_size }))
}
```
As with any stack, cursor-based pagination (using an indexed `id`/`created_at` column as the cursor instead of `OFFSET`) scales better for very large tables, since `OFFSET` forces the database to scan and discard all preceding rows.

### Q64. How does Axum's compile-time approach to routing/extraction change the class of bugs you encounter versus a dynamically-typed framework?
A mismatched extractor type, a handler returning a type that doesn't implement `IntoResponse`, an extractor placed in the wrong position (consuming the body before a later extractor needs it), or state requested via `State<T>` that was never actually provided via `.with_state()` are all **compile-time errors** in Axum — they simply cannot ship. This is the single biggest philosophical difference versus something like FastAPI: Python's dynamic typing catches equivalent mistakes at request time (via `422`s or 500s) or not at all until a specific code path is hit in production; Rust's type system, ownership rules, and Axum's trait-based design collapse an entire category of "worked in dev, broke on an edge case in prod" bugs into "doesn't compile."

### Q65. What are the main tradeoffs of choosing Axum/Rust for a web backend versus a framework like FastAPI/Express?
**Advantages**: raw performance and low, predictable latency (no GC pauses); very low memory footprint per instance; compile-time correctness (type/null/data-race safety) eliminating whole bug classes; single static binary deployment with fast cold starts, good for serverless/edge and containers alike.
**Tradeoffs**: steeper learning curve (ownership/borrowing, lifetimes, trait bounds) meaningfully slows initial feature velocity, especially for teams without prior Rust experience; smaller ecosystem of one-stop libraries compared to Python/Node for things like data science, ORMs with GUI admin panels, or niche third-party integrations; longer compile times can slow the local dev iteration loop compared to interpreted languages. The typical honest answer in an interview: Rust/Axum is a strong choice for performance-critical, high-throughput, or resource-constrained services (and for teams that value compile-time correctness enough to accept the learning curve), while Python/Node often win for iteration speed on business-logic-heavy CRUD apps or teams without existing Rust expertise.

---

# Part B — Complete Theory

## 15. Axum Theoretical Deep Dive & Inner Architecture

### 15.1 The Technology Stack

```
┌───────────────────────────────────────┐
│                 Axum                    │   <- routing sugar, extractors, handler ergonomics
├───────────────────────┬─────────────────┤
│         Tower           │   Tower-HTTP     │   <- Service/Layer middleware abstraction + prebuilt middleware
├───────────────────────┴─────────────────┤
│                 Hyper                    │   <- HTTP/1.1 & HTTP/2 protocol implementation
├───────────────────────────────────────┤
│                 Tokio                    │   <- async runtime: scheduler, timers, async I/O
├───────────────────────────────────────┤
│         Operating System (epoll/kqueue/IOCP)  │
└───────────────────────────────────────┘
```
- **Tokio** provides the async executor (a work-stealing, multi-threaded task scheduler), non-blocking TCP/UDP sockets backed by the OS's native async I/O facility (`epoll` on Linux, `kqueue` on BSD/macOS, IOCP on Windows), timers, and synchronization primitives (`Mutex`, channels, `RwLock`) designed to be held across `.await` points safely.
- **Hyper** implements the actual HTTP protocol (request/response parsing, HTTP/1.1 keep-alive, HTTP/2 multiplexing) on top of Tokio's async I/O — it is a low-level building block, not a framework with routing of its own.
- **Tower** defines the `Service` and `Layer` traits — a protocol-agnostic abstraction for "async request → response" logic and composable decorators around it. Tower itself doesn't know anything about HTTP specifically; it's general enough to also model gRPC services, load balancers, or retry logic.
- **Axum** is the top layer: it implements `Router` (itself a `Service`) on top of Hyper/Tower, adds ergonomic routing (`.route()`, path params via `matchit`), the extractor system (`FromRequest`/`FromRequestParts`), and the `Handler` trait that lets plain async functions be used as route handlers without manually implementing `Service`.

### 15.2 The Request Lifecycle (What Actually Happens)

1. A TCP connection arrives; Tokio's async I/O layer accepts it (non-blocking) and hands it to Hyper.
2. Hyper parses the raw bytes into an HTTP request (headers, method, URI, body stream) and calls into the ASGI-equivalent entry point — here, the `Router`'s `Service::call`.
3. The request passes through the **Tower middleware stack** in the order layers were applied (outermost layer's logic runs first on the way in).
4. Axum's internal router (backed by `matchit`, a radix-tree path matcher) matches the URI + method against registered routes, extracting path parameters as it goes.
5. Axum resolves the handler's function signature: each declared extractor's `from_request_parts`/`from_request` runs in argument order, short-circuiting immediately with the extractor's `Rejection`-derived response if any step fails — the handler body **never executes** on extraction failure.
6. The matched handler's `async fn` body runs as a Tokio task, polled by the runtime's scheduler like any other async task; if it awaits I/O (DB query, external HTTP call), the worker thread is freed to run other tasks in the meantime rather than blocking.
7. The handler's return value is converted to an HTTP response via `IntoResponse`.
8. The response passes back out through the middleware stack in reverse (innermost to outermost).
9. Hyper serializes the response and writes it back over the (still non-blocking) TCP socket.

### 15.3 Extractors as Compile-Time-Checked Dependency Injection

Axum's `FromRequestParts`/`FromRequest` traits let a handler's function signature declare exactly what it needs from the request — conceptually the same value proposition as FastAPI's `Depends()`-based dependency injection, but resolved entirely through Rust's trait system at **compile time** rather than a runtime dependency graph. There is no runtime "container" or registry: the compiler verifies every extractor in a handler's signature actually implements the required trait, that the body-consuming extractor (if any) is positioned last, and that any `State<T>` requested actually matches the type provided to `.with_state()` — a mismatch is a `cargo build` failure, not a runtime panic or an HTTP 500 discovered later.

### 15.4 Ownership and Borrowing in a Web Server Context

Because Axum handlers commonly need to read from state shared across many concurrent requests (a DB pool, a config struct), the ownership model that makes Rust attractive for systems programming shows up directly in web development idioms:
- Shared, read-mostly state is wrapped in `Arc<T>` (atomic reference counting) so many concurrent tasks can hold a cheap, thread-safe handle to the same underlying data without copying it.
- Shared *mutable* state requires explicit synchronization (`Mutex`/`RwLock`, or preferably a purpose-built concurrent structure like a connection pool) — the compiler will not allow unsynchronized shared mutability to compile, which is precisely the guarantee that prevents data races at the language level rather than relying on discipline or runtime detection.
- `Clone` is deliberately cheap for the types you put in `State` (pools, `Arc`-wrapped config) — cloning `AppState` per request (which Axum does internally to hand a copy to each request's task) is a pointer/refcount copy, not a deep copy.

### 15.5 Why "No Blocking in Async" Is a Stronger Rule in Rust Than in Python

In Python's asyncio model, a blocking call inside a coroutine stalls the single-threaded event loop for *all* concurrently scheduled coroutines on that loop. Tokio's default multi-threaded runtime spreads tasks across multiple OS threads via work-stealing, so a single blocking call only stalls the worker thread it happens to be running on — other threads keep serving other tasks. This makes the failure mode *less catastrophic* than Python's single-loop case, but the underlying rule (never block inside an `async fn`; offload via `spawn_blocking`) remains identical in spirit, because a busy enough workload can still starve all worker threads simultaneously, and unlike Python's threadpool-offload-by-default for sync `def` routes, Axum/Tokio does **not** automatically detect or offload blocking calls for you — it's the developer's responsibility to recognize blocking operations and wrap them explicitly.

### 15.6 The Type-State and Builder Patterns in Router Construction

`Router<S>` is generic over its state type `S`, and Axum uses this to enforce at compile time that you cannot `.route()` a handler requiring `State<AppState>` onto a router that hasn't yet been told (via `.with_state()`) what `AppState` actually is — the router's type literally changes (via `Router<AppState>` → `Router<()>`) as you call `.with_state()`, and route registration methods are only available/type-check correctly for the appropriate state type. This "type-state" style of API design — using the type system to make invalid usage sequences fail to compile rather than panic at runtime — recurs throughout the Rust ecosystem (also seen in `sqlx`'s query builders and typed HTTP client builders).

### 15.7 Where Axum Fits in the Broader Ecosystem

- **vs Actix-web**: Actix historically used its own actor-based runtime and middleware system; modern Actix-web runs on Tokio too, but its `Service`/middleware traits are its own, not Tower's — meaning Actix and Axum middleware are not directly interchangeable, whereas Axum benefits from the entire Tower/Tower-HTTP crate ecosystem "for free."
- **vs Node.js/Express**: Similar developer-facing productivity to Express's simplicity, but with compile-time type/null safety and no GC — Axum tends to win decisively on raw throughput per unit of memory/CPU, at the cost of Rust's steeper learning curve and longer compile times versus JavaScript's instant iteration loop.
- **vs Go (net/http, Gin, Echo)**: Go's goroutines + channels offer a comparably lightweight concurrency model without Rust's ownership/borrow-checker learning curve, and Go's GC is typically low-pause enough for most web workloads — the common tradeoff cited is Rust's stronger compile-time correctness guarantees and typically lower baseline memory use versus Go's faster compile times and gentler learning curve.

---

# Part C — Full Tutorial

## 16. Complete Tutorial: Building a Production-Style Task Manager API

We'll build a **Task Manager API** — user registration, JWT login, CRUD for tasks scoped to each user, Postgres persistence via `sqlx` with compile-time-checked queries, and integration tests. This mirrors real production patterns rather than a toy example.

### 16.1 Project Setup

```bash
cargo new task_manager && cd task_manager
```

```toml
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["trace", "cors"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "chrono", "uuid"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
jsonwebtoken = "9"
argon2 = "0.5"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "1"
dotenvy = "0.15"
```

Project structure we'll build:
```
task_manager/
├── src/
│   ├── main.rs
│   ├── state.rs
│   ├── error.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── jwt.rs
│   │   └── password.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── auth_routes.rs
│   │   └── task_routes.rs
│   └── models/
│       ├── mod.rs
│       ├── user.rs
│       └── task.rs
├── migrations/
│   ├── 0001_create_users.sql
│   └── 0002_create_tasks.sql
├── tests/
│   └── task_api.rs
├── .env
└── Cargo.toml
```

### 16.2 Database Migrations

```sql
-- migrations/0001_create_users.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```
```sql
-- migrations/0002_create_tasks.sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    is_completed BOOLEAN NOT NULL DEFAULT false,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tasks_owner_id ON tasks(owner_id);
```
```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
sqlx database create
sqlx migrate run
```

### 16.3 Application State & Error Type

```rust
// src/state.rs
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}
```

```rust
// src/error.rs
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("resource not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("resource already exists".into())
            }
            other => AppError::Internal(other.into()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            AppError::Internal(err) => {
                tracing::error!(error = ?err, "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

### 16.4 Models

```rust
// src/models/user.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: &'static str,
}
```

```rust
// src/models/task.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
}
```

### 16.5 Auth: Password Hashing, JWT, and the `AuthUser` Extractor

```rust
// src/auth/password.rs
use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(hash).map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}
```

```rust
// src/auth/jwt.rs
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user id
    pub exp: usize,
}

pub fn create_token(user_id: Uuid, secret: &str) -> Result<String, AppError> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| AppError::Internal(e.into()))
}

pub fn verify_token(token: &str, secret: &str) -> Result<Uuid, AppError> {
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
        .map_err(|_| AppError::Unauthorized)?;
    Uuid::parse_str(&data.claims.sub).map_err(|_| AppError::Unauthorized)
}
```

```rust
// src/auth/mod.rs
pub mod jwt;
pub mod password;

use axum::{extract::{FromRequestParts, State}, http::request::Parts, async_trait};
use uuid::Uuid;
use crate::{state::AppState, error::AppError};

pub struct AuthUser(pub Uuid);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts.headers.get("Authorization").ok_or(AppError::Unauthorized)?;
        let token = header.to_str().map_err(|_| AppError::Unauthorized)?
            .strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?;
        let user_id = jwt::verify_token(token, &state.jwt_secret)?;
        Ok(AuthUser(user_id))
    }
}
```
Note that `AuthUser` implements `FromRequestParts<AppState>` (not a generic `S`) since it needs access to `state.jwt_secret` — Axum resolves this automatically because the router's concrete state type is `AppState`.

### 16.6 Routes

```rust
// src/routes/auth_routes.rs
use axum::{extract::State, routing::post, Json, Router};
use crate::{state::AppState, error::AppError, auth::{jwt, password}, models::user::*};

async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Json<AuthResponse>, AppError> {
    if payload.password.len() < 8 {
        return Err(AppError::Validation("password must be at least 8 characters".into()));
    }
    let hash = password::hash_password(&payload.password)?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.username).bind(&payload.email).bind(&hash)
    .fetch_one(&state.db)
    .await?;

    let token = jwt::create_token(user.id, &state.jwt_secret)?;
    Ok(Json(AuthResponse { access_token: token, token_type: "bearer" }))
}

async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !password::verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = jwt::create_token(user.id, &state.jwt_secret)?;
    Ok(Json(AuthResponse { access_token: token, token_type: "bearer" }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
```

```rust
// src/routes/task_routes.rs
use axum::{extract::{State, Path}, routing::{get, patch, delete}, Json, Router, http::StatusCode};
use uuid::Uuid;
use crate::{state::AppState, error::AppError, auth::AuthUser, models::task::*};

async fn list_tasks(State(state): State<AppState>, AuthUser(user_id): AuthUser) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE owner_id = $1 ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(&state.db)
        .await?;
    Ok(Json(tasks))
}

async fn create_task(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::Validation("title cannot be empty".into()));
    }
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title, description, owner_id) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.title).bind(&payload.description).bind(user_id)
    .fetch_one(&state.db)
    .await?;
    Ok((StatusCode::CREATED, Json(task)))
}

async fn get_task(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1 AND owner_id = $2")
        .bind(task_id).bind(user_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(task))
}

async fn update_task(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>(
        r#"UPDATE tasks SET
             title = COALESCE($1, title),
             description = COALESCE($2, description),
             is_completed = COALESCE($3, is_completed)
           WHERE id = $4 AND owner_id = $5
           RETURNING *"#
    )
    .bind(&payload.title).bind(&payload.description).bind(payload.is_completed)
    .bind(task_id).bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(Json(task))
}

async fn delete_task(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1 AND owner_id = $2")
        .bind(task_id).bind(user_id)
        .execute(&state.db)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_tasks).post(create_task))
        .route("/{id}", get(get_task).patch(update_task).delete(delete_task))
}
```
Every task query filters by `owner_id = $1` derived from the JWT-authenticated `AuthUser` — this is the row-level authorization pattern that prevents one user from reading/modifying another user's tasks simply by guessing a UUID, enforced consistently at the query layer rather than as an afterthought check.

```rust
// src/routes/mod.rs
pub mod auth_routes;
pub mod task_routes;

use axum::Router;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes::router())
        .nest("/tasks", task_routes::router())
}
```

### 16.7 Main Entry Point

```rust
// src/main.rs
mod state;
mod error;
mod auth;
mod models;
mod routes;

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tower_http::{trace::TraceLayer, cors::{CorsLayer, Any}};
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info,task_manager=debug").init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!("./migrations").run(&db).await.expect("failed to run migrations");

    let state = AppState { db, jwt_secret };

    let app = Router::new()
        .nest("/api", routes::router())
        .route("/health", axum::routing::get(|| async { "ok" }))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
```

### 16.8 Integration Tests

```rust
// tests/task_api.rs
use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;
use http_body_util::BodyExt;
use serde_json::{json, Value};

async fn json_body(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[sqlx::test]
async fn test_register_login_and_create_task(pool: sqlx::PgPool) {
    let state = task_manager::state::AppState { db: pool, jwt_secret: "test-secret".into() };
    let app = task_manager::build_app(state);   // extracted from main() for testability

    // Register
    let response = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/auth/register")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "username": "alice", "email": "alice@example.com", "password": "supersecret"
            }).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    let token = body["access_token"].as_str().unwrap().to_string();

    // Create a task using the returned token
    let response = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/tasks")
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {token}"))
            .body(Body::from(json!({ "title": "Write tests" }).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // List tasks - should contain the one we just created
    let response = app.oneshot(
        Request::builder().uri("/api/tasks")
            .header("Authorization", format!("Bearer {token}"))
            .body(Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let tasks = json_body(response).await;
    assert_eq!(tasks.as_array().unwrap().len(), 1);
}
```
To make this testable, `main.rs`'s router-building logic should be extracted into a reusable `pub fn build_app(state: AppState) -> Router` in `lib.rs`, so both `main()` and the test suite construct the identical middleware/route stack rather than duplicating it — the same "app factory" principle used in most frameworks' testing guides.

### 16.9 Running It

```bash
# .env
DATABASE_URL=postgres://postgres:password@localhost/task_manager
JWT_SECRET=change-me-in-production

cargo run
# curl -X POST localhost:3000/api/auth/register -H 'content-type: application/json' \
#   -d '{"username":"alice","email":"alice@example.com","password":"supersecret"}'
```

This tutorial covers the core production skeleton — layered error handling, JWT auth via a custom extractor, row-level authorization, connection pooling, compile-time-checked SQL, and black-box integration tests — the same shape of concerns as any production REST API, expressed through Axum/Tokio/sqlx's specific idioms.
