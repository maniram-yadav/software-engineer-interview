# Rust Rocket Web Framework — Complete Guide

> Rocket is a batteries-included, type-safe web framework for Rust built on top of `tokio`. It emphasizes compile-time correctness (routes, request data, and responses are checked by the type system), ergonomic macros, and sane defaults.

---

## 1. Setup & Project Structure

```toml
# Cargo.toml
[dependencies]
rocket = { version = "0.5", features = ["json"] }
rocket_db_pools = { version = "0.1", features = ["sqlx_postgres"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}
```

Typical layout:
```
src/
  main.rs        // launch point, mounts routes/fairings
  routes/        // handler modules per resource
  models/        // domain structs + DB row mappings
  db.rs          // pool setup
  guards.rs      // custom request guards (auth, etc.)
  errors.rs      // AppError + Responder impls
Rocket.toml       // per-environment config (dev/staging/release)
```

---

## 2. Routing

Routes are declared with attribute macros and registered via `mount`.

```rust
#[get("/users/<id>")]
fn get_user(id: i32) -> String { format!("user {id}") }

#[get("/users/<id>?<active>")]
fn get_user_query(id: i32, active: Option<bool>) -> String { .. }

#[get("/files/<path..>")]
fn get_file(path: std::path::PathBuf) -> String { .. } // segment wildcard

#[post("/users", data = "<user>")]
fn create_user(user: Json<NewUser>) -> Json<User> { .. }
```

- Dynamic segments `<id>` are parsed via `FromParam`; failures result in Rocket trying the *next matching route* (rank-based fallback), or a 404/422.
- Query params `<active>` implement `FromForm`.
- Route **ranking**: Rocket auto-ranks more specific routes higher; you can override with `#[get("/users/<id>", rank = 2)]` to create fallback chains (e.g., typed match first, catch-all second).
- `routes![a, b, c]` macro collects handlers; `mount("/api/v1", routes![...])` namespaces them.

---

## 3. Request Guards (the core abstraction)

A request guard is any type implementing `FromRequest` — Rocket resolves it from the incoming request *before* the handler body runs, and it can fail, forward, or succeed. This is how Rocket does auth, extracting headers, DB connections, etc. — all statically typed as function parameters.

```rust
pub struct ApiKey(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = std::convert::Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-api-key") {
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key.to_string())),
            Some(_) => Outcome::Error((Status::Unauthorized, ())).into(),
            None => Outcome::Forward(Status::Unauthorized),
        }
    }
}

#[get("/secure")]
fn secure(key: ApiKey) -> &'static str { "authorized" }
```

Outcomes:
- `Success(val)` — handler runs with the value.
- `Error((Status, E))` — request fails with that status.
- `Forward(status)` — Rocket tries the next matching route (lets you implement auth fallbacks / content negotiation).

Guards compose: a handler can take multiple guards as separate parameters (`fn f(key: ApiKey, conn: DbConn, cookie: CookieJar<'_>)`), each resolved independently.

---

## 4. Data Guards — Body Parsing

`data = "<x>"` binds the request body via `FromData`. Built-in implementations:

- `Json<T>` — requires `T: Deserialize`, content-type `application/json`.
- `Form<T>` — `application/x-www-form-urlencoded`, `T: FromForm`.
- `LenientForm` (pre-0.5) / `Form` now lenient by default with `#[field(default)]`.
- `Data<'_>` — raw byte stream for custom parsing / streaming uploads.
- `TempFile` — multipart file uploads streamed to disk/memory.

```rust
#[derive(Deserialize)]
struct NewUser { name: String, email: String }

#[post("/users", format = "json", data = "<user>")]
fn create(user: Json<NewUser>) -> Json<User> { .. }
```

Custom data guards implement `FromData` for things like size-limited raw bodies or non-JSON formats.

---

## 5. Responders

Anything returned from a handler must implement `Responder`. Common ones:

