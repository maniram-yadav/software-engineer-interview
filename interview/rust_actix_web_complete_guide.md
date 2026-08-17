# Actix Web — Complete Guide (with DB, ORM & Interview Prep)

> Actix Web is a high-performance, actor-model-influenced (though modern versions don't require actix-actor) async web framework for Rust, built on top of the `actix-rt` / `tokio` runtime. It is consistently one of the fastest web frameworks in the TechEmpower benchmarks.

---

## Table of Contents

1. [Core Concepts & Architecture](#1-core-concepts--architecture)
2. [Project Setup](#2-project-setup)
3. [Routing](#3-routing)
4. [Handlers & Extractors](#4-handlers--extractors)
5. [Application State & Shared Data](#5-application-state--shared-data)
6. [Middleware](#6-middleware)
7. [Request Guards & Scopes](#7-request-guards--scopes)
8. [JSON, Forms, Query, Path, Multipart](#8-json-forms-query-path-multipart)
9. [Error Handling](#9-error-handling)
10. [Database Connections & Pooling](#10-database-connections--pooling)
11. [ORM Integration — Diesel](#11-orm-integration--diesel)
12. [ORM Integration — SQLx](#12-orm-integration--sqlx)
13. [ORM Integration — SeaORM](#13-orm-integration--seaorm)
14. [Full REST API Example (SQLx + PostgreSQL)](#14-full-rest-api-example-sqlx--postgresql)
15. [Authentication & Authorization (JWT, Sessions, Cookies)](#15-authentication--authorization)
16. [WebSockets](#16-websockets)
17. [Streaming, Chunked Responses & Server-Sent Events](#17-streaming-chunked-responses--server-sent-events)
18. [Testing](#18-testing)
19. [Logging & Observability](#19-logging--observability)
20. [Configuration Management](#20-configuration-management)
21. [Performance Tuning](#21-performance-tuning)
22. [Deployment (Docker, TLS, Reverse Proxy)](#22-deployment-docker-tls-reverse-proxy)
23. [Common Pitfalls](#23-common-pitfalls)
24. [Interview Questions](#24-interview-questions)

---

## 1. Core Concepts & Architecture

Actix Web is built on several layers:

- **actix-rt** — a lightweight async runtime built on `tokio`, providing a multi-threaded, multi-process execution model.
- **actix-server** — handles the low-level TCP/socket accept loop, and worker-process/worker-thread management.
- **actix-http** — the HTTP/1.x and HTTP/2 protocol implementation.
- **actix-web** — the ergonomic web-framework layer: routing, extractors, middleware, `App`, `HttpServer`.

### Key architectural facts

- **`HttpServer`** binds a socket and spawns **N worker threads** (default = number of logical CPUs). Each worker runs its **own instance of the `App`**, and each worker has its own `tokio` single-threaded (actually multi-threaded per-worker) executor.
- Because each worker constructs its own `App`, **application state must be `Clone`-able cheaply** (typically wrapped in `web::Data<T>`, which is an `Arc` under the hood) — the *factory closure* passed to `HttpServer::new` runs once per worker.
- Actix Web is **not** inherently tied to the actor model anymore (historically it was built on `actix` actors — as of Actix Web 3+/4, actors are optional and mostly used for WebSockets).
- Everything is `async`/`.await` based using `Future`s; handlers can be `async fn`.

```
Client → HttpServer → [Worker 1: App instance, Worker 2: App instance, ...] → Router → Middleware chain → Handler → Response
```

---

## 2. Project Setup

```toml
# Cargo.toml
[package]
name = "actix-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
actix-rt = "2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
env_logger = "0.11"
log = "0.4"

# DB (pick one stack)
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "chrono", "uuid", "migrate"] }
# diesel = { version = "2", features = ["postgres", "r2d2", "chrono"] }
# sea-orm = { version = "0.12", features = ["sqlx-postgres", "runtime-tokio-native-tls", "macros"] }

uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
dotenvy = "0.15"
thiserror = "1"
validator = { version = "0.16", features = ["derive"] }
```

### Minimal server

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

#[actix_web::main] // macro expands to a tokio runtime + actix_rt::System::new
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)          // default = num_cpus
    .run()
    .await
}
```

`#[actix_web::main]` is sugar for:
```rust
fn main() -> std::io::Result<()> {
    actix_web::rt::System::new().block_on(async_main())
}
```

---

## 3. Routing

### Method 1: `App::route`
```rust
App::new()
    .route("/users", web::get().to(list_users))
    .route("/users", web::post().to(create_user))
    .route("/users/{id}", web::get().to(get_user))
    .route("/users/{id}", web::put().to(update_user))
    .route("/users/{id}", web::delete().to(delete_user));
```

### Method 2: `#[get]` / `#[post]` attribute macros (most common in real apps)
```rust
use actix_web::{get, post, put, delete, web, HttpResponse};

#[get("/users/{id}")]
async fn get_user(path: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "id": path.into_inner() }))
}

// register with .service(get_user)
```

### Method 3: `App::service` + `web::resource`
```rust
App::new().service(
    web::resource("/users/{id}")
        .route(web::get().to(get_user))
        .route(web::put().to(update_user))
);
```

### Path parameters, multiple segments, and tail matching
```rust
"/users/{id}"                 // single segment
"/files/{tail:.*}"            // greedy tail match
"/users/{user_id}/posts/{post_id}"
```

### Scopes (route grouping / versioning / sub-routers)
```rust
App::new().service(
    web::scope("/api/v1")
        .service(web::scope("/users")
            .route("", web::get().to(list_users))
            .route("/{id}", web::get().to(get_user))
        )
        .service(web::scope("/posts")
            .route("", web::get().to(list_posts))
        )
);
```

### `App::configure` — modularizing route registration across files
```rust
// routes/users.rs
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(list_users))
            .route("/{id}", web::get().to(get_user))
    );
}

// main.rs
App::new().configure(routes::users::config)
```

---

## 4. Handlers & Extractors

A handler is any `async fn` whose arguments implement `FromRequest` and whose return type implements `Responder`.

```rust
async fn handler(
    path: web::Path<(u32, String)>,
    query: web::Query<Filter>,
    body: web::Json<CreateUserDto>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder { ... }
```

### Built-in extractors

| Extractor | Purpose |
|---|---|
| `web::Path<T>` | URL path segments (tuple or struct via `Deserialize`) |
| `web::Query<T>` | Query string `?key=value` |
| `web::Json<T>` | JSON body (requires `Content-Type: application/json`) |
| `web::Form<T>` | `application/x-www-form-urlencoded` |
| `web::Data<T>` | Shared application state (`Arc`-wrapped) |
| `web::Bytes` | Raw request body bytes |
| `web::Payload` | Raw async stream of body chunks |
| `HttpRequest` | Full request object (headers, extensions, connection info) |
| `web::Header<T>` | A single typed header |
| `Option<T>` / `Result<T, E>` | Makes any extractor optional / fallible without failing the whole request |

### Custom extractors — implement `FromRequest`
```rust
use actix_web::{FromRequest, HttpRequest, dev::Payload, Error};
use futures_util::future::{ready, Ready};

pub struct AuthedUser(pub String);

impl FromRequest for AuthedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get("Authorization") {
            Some(v) => ready(Ok(AuthedUser(v.to_str().unwrap_or("").to_string()))),
            None => ready(Err(actix_web::error::ErrorUnauthorized("missing token"))),
        }
    }
}
```

### Responder trait — how return values become HTTP responses
```rust
impl Responder for MyType {
    type Body = BoxBody;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> { ... }
}
```
Built-in `Responder` impls exist for `String`, `&str`, `HttpResponse`, `web::Json<T>`, `impl Serialize` via `web::Json`, `(T, StatusCode)`, `Option<T>`, `Result<T, E>`, etc.

---

## 5. Application State & Shared Data

```rust
struct AppState {
    db_pool: PgPool,
    app_name: String,
    counter: std::sync::atomic::AtomicI64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new().connect("...").await.unwrap();
    let state = web::Data::new(AppState {
        db_pool: pool,
        app_name: "demo".into(),
        counter: Default::default(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())   // clone of Arc, cheap
            .route("/", web::get().to(handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn handler(data: web::Data<AppState>) -> impl Responder {
    data.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    HttpResponse::Ok().body(format!("app={}", data.app_name))
}
```

**Critical gotcha:** `web::Data::new(x)` must be created **once outside** the `HttpServer::new` closure and then `.clone()`d inside — because `HttpServer::new` factory runs once *per worker thread*. If you call `web::Data::new` *inside* the closure, each worker gets an independent, disconnected copy of state (usually wrong for a DB pool/counter that must be shared).

For mutable shared state, wrap in `Mutex`/`RwLock` or use atomics — `web::Data<T>` itself only gives you `&T` (shared reference), not `&mut T`.

```rust
web::Data<Mutex<HashMap<String, String>>>
```

### Configuring JSON payload limits, per-extractor config
```rust
App::new().app_data(
    web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
        })
)
```

---

## 6. Middleware

Middleware wraps the service chain — logging, auth, compression, CORS, rate limiting, etc.

### Built-in middleware
```rust
use actix_web::middleware::{Logger, Compress, NormalizePath, DefaultHeaders};

App::new()
    .wrap(Logger::default())                 // access logging
    .wrap(Compress::default())                // gzip/br response compression
    .wrap(NormalizePath::trim())               // strip trailing slashes
    .wrap(DefaultHeaders::new().add(("X-Version", "1.0")))
```

### CORS (via `actix-cors` crate)
```rust
use actix_cors::Cors;

App::new().wrap(
    Cors::default()
        .allowed_origin("https://example.com")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::CONTENT_TYPE])
        .max_age(3600)
)
```

### Custom middleware — the simple way (`wrap_fn`)
```rust
use actix_web::dev::Service;

App::new().wrap_fn(|req, srv| {
    println!("Incoming: {} {}", req.method(), req.path());
    let fut = srv.call(req);
    async {
        let res = fut.await?;
        println!("Outgoing status: {}", res.status());
        Ok(res)
    }
});
```

### Custom middleware — the full way (implementing `Transform` + `Service`)
```rust
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct RequestTimer;

impl<S, B> Transform<S, ServiceRequest> for RequestTimer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestTimerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestTimerMiddleware { service }))
    }
}

pub struct RequestTimerMiddleware<S> { service: S }

impl<S, B> Service<ServiceRequest> for RequestTimerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            log::info!("request took {:?}", start.elapsed());
            Ok(res)
        })
    }
}
```

**Note:** Middleware order matters — `wrap()` calls are applied so the *last-registered* middleware runs *first* on the request (outermost), which is the opposite of what beginners expect. Think of it like an onion: last `.wrap()` = outer layer.

---

## 7. Request Guards & Scopes

```rust
use actix_web::guard;

App::new().service(
    web::resource("/admin")
        .guard(guard::Header("X-Admin", "true"))
        .to(admin_handler)
);

// Method guards, combinators
web::resource("/x")
    .guard(guard::Any(guard::Get()).or(guard::Post()))
    .to(handler);
```

Guards let you route the *same path* to different handlers based on headers, methods, host, or custom predicates — useful for API versioning via `Accept` headers.

---

## 8. JSON, Forms, Query, Path, Multipart

### JSON in / out
```rust
#[derive(Deserialize)]
struct CreateUserDto { name: String, email: String }

#[derive(Serialize)]
struct UserDto { id: i32, name: String }

async fn create_user(body: web::Json<CreateUserDto>) -> impl Responder {
    HttpResponse::Created().json(UserDto { id: 1, name: body.name.clone() })
}
```

### Query strings
```rust
#[derive(Deserialize)]
struct Pagination { page: Option<u32>, limit: Option<u32> }

async fn list(q: web::Query<Pagination>) -> impl Responder {
    let page = q.page.unwrap_or(1);
    HttpResponse::Ok().json(serde_json::json!({ "page": page }))
}
```

### Multipart file upload (via `actix-multipart`)
```rust
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use std::io::Write;

async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let filename = field.content_disposition().get_filename().unwrap_or("file").to_string();
        let filepath = format!("./uploads/{filename}");
        let mut f = web::block(move || std::fs::File::create(filepath)).await??;
        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }
    Ok(HttpResponse::Ok().finish())
}
```

### Path with multiple typed segments
```rust
async fn handler(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, slug) = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({ "id": id, "slug": slug }))
}
```

---

## 9. Error Handling

Actix Web uses `actix_web::Error`, which any type implementing `std::error::Error + ResponseError` can convert into.

```rust
use actix_web::{ResponseError, HttpResponse, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("internal error")]
    Internal,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Database(_) | AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}

// Handlers just use `?`
async fn get_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await?                     // sqlx::Error -> AppError via #[from]
        .ok_or(AppError::NotFound)?;
    Ok(HttpResponse::Ok().json(row))
}
```

### Global JSON error handler for extractor failures
```rust
web::JsonConfig::default().error_handler(|err, _req| {
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(serde_json::json!({"error": "invalid json"})),
    ).into()
})
```

### Custom 404 / default service
```rust
App::new().default_service(web::route().to(|| async {
    HttpResponse::NotFound().json(serde_json::json!({"error": "route not found"}))
}))
```

---

## 10. Database Connections & Pooling

Rust has **no built-in ORM/DB layer** in std — you pick a crate. The three dominant choices:

| Library | Style | Compile-time SQL check | Async | Notes |
|---|---|---|---|---|
| **SQLx** | Query builder / raw SQL macros | Yes (`query!` macros, needs `DATABASE_URL` or offline cache) | Native async | Not a "traditional" ORM — closer to a typed query layer |
| **Diesel** | Full ORM, DSL query builder | Yes (via schema macros, compile-time) | Sync by default (use `diesel-async` for async) | Most mature, strictest type system |
| **SeaORM** | Full async ORM (ActiveRecord-style) | Runtime-checked (built on SQLx) | Native async | Closest to something like TypeORM/Entity Framework |

### General pooling concept

All three ultimately manage a **connection pool** — a fixed-size set of already-established DB connections that handlers borrow (`.acquire()` / `.get()`), use, and return. Pooling avoids the cost of a fresh TCP+auth handshake per request.

- **SQLx**: `sqlx::Pool<Postgres>` (aka `PgPool`), built on top of its own internal pooling (no external pool crate needed).
- **Diesel (sync)**: typically paired with `r2d2` (`Pool<ConnectionManager<PgConnection>>`).
- **Diesel-async**: paired with `deadpool` or `bb8`.
- **SeaORM**: wraps SQLx pools internally (`DatabaseConnection`).

Pool is created **once** at startup and stored in `web::Data`, shared (via `Arc`) across all workers.

```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(20)          // upper bound — tune vs DB max_connections / (num_app_instances)
    .min_connections(2)
    .acquire_timeout(std::time::Duration::from_secs(3))
    .idle_timeout(std::time::Duration::from_secs(600))
    .connect(&database_url)
    .await
    .expect("failed to connect to Postgres");
```

**Sizing rule of thumb:** `max_connections * num_app_replicas` should stay comfortably under the database's own `max_connections` limit (Postgres default 100), leaving headroom for admin/migration connections. For Actix Web specifically, remember there are `N` workers *within one process*, all sharing the *same* pool instance (since `web::Data` is `Arc`-cloned, not re-created per worker) — so the pool's `max_connections` is a process-wide cap, not per-worker.

---

## 11. ORM Integration — Diesel

Diesel is a **synchronous**, compile-time-checked ORM/query-builder. To use it inside Actix Web's async handlers you must offload blocking DB calls via `web::block` (backed by a `tokio` blocking thread pool), or use `diesel-async`.

### Setup
```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration generate create_users
```

```sql
-- migrations/xxxx_create_users/up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

### Schema (auto-generated by `diesel print-schema`)
```rust
// schema.rs
diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}
```

### Models
```rust
use diesel::prelude::*;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
```

### Pool with r2d2 + Actix integration
```rust
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

let manager = ConnectionManager::<PgConnection>::new(&database_url);
let pool: DbPool = r2d2::Pool::builder()
    .max_size(15)
    .build(manager)
    .expect("failed to create pool");

App::new().app_data(web::Data::new(pool.clone()))
```

### Handler using `web::block` to avoid blocking the async executor
```rust
use crate::schema::users::dsl::*;

async fn create_user(
    pool: web::Data<DbPool>,
    payload: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    let new_user = payload.into_inner();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut conn)
    })
    .await
    .map_err(|_| AppError::Internal)?
    .map_err(|_: diesel::result::Error| AppError::Internal)?;

    Ok(HttpResponse::Created().json(user))
}

async fn get_user(pool: web::Data<DbPool>, uid: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let uid = uid.into_inner();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        users.filter(id.eq(uid)).first::<User>(&mut conn)
    })
    .await
    .map_err(|_| AppError::Internal)?
    .map_err(|_| AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(user))
}
```

**Why `web::block`?** Diesel's `PgConnection` is blocking/synchronous. Calling it directly inside an `async fn` would block the entire tokio worker thread, stalling every other request scheduled on it. `web::block` moves the closure to a dedicated blocking-thread-pool (`tokio::task::spawn_blocking` under the hood) and returns a `Future` you can `.await`.

### `diesel-async` alternative (native async, no `web::block` needed)
```rust
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager}};

let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&database_url);
let pool = Pool::builder(config).build().unwrap();

async fn get_user(pool: web::Data<Pool<AsyncPgConnection>>, uid: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().await.map_err(|_| AppError::Internal)?;
    let user = users.filter(id.eq(uid.into_inner())).first::<User>(&mut conn).await.map_err(|_| AppError::NotFound)?;
    Ok(HttpResponse::Ok().json(user))
}
```

---

## 12. ORM Integration — SQLx

SQLx is the most popular choice in Actix Web projects because it's natively async and integrates with zero glue code.

### Compile-time checked queries
```rust
// requires DATABASE_URL env var at compile time, or `cargo sqlx prepare` for offline mode
let user = sqlx::query_as!(
    User,
    "SELECT id, name, email, created_at FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;
```

### Dynamic (non-macro) queries — no compile-time DB needed
```rust
#[derive(sqlx::FromRow, Serialize)]
struct User { id: i32, name: String, email: String }

let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = $1")
    .bind(user_id)
    .fetch_one(&pool)
    .await?;
```

### Migrations
```bash
cargo install sqlx-cli
sqlx migrate add create_users
sqlx migrate run
```
```rust
// run migrations programmatically at startup
sqlx::migrate!("./migrations").run(&pool).await?;
```

### Transactions
```rust
let mut tx = pool.begin().await?;
sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
    .bind(&name).bind(&email)
    .execute(&mut *tx)
    .await?;
sqlx::query("UPDATE accounts SET balance = balance - $1 WHERE user_id = $2")
    .bind(amount).bind(user_id)
    .execute(&mut *tx)
    .await?;
tx.commit().await?;   // or tx.rollback().await? — auto-rollback on drop if not committed
```

---

## 13. ORM Integration — SeaORM

SeaORM gives ActiveRecord-style ergonomics (similar to Django ORM / TypeORM) on top of SQLx.

```bash
sea-orm-cli generate entity -o src/entities --database-url $DATABASE_URL
```

```rust
// entities/user.rs (generated)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
```

```rust
use sea_orm::{Database, DatabaseConnection, EntityTrait, ActiveModelTrait, Set};

let db: DatabaseConnection = Database::connect(&database_url).await?;

// Create
let new_user = user::ActiveModel {
    name: Set("Alice".to_owned()),
    email: Set("alice@example.com".to_owned()),
    ..Default::default()
};
let inserted = new_user.insert(&db).await?;

// Read
let user = user::Entity::find_by_id(1).one(&db).await?;

// Update
let mut active: user::ActiveModel = user.unwrap().into();
active.name = Set("Alice2".to_owned());
active.update(&db).await?;

// Delete
user::Entity::delete_by_id(1).exec(&db).await?;
```

Store `DatabaseConnection` in `web::Data<DatabaseConnection>` exactly like a SQLx pool — it's `Clone` + internally `Arc`-backed.

---

## 14. Full REST API Example (SQLx + PostgreSQL)

```rust
// main.rs
use actix_web::{web, App, HttpServer, middleware::Logger};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new()
        .max_connections(15)
        .connect(&database_url)
        .await
        .expect("failed to connect to db");

    sqlx::migrate!("./migrations").run(&pool).await.expect("migration failed");

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1/users")
                    .route("", web::get().to(handlers::list_users))
                    .route("", web::post().to(handlers::create_user))
                    .route("/{id}", web::get().to(handlers::get_user))
                    .route("/{id}", web::put().to(handlers::update_user))
                    .route("/{id}", web::delete().to(handlers::delete_user))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

```rust
// models.rs
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, validator::Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}
```

```rust
// handlers.rs
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;
use crate::{models::*, errors::AppError};

pub async fn list_users(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users ORDER BY id")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = $1")
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user(pool: web::Data<PgPool>, body: web::Json<CreateUserDto>) -> Result<HttpResponse, AppError> {
    body.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email"
    )
    .bind(&body.name)
    .bind(&body.email)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn update_user(pool: web::Data<PgPool>, id: web::Path<i32>, body: web::Json<CreateUserDto>) -> Result<HttpResponse, AppError> {
    body.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email"
    )
    .bind(&body.name)
    .bind(&body.email)
    .bind(id.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id.into_inner())
        .execute(pool.get_ref())
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(HttpResponse::NoContent().finish())
}
```

---

## 15. Authentication & Authorization

### JWT-based auth
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Serialize, Deserialize)]
struct Claims { sub: String, exp: usize }

fn create_token(user_id: &str, secret: &[u8]) -> String {
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap()
}

// Extractor-based auth guard
impl FromRequest for AuthedUser {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let result = req.headers().get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .and_then(|token| decode::<Claims>(token, &DecodingKey::from_secret(b"secret"), &Validation::default()).ok())
            .map(|data| AuthedUser(data.claims.sub));

        std::future::ready(result.ok_or_else(|| actix_web::error::ErrorUnauthorized("invalid token")))
    }
}
```

### Session-based auth (`actix-session` + `actix-identity`)
```rust
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;

App::new().wrap(
    SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
        .cookie_secure(true)
        .build()
)
```

### Role-based middleware guard
```rust
async fn admin_only(user: AuthedUser, req: HttpRequest) -> Result<HttpResponse, AppError> {
    if user.role != "admin" {
        return Err(AppError::Forbidden);
    }
    Ok(HttpResponse::Ok().finish())
}
```

---

## 16. WebSockets

Actix Web integrates with `actix-ws` (modern) or the older `actix` actor-based `ws::WebsocketContext`.

### Modern approach with `actix-ws`
```rust
use actix_ws::Message;
use futures_util::StreamExt;

async fn ws_handler(req: HttpRequest, body: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => { let _ = session.text(text).await; }
                Message::Ping(bytes) => { let _ = session.pong(&bytes).await; }
                Message::Close(reason) => { let _ = session.close(reason).await; break; }
                _ => {}
            }
        }
    });

    Ok(response)
}
```

### Legacy actor-based approach
```rust
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

struct MyWs;
impl Actor for MyWs { type Context = ws::WebsocketContext<Self>; }

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(MyWs {}, &req, stream)
}
```

---

## 17. Streaming, Chunked Responses & Server-Sent Events

```rust
use actix_web::{HttpResponse, web};
use futures_util::stream;

async fn stream_numbers() -> HttpResponse {
    let s = stream::iter(1..=10).map(|n| Ok::<_, actix_web::Error>(web::Bytes::from(format!("{n}\n"))));
    HttpResponse::Ok().content_type("text/plain").streaming(s)
}

// Server-Sent Events
async fn sse() -> HttpResponse {
    let stream = stream::unfold(0u32, |count| async move {
        actix_web::rt::time::sleep(std::time::Duration::from_secs(1)).await;
        Some((Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: {count}\n\n"))), count + 1))
    });
    HttpResponse::Ok().content_type("text/event-stream").streaming(stream)
}
```

---

## 18. Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, http::StatusCode};

    #[actix_web::test]
    async fn test_get_user() {
        let pool = create_test_pool().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .route("/users/{id}", web::get().to(get_user))
        ).await;

        let req = test::TestRequest::get().uri("/users/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: User = test::read_body_json(resp).await;
        assert_eq!(body.id, 1);
    }

    #[actix_web::test]
    async fn test_create_user_validation() {
        let app = test::init_service(App::new().route("/users", web::post().to(create_user))).await;
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&serde_json::json!({ "name": "", "email": "not-an-email" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
```

Use `sqlx::test` for DB-backed tests that automatically get a fresh transactional/migrated test DB:
```rust
#[sqlx::test]
async fn test_insert(pool: PgPool) {
    let user = sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES ($1,$2) RETURNING *")
        .bind("Bob").bind("bob@example.com")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(user.name, "Bob");
}
```

---

## 19. Logging & Observability

```rust
// env_logger + log crate
env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
App::new().wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" %T"))
```

For structured logging / tracing (recommended for production):
```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-actix-web = "0.7"
```
```rust
use tracing_actix_web::TracingLogger;

tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();

App::new().wrap(TracingLogger::default())
```

Metrics: `actix-web-prom` exposes a `/metrics` Prometheus endpoint; combine with `opentelemetry` exporters for distributed tracing.

---

## 20. Configuration Management

```rust
// using `config` crate + `dotenvy` + `serde`
#[derive(Deserialize, Clone)]
struct Settings {
    database_url: String,
    port: u16,
    jwt_secret: String,
}

fn load_settings() -> Settings {
    dotenvy::dotenv().ok();
    config::Config::builder()
        .add_source(config::Environment::default())
        .build().unwrap()
        .try_deserialize().unwrap()
}
```

Typical `.env`:
```
DATABASE_URL=postgres://user:pass@localhost:5432/mydb
PORT=8080
JWT_SECRET=supersecret
RUST_LOG=info
```

---

## 21. Performance Tuning

- **Workers**: `HttpServer::workers(n)` — default is `num_cpus::get()`. Increasing beyond CPU count rarely helps for CPU-bound work; for I/O-bound (typical DB-backed API) it can help a bit due to blocking calls, but prefer moving blocking work to `web::block`.
- **Keep-Alive**: `HttpServer::keep_alive(Duration)` — tune for your load balancer.
- **`web::Data` cloning cost**: `Arc` clone is cheap (atomic increment) — safe to clone per-request via extractor.
- **Avoid blocking the executor**: any `std::fs`, `std::thread::sleep`, synchronous DB driver (Diesel sync), or CPU-heavy computation *must* go through `web::block` / `spawn_blocking`, or it starves the worker's other in-flight requests.
- **Connection pool sizing**: undersized pools cause request queueing/latency spikes; oversized pools can overload the DB server. Benchmark under realistic concurrency.
- **Compression**: `Compress` middleware trades CPU for bandwidth — usually worth it for JSON APIs behind a slow network, questionable for internal service-to-service calls.
- **Payload limits**: always set `JsonConfig`/`PayloadConfig` limits to avoid memory-exhaustion DoS from huge bodies.
- **`actix-web` uses `Rc` internally in single-threaded-per-worker contexts** — this is why many Actix Web types (like the `App` service factory) are `!Send`; be careful mixing in `tokio::spawn` (needs `Send`) vs `actix_web::rt::spawn` (works with `!Send` futures inside a worker).

---

## 22. Deployment (Docker, TLS, Reverse Proxy)

### Multi-stage Dockerfile
```dockerfile
FROM rust:1.79 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/actix-demo /usr/local/bin/app
EXPOSE 8080
CMD ["app"]
```

### Native TLS (rustls) termination in Actix itself
```rust
use rustls::ServerConfig;

let tls_config = load_rustls_config(); // build from cert/key files
HttpServer::new(|| App::new())
    .bind_rustls_0_23(("0.0.0.0", 8443), tls_config)?
    .run().await
```

In most production setups, TLS is terminated at a reverse proxy (nginx, Caddy, or a cloud load balancer) in front of Actix Web, which then just listens on plain HTTP internally.

---

## 23. Common Pitfalls

1. **Creating `web::Data` inside `HttpServer::new` closure** → each worker gets a disconnected instance; shared counters/pools silently don't share state.
2. **Blocking calls inside `async fn` handlers** (sync Diesel, `std::fs::read`, `std::thread::sleep`) → stalls the entire worker thread, killing throughput under load.
3. **Forgetting `.into_inner()` needed in some extractor contexts**, or fighting the borrow checker with `web::Path<T>`'s `Deref`.
4. **Assuming middleware runs top-to-bottom** — `.wrap()` order is LIFO for the request path (outermost = last registered).
5. **Not setting `JsonConfig`/`PayloadConfig` limits** — default payload limit is 256KB for JSON; large legitimate payloads will 400 unless raised, while omitting *any* thought about limits on custom payload types risks DoS.
6. **Pool exhaustion under load** — no visibility into `pool.size()` / `pool.num_idle()` metrics until production incident.
7. **Overusing `unwrap()`/`expect()` in handlers** — panics inside a handler currently crash *that request* (Actix catches panics per-worker-task in most configurations) but this is fragile; use `Result` + `ResponseError` consistently.
8. **CORS misconfiguration** — using `Cors::permissive()` in production.
9. **Mixing `tokio::spawn` and `actix_web::rt::spawn`** carelessly — `actix_web::rt::spawn` is required for `!Send` futures tied to the single-threaded-per-worker executor.

---

## 24. Interview Questions

### Conceptual / Architecture

1. What is Actix Web, and how does it differ from frameworks like Axum or Rocket?
2. Explain the actix-web worker model — what happens when `HttpServer::new(factory)` is called and `.workers(n)` is set?
3. Why must application state be wrapped in `web::Data<T>` rather than captured directly by reference in a closure?
4. What is the difference between creating `web::Data` inside vs. outside the `HttpServer::new` factory closure? What bug does the wrong placement cause?
5. Is Actix Web still built on the actor model? What role do `actix`-crate actors play in a modern Actix Web 4 app?
6. Explain the difference between `actix_web::rt::spawn` and `tokio::spawn`. Why does Actix Web care about `Send`/`!Send`?
7. What is the `Service` trait and how does Actix Web's middleware system use `Transform`/`Service`?
8. How does Actix Web achieve its high throughput compared to other web frameworks (per TechEmpower benchmarks)?
9. What's the execution model difference between a "worker" in Actix Web and a "thread" in a traditional thread-per-request server (e.g., classic Java servlet containers)?
10. Explain how `HttpServer` handles graceful shutdown.

### Routing & Handlers

11. What is the `FromRequest` trait, and how do you implement a custom extractor?
12. What is the `Responder` trait? Name built-in types that implement it.
13. Compare `App::route`, `#[get("/path")]` macro, and `web::resource` — when would you use each?
14. How do route guards (`guard::Header`, `guard::Any`) differ from middleware?
15. How does `web::scope` help with API versioning and modular route organization?
16. What's the difference between `web::Path<(u32, String)>` and `web::Path<MyStruct>`?
17. How would you make an extractor optional so a missing header doesn't fail the whole request?
18. Explain what `.into_inner()` does and why it's needed on `web::Json<T>`/`web::Path<T>`.
19. How would you implement API rate limiting as middleware?
20. Explain middleware ordering — why does `.wrap(A).wrap(B)` execute B before A on the way in?

### Error Handling

21. How does Actix Web convert application errors into HTTP responses? Explain the `ResponseError` trait.
22. How do you write a global error type that covers DB errors, validation errors, and auth errors uniformly?
23. What's the default behavior when a handler panics? How would you add a "catch panic" middleware or fallback?
24. How would you customize the error body returned when JSON deserialization of the request body fails?

### Database & ORM

25. Compare Diesel, SQLx, and SeaORM — sync vs async, compile-time vs runtime query checking, and when you'd choose each.
26. Why is `web::block` necessary when using synchronous Diesel inside an async Actix Web handler? What does it do internally?
27. How does SQLx's `query!`/`query_as!` macro achieve compile-time SQL verification? What's required at build time (or in CI) for it to work?
28. Explain connection pooling — why not open a new DB connection per request?
29. How do you size a connection pool correctly relative to `HttpServer::workers(n)` and the DB's own `max_connections`?
30. Walk through implementing a transactional operation (e.g., a money transfer) using SQLx — how do you ensure atomicity and rollback on error?
31. How would you run database migrations automatically at application startup vs. as a separate deploy step? Tradeoffs?
32. What does `r2d2` do, and how does it integrate with Diesel in an Actix Web app?
33. How does SeaORM's `ActiveModel` pattern compare to Diesel's `Insertable`/`Queryable` derive macros?
34. How would you write an integration test that hits a real (test) database, and how does `#[sqlx::test]` help?
35. What are N+1 query problems, and how would you avoid them in an ORM like SeaORM or Diesel?

### State, Concurrency & Performance

36. How do you safely share mutable state (e.g., an in-memory cache or counter) across Actix Web workers?
37. What's the cost of cloning `web::Data<T>` per request, and why is it cheap?
38. Why can blocking a single Actix Web worker thread degrade throughput for unrelated concurrent requests?
39. How would you profile and identify a blocking-call bottleneck in a production Actix Web service?
40. Explain backpressure in the context of streaming responses/`Payload` extraction for large file uploads.
41. What tools would you use to load-test an Actix Web API, and what metrics matter (p50/p99 latency, RPS, error rate)?

### Security & Auth

42. How would you implement JWT-based authentication as a custom extractor?
43. What's the difference between session-based (`actix-session`) and token-based (JWT) authentication in terms of scalability and revocation?
44. How do you configure CORS correctly (and why is `Cors::permissive()` dangerous in production)?
45. How would you protect an Actix Web API against payload-size DoS attacks?
46. How do you securely store and rotate a JWT signing secret in an Actix Web deployment?
47. How would you implement role-based access control (RBAC) middleware?

### Testing & Deployment

48. How do you write unit/integration tests for Actix Web handlers using `actix_web::test`?
49. How would you containerize an Actix Web app for production, and what goes in a multi-stage Dockerfile?
50. Where should TLS termination happen — in Actix Web itself (`rustls`) or at a reverse proxy — and what are the tradeoffs?
51. How do you implement structured logging/tracing (`tracing-actix-web`) and expose Prometheus metrics from an Actix Web app?
52. How would you perform a zero-downtime deployment/rolling restart for an Actix Web service?

### Practical / Coding Prompts (common in live interviews)

53. Implement a paginated `GET /users?page=&limit=` endpoint backed by SQLx/Postgres.
54. Implement custom middleware that logs request duration and injects a `X-Request-Id` header.
55. Implement a custom extractor that validates a Bearer JWT and injects the authenticated user into the handler.
56. Given a `User` model, implement full CRUD with proper error handling (404 on missing, 400 on validation failure, 201 on create).
57. Implement a WebSocket echo server using `actix-ws`.
58. Implement a rate limiter middleware (e.g., token bucket per IP) without external crates.
59. Diagnose this bug: "My counter in shared state resets randomly and isn't consistent across requests" (answer: `web::Data::new` called inside the server factory closure, or missing `Arc`/atomic).
60. Diagnose this bug: "My API becomes unresponsive under moderate load even though CPU usage is low" (answer: likely a blocking synchronous call — e.g., sync Diesel or `std::fs`— starving the async worker thread; fix with `web::block`/`spawn_blocking`).