| Type | Behavior |
|---|---|
| `&str` / `String` | 200, `text/plain` |
| `Json<T>` | 200, `application/json`, serializes `T` |
| `status::Created<T>` | 201 with Location header |
| `status::NotFound<T>` | 404 |
| `Redirect` | 302/303/307/308 |
| `(Status, T)` | custom status + body |
| `Option<T>` | `Some` → 200, `None` → 404 |
| `Result<T, E>` (E: Responder) | `Ok` → T's response, `Err` → E's response |
| `Stream<T>` | chunked streaming response |

Custom responder example (typical error envelope):

```rust
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Db(sqlx::Error),
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let (status, msg) = match self {
            AppError::NotFound(m) => (Status::NotFound, m),
            AppError::Validation(m) => (Status::UnprocessableEntity, m),
            AppError::Db(e) => (Status::InternalServerError, e.to_string()),
        };
        Json(serde_json::json!({ "error": msg })).respond_to(req).map(|mut r| {
            r.set_status(status);
            r
        })
    }
}
```

This lets handlers just return `Result<Json<T>, AppError>` and get consistent JSON error bodies.

---

## 6. Managed State

Application-wide shared state (config, connection pools, caches) is registered once and injected as a guard:

```rust
struct AppConfig { max_page_size: usize }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppConfig { max_page_size: 50 })
        .mount("/", routes![list])
}

#[get("/items")]
fn list(config: &State<AppConfig>) -> String {
    format!("max: {}", config.max_page_size)
}
```

`State<T>` is `Arc`-like — cheap to clone, shared across all workers. Must be `Send + Sync + 'static`.

---

## 7. Fairings (Middleware)

Fairings hook into the request/response lifecycle globally — analogous to middleware in Express/Actix.

```rust
pub struct RequestTimer;

#[rocket::async_trait]
impl Fairing for RequestTimer {
    fn info(&self) -> Info {
        Info { name: "Request Timer", kind: Kind::Request | Kind::Response }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        req.local_cache(|| Instant::now());
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let start = req.local_cache(|| Instant::now());
        res.set_raw_header("X-Response-Time", format!("{:?}", start.elapsed()));
    }
}

// registration
rocket::build().attach(RequestTimer)
```

Fairing kinds: `Ignite` (startup, can inspect/modify `Rocket` instance before launch — used for DB pool init), `Liftoff` (after successful launch), `Request`, `Response`. CORS, logging, metrics, and DB pool attachment are all typically implemented as fairings.

---

## 8. Catchers (Error Pages)

```rust
#[catch(404)]
fn not_found(req: &Request) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "error": "not found", "path": req.uri().to_string() }))
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> String {
    format!("{status}")
}

rocket::build().register("/", catchers![not_found, default_catcher])
```

---

## 9. Configuration (`Rocket.toml` / env vars)

```toml
[default]
address = "0.0.0.0"
port = 8000
workers = 8
log_level = "normal"

[default.databases.main_db]
url = "postgres://user:pass@localhost/mydb"

[release]
secret_key = "..." # required for cookies/sessions in production
```

Overridable via env vars: `ROCKET_PORT=9000`, `ROCKET_DATABASES='{main_db={url="..."}}'`. Custom typed config via `AdHoc::config::<MyConfig>()` and `figment` (Rocket's config layer is built on the `figment` crate, which merges TOML + env + defaults).

---

## 10. Database Connections & Pooling Strategies

Rocket doesn't ship an ORM; you plug in a database layer. The two dominant strategies:

### 10.1 `rocket_db_pools` (official, async, recommended for 0.5+)

Wraps `sqlx`, `deadpool`, or `mongodb` pools as a managed `Database` fairing.

```rust
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, PgPool};

#[derive(Database)]
#[database("main_db")]
struct MainDb(PgPool);

#[get("/users/<id>")]
async fn get_user(mut db: Connection<MainDb>, id: i32) -> Option<Json<User>> {
    sqlx::query_as!(User, "SELECT id, name, email FROM users WHERE id = $1", id)
        .fetch_optional(&mut **db)
        .await
        .ok()
        .flatten()
        .map(Json)
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(MainDb::init()).mount("/", routes![get_user])
}
```

- Pool is created once at launch (`Ignite` fairing) and pooled per-request via the `Connection<T>` guard — no manual pool management.
- Pool size configured via `Rocket.toml` (`[default.databases.main_db] url = "..."`, optional `max_connections`, `connect_timeout`).
- Health-checked automatically; failed pool init aborts launch.

### 10.2 Diesel (sync ORM) + `r2d2`/`diesel-async`

Diesel is a compile-time-checked, synchronous-by-default ORM (query DSL validated against your schema at compile time via `diesel print-schema` → `schema.rs`).

```rust
// schema.rs (generated)
diesel::table! {
    users (id) { id -> Int4, name -> Varchar, email -> Varchar }
}

// models.rs
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
struct User { id: i32, name: String, email: String }

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
struct NewUser<'a> { name: &'a str, email: &'a str }
```

Because Diesel's blocking calls can't run directly on Rocket's async executor, wrap them in `rocket_sync_db_pools` (spawns a blocking-thread pool per DB, exposes `.run(|conn| ...)`):

```rust
#[derive(Database)]
#[database("main_db")]
struct DbConn(diesel::PgConnection);

#[get("/users/<id>")]
async fn get_user(db: DbConn, id: i32) -> Option<Json<User>> {
    db.run(move |conn| {
        users::table.find(id).first::<User>(conn).optional().ok().flatten()
    }).await.map(Json)
}
```

Diesel migrations: `diesel migration generate create_users` → `up.sql` / `down.sql`, applied with `diesel migration run` (CLI) or `diesel::migrations::run_pending_migrations` at startup.

### 10.3 SeaORM (async, ActiveRecord-style)

```rust
use sea_orm::{Database, DatabaseConnection, EntityTrait};

#[derive(Database)]
struct DbConn; // typically manage a plain sea_orm::DatabaseConnection via .manage()

#[get("/users/<id>")]
async fn get_user(db: &State<DatabaseConnection>, id: i32) -> Option<Json<user::Model>> {
    user::Entity::find_by_id(id).one(db.inner()).await.ok().flatten().map(Json)
}
```

SeaORM entities are generated from the DB (`sea-orm-cli generate entity`) or hand-written; it supports async natively (no sync-pool bridging needed) and has a migration crate (`sea-orm-migration`).

### 10.4 Raw `sqlx` without the ORM layer

Many Rocket users skip an ORM entirely and use `sqlx::query!`/`query_as!` macros directly (compile-time checked against a live DB or `sqlx-data.json` offline cache). This is the most idiomatic "Rust way" pairing with `rocket_db_pools` shown above — type safety without ORM overhead.

### Comparison

| Approach | Async native | Compile-time SQL check | Migrations | Learning curve |
|---|---|---|---|---|
| `sqlx` + `rocket_db_pools` | Yes | Yes (macros) | `sqlx-cli migrate` | Low |
| Diesel + `rocket_sync_db_pools` | No (thread pool bridge) | Yes (schema DSL) | `diesel_migrations` | Medium-High |
| SeaORM | Yes | No (runtime) | `sea-orm-migration` | Medium |
| Raw `tokio-postgres`/`deadpool` | Yes | No | Manual | Low, more boilerplate |

**Recommendation given in most production Rocket setups:** `sqlx` + `rocket_db_pools` for new projects (async-first, no ORM magic, compile-time safety); Diesel when you want a mature, strongly-typed query builder and don't mind sync bridging; SeaORM when you want ActiveRecord ergonomics (`.save()`, relations, `find_related`) similar to Django/Rails.

---

## 11. Transactions

```rust
#[post("/transfer")]
async fn transfer(mut db: Connection<MainDb>) -> Result<(), AppError> {
    let mut tx = db.begin().await?;
    sqlx::query!("UPDATE accounts SET balance = balance - $1 WHERE id = $2", amt, from)
        .execute(&mut *tx).await?;
    sqlx::query!("UPDATE accounts SET balance = balance + $1 WHERE id = $2", amt, to)
        .execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}
```

---

## 12. Authentication & Sessions

- **Cookies**: `CookieJar<'_>` guard; `private_cookies` (encrypted+signed, requires `secret_key` in prod) for session tokens.
- **JWT**: implement a custom `FromRequest` guard that reads `Authorization: Bearer`, verifies with `jsonwebtoken` crate, and forwards claims as a typed guard (`AuthUser`).
- **OAuth**: `rocket_oauth2` crate provides `OAuth2<Provider>` guard + redirect flow helpers.
- **RBAC**: layer a second guard (`AdminUser`) that wraps `AuthUser` and checks role, returning `Forward`/`Error` if unauthorized — keeps permission checks in the type system rather than in handler bodies.

---

## 13. Validation

Rocket doesn't validate payload semantics itself; combine with:
- `validator` crate (`#[validate(email, length(min=1))]` on `Deserialize` structs), called manually in the handler or via a wrapping data guard.
- Custom `FromForm`/`FromData` impls that reject invalid input before the handler runs (fails fast, keeps handlers clean).

---

## 14. CORS

No first-party CORS; use the `rocket_cors` crate as a fairing:

```rust
let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::some_exact(&["https://example.com"]))
    .allowed_methods(vec![Method::Get, Method::Post].into_iter().map(From::from).collect())
    .to_cors()?;

rocket::build().attach(cors)
```

---

## 15. Testing

Rocket ships a first-class local test client — no real socket needed.

```rust
#[cfg(test)]
mod tests {
    use rocket::local::blocking::Client; // or rocket::local::asynchronous::Client
    use rocket::http::Status;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, Rocket!");
    }
}
```

For DB-backed tests: spin up a test DB (or `sqlx::test` attribute macro which auto-creates/migrates/tears-down a scoped test DB per test).

---

## 16. Streaming, WebSockets, Async Tasks

- Streaming responses: `rocket::response::stream::ReaderStream` / `EventStream` (Server-Sent Events) for chunked/live data.
- WebSockets: not built-in; use `ws` crate alongside Rocket, or run a separate `tokio-tungstenite` listener, since Rocket's routing model is HTTP-request/response centric.
- Background tasks: spawn via `rocket::tokio::spawn` inside a handler, or register long-running jobs in an `Ignite`/`Liftoff` fairing.

---

## 17. Deployment

- Compile release binary: `cargo build --release`.
- Config via `ROCKET_PROFILE=release` + env-var overrides (12-factor friendly).
- Typically run behind a reverse proxy (nginx/Caddy) for TLS termination, or use Rocket's built-in TLS (`tls` feature, rustls-based) directly.
- Docker: multi-stage build (`rust:slim` builder → `debian:slim`/`distroless` runtime) since binaries are static-ish but still need libc.
- Graceful shutdown: Rocket handles `SIGTERM`/`Shutdown` fairing hooks (`rocket::Shutdown` guard lets handlers trigger programmatic shutdown, e.g., for admin endpoints).

---

## 18. Rocket vs Other Rust Frameworks (context for interviews)

| | Rocket | Actix-web | Axum |
|---|---|---|---|
| Ergonomics | Macro-heavy, very declarative | Actor-based historically, now more direct | Tower-based, composable extractors |
| Async runtime | tokio (since 0.5) | tokio | tokio |
| Routing | Attribute macros + rank system | Builder pattern | Builder pattern, `Router` |
| Type safety | Request guards, compile-time route checks | Extractors (similar concept) | Extractors (similar concept) |
| Middleware | Fairings | Middleware trait/`wrap` | Tower `Layer`/`Service` |
| Maturity/perf | Slightly higher-level, historically behind Actix in raw benchmarks | Extremely high perf, actor-heritage complexity | Minimal, very close to raw Hyper/Tower, popular for new services |

---

## 19. Interview Questions

### Conceptual / Core Framework
1. What is a request guard in Rocket, and how does it differ from a data guard?
2. Explain the three `Outcome` variants (`Success`, `Error`, `Forward`) — when would you use `Forward` over `Error`?
3. How does Rocket's route ranking system work, and when do you need to set an explicit `rank`?
4. What must a type implement to be returned from a route handler? Explain `Responder`.
5. How does managed state (`State<T>`) differ from a request guard that reads from a DB pool per request?
6. What is a fairing, and what are the four kinds (`Ignite`, `Liftoff`, `Request`, `Response`)? Give a real use case for each.
7. How does Rocket achieve compile-time route-signature checking, and what class of bugs does that prevent versus a framework like Express?
8. Explain the difference between `FromParam`, `FromForm`, and `FromData`.
9. How would you implement role-based access control using nested request guards?
10. What is `local_cache` used for on a `Request`, and why is it needed (e.g., in the RequestTimer fairing example)?
11. How does Rocket handle catch-all/404 responses, and how do you customize them (`catchers!`)?
12. What is Rocket's config system built on (`figment`), and how do env-var overrides interact with `Rocket.toml`?
13. Why does Rocket require a `secret_key` in release mode for private cookies?

### Async & Concurrency
14. Rocket 0.5 moved to `tokio`-based async — what had to change from the sync 0.4 model, and why was `rocket_sync_db_pools` introduced?
15. How would you run a blocking/CPU-heavy operation inside an async Rocket handler without stalling the executor? (`tokio::task::spawn_blocking`)
16. How does the `workers` config value in `Rocket.toml` relate to the tokio runtime's thread pool?

### Database / ORM
17. Compare `sqlx`, Diesel, and SeaORM for use with Rocket — async support, compile-time query checking, and migration story.
18. Why can't a `diesel::PgConnection` be used directly in an async Rocket handler? How does `rocket_sync_db_pools` solve this?
19. How does `rocket_db_pools` differ architecturally from `rocket_sync_db_pools`?
20. Walk through implementing a transactional multi-step DB operation (e.g., a funds transfer) safely in a Rocket handler.
21. How do you structure DB connection pooling for multiple databases (e.g., a primary Postgres + a Redis cache) in one Rocket app?
22. What's the tradeoff of compile-time-checked SQL (`sqlx::query!`) vs a runtime query builder (SeaORM)?
23. How would you write an integration test that hits a real (ephemeral) database per test without cross-test pollution?

### Error Handling
24. How do you design a unified `AppError` enum that maps cleanly to HTTP status codes across many failure modes (validation, not-found, DB, auth)?
25. What's the idiomatic way to propagate errors from a DB call up through a handler using `?` with a custom `Responder`-implementing error type?

### Security
26. How do you implement JWT-based auth as a reusable request guard?
27. What are the risks of putting secrets directly in `Rocket.toml` vs environment variables, and how would you manage this across dev/staging/prod?
28. How would you rate-limit an endpoint in Rocket (no built-in support — discuss fairing-based or external approaches, e.g., a token-bucket fairing or a reverse-proxy layer)?
29. How does Rocket's private/signed cookie mechanism protect session data, and what does the `secret_key` actually do cryptographically?

### System Design / Practical
30. Design the module structure for a medium-sized REST API in Rocket (auth, users, orders) — where do guards, models, and DB logic live?
31. How would you version an API in Rocket (`/api/v1` vs `/api/v2`) while sharing common guards/state?
32. How would you add structured logging/tracing (e.g., `tracing` crate) across all requests via a fairing?
33. How would you implement pagination, filtering, and sorting on a "list resources" endpoint in a type-safe way using query guards?
34. What's your strategy for zero-downtime deploys of a Rocket service (graceful shutdown, health checks, readiness probes)?
35. How do you expose OpenAPI/Swagger docs for a Rocket API (e.g., `okapi`/`rocket_okapi` crate)?

### Comparative
36. How does Rocket's request-guard model compare to Axum's extractor model or Actix-web's `FromRequest`? What's philosophically similar/different?
37. When would you choose Rocket over Axum or Actix-web for a new service, and when would you not?
