# The Complete FastAPI Guide
### Interview Questions with Detailed Answers + Full Theory + Step-by-Step Web App Tutorial

---

## Table of Contents

**Part A — Interview Questions**
1. [FastAPI Fundamentals](#1-fastapi-fundamentals)
2. [Routing & Request Handling](#2-routing--request-handling)
3. [Pydantic & Data Validation](#3-pydantic--data-validation)
4. [Dependency Injection](#4-dependency-injection)
5. [Async, Concurrency & Performance](#5-async-concurrency--performance)
6. [Middleware, CORS & Error Handling](#6-middleware-cors--error-handling)
7. [Authentication & Security](#7-authentication--security)
8. [Database Integration](#8-database-integration)
9. [Background Tasks, WebSockets & File Uploads](#9-background-tasks-websockets--file-uploads)
10. [Testing FastAPI Applications](#10-testing-fastapi-applications)
11. [Deployment & Production](#11-deployment--production)
12. [Advanced / Architecture Questions](#12-advanced--architecture-questions)

**Part B — Complete Theory**
13. [FastAPI Theoretical Deep Dive](#13-fastapi-theoretical-deep-dive)

**Part C — Full Tutorial**
14. [Complete Tutorial: Building a Production-Style Web App](#14-complete-tutorial-building-a-production-style-web-app)

---

# Part A — Interview Questions

## 1. FastAPI Fundamentals

### Q1. What is FastAPI, and why has it become so popular?
FastAPI is a modern, high-performance Python web framework for building APIs, built on top of **Starlette** (for the web/ASGI parts) and **Pydantic** (for data validation/serialization). It's designed around Python type hints.

**Why it's popular:**
- **Speed**: One of the fastest Python frameworks available, comparable to Node.js and Go (thanks to Starlette + `uvicorn`/ASGI, and no runtime overhead for validation since Pydantic v2 uses a Rust core).
- **Automatic interactive docs**: Swagger UI (`/docs`) and ReDoc (`/redoc`) generated automatically from your code.
- **Type-hint driven**: Request/response validation, serialization, and editor autocompletion all come from standard Python type hints — no separate schema definitions needed.
- **Async-native**: First-class `async`/`await` support for high-concurrency I/O-bound workloads.
- **Fewer bugs**: Automatic validation catches many errors before they reach business logic.

### Q2. What is ASGI, and how does it differ from WSGI?
**WSGI** (Web Server Gateway Interface) is the traditional synchronous interface between Python web apps and web servers (used by Flask, Django's classic sync views). It handles **one request at a time per worker thread/process** — blocking I/O ties up a worker.

**ASGI** (Asynchronous Server Gateway Interface) is the async successor — it supports `async`/`await`, WebSockets, HTTP/2, and long-lived connections, allowing a single worker to handle many concurrent connections cooperatively.

```python
# WSGI-style (blocking) - one request blocks the whole worker during I/O
def wsgi_view(request):
    data = requests.get("https://slow-api.com")   # blocks the thread
    return data

# ASGI-style (non-blocking) - the event loop can serve other requests during the await
async def asgi_view(request):
    async with httpx.AsyncClient() as client:
        data = await client.get("https://slow-api.com")   # yields control while waiting
    return data
```
FastAPI apps run on an ASGI server — typically **uvicorn** (built on `uvloop` + `httptools`) — instead of a WSGI server like `gunicorn`'s sync workers.

### Q3. What are the core building blocks FastAPI is built upon?
- **Starlette** — handles routing, middleware, WebSockets, background tasks, testing client, and the underlying ASGI application.
- **Pydantic** — handles data validation, serialization/deserialization, and schema generation (JSON Schema → OpenAPI).
- **OpenAPI & JSON Schema** — FastAPI auto-generates a machine-readable API spec from your path operations and Pydantic models, which powers Swagger UI/ReDoc.

### Q4. How do you install and create the most minimal FastAPI app?
```bash
pip install fastapi uvicorn[standard]
```
```python
# main.py
from fastapi import FastAPI

app = FastAPI()

@app.get("/")
async def root():
    return {"message": "Hello World"}
```
```bash
uvicorn main:app --reload
# Visit http://127.0.0.1:8000/docs for interactive Swagger UI
```

### Q5. What is the difference between FastAPI's `def` and `async def` path operations?
```python
@app.get("/sync-route")
def sync_route():
    # runs in an external threadpool (doesn't block the event loop),
    # good for blocking/synchronous code (e.g., legacy sync DB drivers)
    return {"type": "sync"}

@app.get("/async-route")
async def async_route():
    # runs directly on the event loop
    # MUST NOT contain blocking calls (e.g., time.sleep, sync requests.get)
    # or it will block the entire event loop for all concurrent requests
    return {"type": "async"}
```
FastAPI automatically detects whether your path operation function is a coroutine (`async def`) or a regular function (`def`). Regular `def` functions are run in a separate thread pool via `run_in_threadpool` so they don't block the event loop, but this has thread-pool overhead compared to native async execution.

### Q6. What is Starlette's role vs FastAPI's role?
Starlette provides the low-level ASGI toolkit (routing, middleware base classes, `Request`/`Response` objects, WebSockets, `TestClient`, background tasks). FastAPI is a **higher-level layer on top of Starlette** that adds automatic validation, serialization, dependency injection, and OpenAPI schema generation. You can drop down to raw Starlette `Request`/`Response` objects inside FastAPI whenever you need lower-level control.

---

## 2. Routing & Request Handling

### Q7. How do you define path parameters, query parameters, and their validation?
```python
from fastapi import FastAPI, Query, Path
from typing import Optional

app = FastAPI()

@app.get("/items/{item_id}")
async def read_item(
    item_id: int = Path(..., gt=0, description="The ID of the item"),
    q: Optional[str] = Query(None, min_length=3, max_length=50),
    skip: int = Query(0, ge=0),
    limit: int = Query(10, le=100),
):
    return {"item_id": item_id, "q": q, "skip": skip, "limit": limit}
```
- **Path parameters** (`{item_id}`) are part of the URL path — always required.
- **Query parameters** are everything else in the function signature not part of the path and not a Pydantic model — appear after `?` in the URL, e.g., `/items/5?q=test&skip=2`.
- `Path(...)` and `Query(...)` let you attach validation constraints (`gt`, `ge`, `lt`, `le`, `min_length`, `max_length`, `regex`) and metadata for the docs. `...` (Ellipsis) marks the parameter as required.

### Q8. How does FastAPI distinguish a request body from query parameters?
Any function parameter that is:
- A simple type (`int`, `str`, `float`, `bool`) → treated as a **query parameter**.
- A Pydantic `BaseModel` → treated as the **request body** (parsed from JSON).
- Declared with `Path(...)` and matching a `{}` in the route → **path parameter**.

```python
from pydantic import BaseModel

class Item(BaseModel):
    name: str
    price: float
    is_offer: Optional[bool] = None

@app.put("/items/{item_id}")
async def update_item(item_id: int, item: Item, q: Optional[str] = None):
    # item_id -> path param, item -> request body (JSON), q -> query param
    return {"item_id": item_id, **item.dict(), "q": q}
```

### Q9. How do you handle multiple request body parameters?
```python
class Item(BaseModel):
    name: str
    price: float

class User(BaseModel):
    username: str

@app.post("/purchases/")
async def create_purchase(item: Item, user: User):
    # FastAPI expects a JSON body shaped like:
    # {"item": {"name": "...", "price": ...}, "user": {"username": "..."}}
    return {"item": item, "user": user}
```
When multiple Pydantic models are declared as parameters, FastAPI nests them under their parameter names in the expected JSON body automatically.

### Q10. What are `response_model`, and why declare it explicitly?
```python
class UserIn(BaseModel):
    username: str
    password: str        # sensitive - should never be returned

class UserOut(BaseModel):
    username: str          # excludes password

@app.post("/users/", response_model=UserOut)
async def create_user(user: UserIn):
    # even if we return the full user object internally, FastAPI
    # filters the response to match UserOut's fields only
    return user
```
`response_model` defines the **output shape** independent of what your function internally returns or computes — critical for hiding sensitive fields (like password hashes), controlling API contracts, and getting accurate OpenAPI docs.

### Q11. How do path operations map to HTTP methods, and what's the significance of `status_code`?
```python
@app.get("/items/")           # read
@app.post("/items/")          # create
@app.put("/items/")           # full update
@app.patch("/items/")         # partial update
@app.delete("/items/")        # delete

@app.post("/items/", status_code=201)   # explicit HTTP status code
async def create_item(item: Item):
    return item
```
Setting the correct `status_code` (e.g., `201 Created` for POST, `204 No Content` for DELETE with no body) matters for REST convention compliance and client-side handling.

### Q12. How do you organize routes using `APIRouter`?
```python
# routers/items.py
from fastapi import APIRouter

router = APIRouter(prefix="/items", tags=["items"])

@router.get("/")
async def list_items():
    return []

@router.get("/{item_id}")
async def get_item(item_id: int):
    return {"item_id": item_id}

# main.py
from fastapi import FastAPI
from routers import items

app = FastAPI()
app.include_router(items.router)
```
`APIRouter` lets you split a large app into modular files (similar to Flask Blueprints or Django apps), each with its own prefix, tags (for Swagger grouping), and dependencies.

---

## 3. Pydantic & Data Validation

### Q13. What is Pydantic, and how does FastAPI use it?
Pydantic is a data validation library that uses Python type hints to validate, parse, and serialize data at runtime. FastAPI uses Pydantic `BaseModel` classes to:
1. Parse and validate incoming request bodies (JSON → Python objects).
2. Serialize outgoing responses (Python objects → JSON).
3. Auto-generate JSON Schema, which feeds into the OpenAPI spec for Swagger docs.

```python
from pydantic import BaseModel, EmailStr, Field
from typing import Optional
from datetime import datetime

class User(BaseModel):
    id: int
    username: str = Field(..., min_length=3, max_length=20)
    email: EmailStr
    age: Optional[int] = Field(None, ge=0, le=150)
    created_at: datetime = Field(default_factory=datetime.utcnow)

    class Config:
        str_strip_whitespace = True
```
Invalid data automatically raises a `422 Unprocessable Entity` response with a detailed field-by-field error breakdown — no manual `if`/`else` validation code required.

### Q14. Pydantic v1 vs v2 — what changed, and why does it matter for interviews?
Pydantic v2 (used by current FastAPI versions) rewrote its core validation engine in **Rust** (`pydantic-core`), giving massive performance gains (5-50x faster validation in many cases) over the pure-Python v1 implementation.

Key API changes to know:
```python
# Pydantic v1
class Config:
    orm_mode = True
user.dict()
user.json()

# Pydantic v2
class Config:
    from_attributes = True     # renamed from orm_mode
user.model_dump()
user.model_dump_json()
```
Interviewers may ask this to gauge whether you're up to date with the current ecosystem, since much tutorial content online still shows v1 syntax.

### Q15. How do you write custom validators in Pydantic?
```python
from pydantic import BaseModel, field_validator, model_validator

class SignupForm(BaseModel):
    password: str
    confirm_password: str

    @field_validator("password")
    @classmethod
    def password_strength(cls, v):
        if len(v) < 8:
            raise ValueError("Password must be at least 8 characters")
        return v

    @model_validator(mode="after")
    def passwords_match(self):
        if self.password != self.confirm_password:
            raise ValueError("Passwords do not match")
        return self
```
`field_validator` validates a single field; `model_validator` validates across multiple fields (e.g., password confirmation, date range checks).

### Q16. How do nested models and lists of models work?
```python
class Address(BaseModel):
    city: str
    zip_code: str

class Company(BaseModel):
    name: str
    address: Address                 # nested model

class Employee(BaseModel):
    name: str
    companies: list[Company] = []     # list of nested models

# Incoming JSON is automatically validated recursively at every nesting level
```

### Q17. What's the difference between `Optional[str]`, `str | None`, and required fields?
```python
from typing import Optional

class Item(BaseModel):
    name: str                          # required
    description: Optional[str] = None    # optional, defaults to None if omitted
    price: float | None = None            # modern syntax (Python 3.10+), same meaning
    tags: list[str] = []                   # optional with a mutable-looking default
                                             # (Pydantic handles this safely, unlike raw Python)
```
A field is **required** if it has no default value; it's **optional** if it has a default (including `None`). Merely typing it `Optional[str]` without `= None` still makes it required — a common gotcha.

### Q18. How do you use Pydantic's `BaseSettings` for configuration management?
```python
from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    database_url: str
    secret_key: str
    debug: bool = False

    class Config:
        env_file = ".env"

settings = Settings()   # automatically reads from environment variables / .env file
```
This gives typed, validated configuration loaded from environment variables — safer than scattering `os.getenv()` calls throughout the codebase.

---

## 4. Dependency Injection

### Q19. What is FastAPI's dependency injection system, and why is it useful?
`Depends()` lets you declare reusable, composable pieces of logic (auth checks, DB sessions, shared query params, permission checks) that FastAPI automatically resolves and injects into your path operations.

```python
from fastapi import Depends

def common_params(skip: int = 0, limit: int = 10):
    return {"skip": skip, "limit": limit}

@app.get("/items/")
async def list_items(params: dict = Depends(common_params)):
    return params

@app.get("/users/")
async def list_users(params: dict = Depends(common_params)):
    return params
```
Benefits: DRY reusable logic, easy to mock/override in tests, and encourages separation of concerns (auth/db/business logic stay decoupled from route handlers).

### Q20. How do dependencies with cleanup (using `yield`) work?
```python
def get_db():
    db = SessionLocal()
    try:
        yield db          # everything before yield = setup, runs before the request
    finally:
        db.close()          # everything after yield = teardown, runs after the response is sent

@app.get("/users/{user_id}")
async def get_user(user_id: int, db: Session = Depends(get_db)):
    return db.query(User).filter(User.id == user_id).first()
```
This mirrors a context manager (`__enter__`/`__exit__`): resources are acquired before the request handler runs and released afterward — even if an exception occurs, thanks to the `finally` block.

### Q21. What is the difference between dependencies at the path-operation level vs the router/app level?
```python
# Path-operation level
@app.get("/items/", dependencies=[Depends(verify_api_key)])
async def list_items():
    return []

# Router level (applies to all routes in the router)
router = APIRouter(dependencies=[Depends(verify_api_key)])

# App level (applies globally to every route in the app)
app = FastAPI(dependencies=[Depends(verify_api_key)])
```
Dependencies declared without capturing a return value (just for side effects, like auth checks) are still executed — useful for global security enforcement without modifying every function signature.

### Q22. How does FastAPI cache dependencies within a single request?
By default, a dependency used multiple times within the same request is only **computed once** and the cached result is reused (`use_cache=True` by default).
```python
def get_settings():
    print("Computing settings")   # only prints ONCE per request, even if used twice below
    return Settings()

@app.get("/config")
async def config(
    s1: Settings = Depends(get_settings),
    s2: Settings = Depends(get_settings),
):
    return s1 is s2   # True - same cached instance within this request
```
Set `Depends(get_settings, use_cache=False)` to force recomputation each time it's referenced.

### Q23. How do you override dependencies for testing?
```python
from fastapi.testclient import TestClient

def override_get_db():
    db = TestingSessionLocal()
    try:
        yield db
    finally:
        db.close()

app.dependency_overrides[get_db] = override_get_db   # swap real DB for test DB

client = TestClient(app)
response = client.get("/users/1")
```
This is one of FastAPI's most powerful testing features — swap real dependencies (DB, auth, external APIs) with test doubles without touching route code.

### Q24. What is `Annotated` and why is it now the recommended way to declare dependencies?
```python
from typing import Annotated
from fastapi import Depends

# Older style (still works)
async def get_items(db: Session = Depends(get_db)):
    ...

# Modern style using Annotated (Python 3.9+, recommended since FastAPI 0.95+)
DbDependency = Annotated[Session, Depends(get_db)]

async def get_items(db: DbDependency):
    ...
```
`Annotated` separates the **type** (`Session`) from the **FastAPI-specific metadata** (`Depends(get_db)`), making the dependency reusable as a type alias across many functions and more compatible with other tools that only understand the plain type hint.

---

## 5. Async, Concurrency & Performance

### Q25. When should you use `async def` vs regular `def` in a path operation?
Use `async def` when your I/O calls are truly **awaitable** (async DB drivers, `httpx.AsyncClient`, async file libraries). Use regular `def` when you're calling blocking/synchronous code (traditional `requests`, sync SQLAlchemy, CPU-bound work) — FastAPI runs these in a thread pool automatically so they don't block the event loop.

```python
# GOOD: async with an async-compatible library
@app.get("/weather")
async def get_weather():
    async with httpx.AsyncClient() as client:
        resp = await client.get("https://api.weather.com/data")
    return resp.json()

# BAD: blocking call inside async def blocks the ENTIRE event loop
@app.get("/weather-bad")
async def get_weather_bad():
    resp = requests.get("https://api.weather.com/data")   # blocks everyone!
    return resp.json()

# OK alternative: let FastAPI run it in a thread pool
@app.get("/weather-ok")
def get_weather_ok():                                     # plain def
    resp = requests.get("https://api.weather.com/data")    # blocking, but isolated to a worker thread
    return resp.json()
```

### Q26. Why is mixing blocking calls inside `async def` dangerous?
The ASGI event loop is **single-threaded** for coroutine execution. If a coroutine performs a blocking call (e.g., `time.sleep()`, sync `requests.get()`, CPU-heavy computation) without `await`, it freezes the entire event loop — **every other concurrent request is stalled** until that call finishes, defeating the purpose of async.

```python
import time

@app.get("/bad")
async def bad_endpoint():
    time.sleep(5)     # BLOCKS the whole server for 5 seconds for ALL clients!
    return {"done": True}

@app.get("/good")
async def good_endpoint():
    await asyncio.sleep(5)   # yields control, other requests proceed normally
    return {"done": True}
```

### Q27. How do you run CPU-bound work without blocking the event loop?
```python
from fastapi.concurrency import run_in_threadpool
import asyncio
from concurrent.futures import ProcessPoolExecutor

executor = ProcessPoolExecutor()

def cpu_heavy_task(n):
    return sum(i * i for i in range(n))

@app.get("/compute")
async def compute():
    loop = asyncio.get_event_loop()
    result = await loop.run_in_executor(executor, cpu_heavy_task, 10_000_000)
    return {"result": result}
```
True CPU-bound parallelism still requires a **process pool** (bypassing the GIL), run via `loop.run_in_executor`. Thread pools help with blocking I/O, not CPU-bound work.

### Q28. How do you run multiple async operations concurrently?
```python
import asyncio

async def fetch_user(user_id): ...
async def fetch_orders(user_id): ...

@app.get("/dashboard/{user_id}")
async def dashboard(user_id: int):
    user, orders = await asyncio.gather(
        fetch_user(user_id),
        fetch_orders(user_id),
    )
    return {"user": user, "orders": orders}
```
`asyncio.gather()` runs both coroutines concurrently rather than sequentially — critical for reducing latency when a single endpoint depends on multiple independent I/O calls.

### Q29. How does FastAPI/uvicorn achieve high performance in production?
- Multiple **uvicorn worker processes** (via `gunicorn -k uvicorn.workers.UvicornWorker` or `uvicorn --workers N`) utilize multiple CPU cores, since each process has its own event loop and GIL.
- **`uvloop`** (a faster C-based event loop implementation replacing the default `asyncio` loop) and **`httptools`** (fast HTTP parser) are installed via `uvicorn[standard]`.
- Pydantic v2's Rust core minimizes validation overhead.
- Async I/O allows a single worker to serve thousands of concurrent I/O-bound connections rather than one-per-thread.

---

## 6. Middleware, CORS & Error Handling

### Q30. What is middleware in FastAPI, and how do you write custom middleware?
```python
import time
from fastapi import FastAPI, Request

app = FastAPI()

@app.middleware("http")
async def add_process_time_header(request: Request, call_next):
    start_time = time.time()
    response = await call_next(request)      # calls the next middleware / route handler
    response.headers["X-Process-Time"] = str(time.time() - start_time)
    return response
```
Middleware wraps every request/response cycle — useful for logging, timing, adding headers, authentication pre-checks, and request/response transformation. Middleware runs **outside** exception handlers registered via `@app.exception_handler`, so uncaught exceptions in a route may not be visible to `try/except` inside middleware unless handled carefully.

### Q31. How do you configure CORS in FastAPI?
```python
from fastapi.middleware.cors import CORSMiddleware

app.add_middleware(
    CORSMiddleware,
    allow_origins=["https://myfrontend.com", "http://localhost:3000"],
    allow_credentials=True,
    allow_methods=["GET", "POST", "PUT", "DELETE"],
    allow_headers=["*"],
)
```
CORS (Cross-Origin Resource Sharing) middleware controls which frontend origins are allowed to call your API from a browser. Without it, browsers block cross-origin JavaScript requests due to the same-origin policy. **Never use `allow_origins=["*"]` together with `allow_credentials=True`** in production — it's a security risk (this combination is actually rejected by browsers per the CORS spec).

### Q32. How do you handle exceptions globally with custom exception handlers?
```python
from fastapi import Request
from fastapi.responses import JSONResponse

class ItemNotFoundError(Exception):
    def __init__(self, item_id: int):
        self.item_id = item_id

@app.exception_handler(ItemNotFoundError)
async def item_not_found_handler(request: Request, exc: ItemNotFoundError):
    return JSONResponse(
        status_code=404,
        content={"detail": f"Item {exc.item_id} not found"},
    )

@app.get("/items/{item_id}")
async def get_item(item_id: int):
    if item_id not in fake_db:
        raise ItemNotFoundError(item_id)
    return fake_db[item_id]
```
Custom exception handlers centralize error-response formatting, keeping route handlers clean and ensuring consistent error payloads across the whole API.

### Q33. How does `HTTPException` work, and how do you customize validation error responses?
```python
from fastapi import HTTPException
from fastapi.exceptions import RequestValidationError
from fastapi.responses import JSONResponse

@app.get("/items/{item_id}")
async def get_item(item_id: int):
    if item_id < 0:
        raise HTTPException(status_code=400, detail="item_id must be positive")
    return {"item_id": item_id}

# Override the default 422 validation error format
@app.exception_handler(RequestValidationError)
async def validation_exception_handler(request: Request, exc: RequestValidationError):
    return JSONResponse(
        status_code=422,
        content={"message": "Validation failed", "errors": exc.errors()},
    )
```
`HTTPException` is the standard way to return client-facing HTTP errors with a status code and detail message from anywhere in your route logic.

---

## 7. Authentication & Security

### Q34. What authentication mechanisms does FastAPI support out of the box?
FastAPI's `fastapi.security` module provides building blocks (not full implementations) for:
- **OAuth2** (via `OAuth2PasswordBearer` — most common for JWT-based token auth)
- **API Key** (header, query, or cookie based)
- **HTTP Basic Auth**
- **HTTP Bearer** tokens

These integrate with the dependency injection system and automatically document the auth scheme in Swagger UI (showing an "Authorize" button).

### Q35. How do you implement JWT authentication with OAuth2PasswordBearer?
```python
from fastapi import Depends, FastAPI, HTTPException, status
from fastapi.security import OAuth2PasswordBearer, OAuth2PasswordRequestForm
from jose import JWTError, jwt
from passlib.context import CryptContext
from datetime import datetime, timedelta

SECRET_KEY = "your-secret-key"
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 30

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")
oauth2_scheme = OAuth2PasswordBearer(tokenUrl="token")
app = FastAPI()

def verify_password(plain, hashed):
    return pwd_context.verify(plain, hashed)

def create_access_token(data: dict):
    to_encode = data.copy()
    expire = datetime.utcnow() + timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    to_encode.update({"exp": expire})
    return jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)

@app.post("/token")
async def login(form_data: OAuth2PasswordRequestForm = Depends()):
    user = authenticate_user(form_data.username, form_data.password)   # your DB lookup
    if not user:
        raise HTTPException(status_code=401, detail="Incorrect username or password")
    token = create_access_token({"sub": user.username})
    return {"access_token": token, "token_type": "bearer"}

async def get_current_user(token: str = Depends(oauth2_scheme)):
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
        username = payload.get("sub")
        if username is None:
            raise HTTPException(status_code=401, detail="Invalid token")
    except JWTError:
        raise HTTPException(status_code=401, detail="Invalid token")
    user = get_user_from_db(username)
    if user is None:
        raise HTTPException(status_code=401, detail="User not found")
    return user

@app.get("/users/me")
async def read_current_user(current_user=Depends(get_current_user)):
    return current_user
```
`OAuth2PasswordBearer` tells FastAPI to look for a `Bearer <token>` header and extract the token; the actual validation (decoding the JWT, checking the DB) is up to your `get_current_user` dependency, which you then inject into any protected route.

### Q36. How do you hash passwords securely?
```python
from passlib.context import CryptContext

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

hashed = pwd_context.hash("plaintext_password")     # store this, never the plaintext
pwd_context.verify("plaintext_password", hashed)      # True on login
```
Never store plaintext passwords. **bcrypt** (or **argon2**) is preferred over faster hashes like SHA-256 for passwords, precisely because it's intentionally slow — this resists brute-force attacks.

### Q37. How do you implement role-based access control (RBAC)?
```python
from enum import Enum

class Role(str, Enum):
    admin = "admin"
    user = "user"

def require_role(required_role: Role):
    def role_checker(current_user=Depends(get_current_user)):
        if current_user.role != required_role:
            raise HTTPException(status_code=403, detail="Insufficient permissions")
        return current_user
    return role_checker

@app.delete("/users/{user_id}")
async def delete_user(user_id: int, admin=Depends(require_role(Role.admin))):
    ...
```
A dependency **factory** (`require_role`) returns a dependency configured for a specific required role — a clean, reusable pattern for permission checks.

### Q38. What security best practices should you mention for a production FastAPI app?
- Store secrets (`SECRET_KEY`, DB credentials) in environment variables, never hard-coded.
- Use HTTPS in production (terminate TLS at a reverse proxy like nginx or a load balancer).
- Set short-lived access tokens + refresh token rotation for JWTs.
- Validate and sanitize all user input (Pydantic mostly handles this).
- Rate-limit sensitive endpoints (login, password reset) — e.g., via `slowapi`.
- Use parameterized queries / ORM (never raw string-interpolated SQL) to prevent SQL injection.
- Set restrictive CORS origins, not wildcard, especially with credentials.
- Keep dependencies patched (`pip-audit`, `safety` tools) for known CVEs.

---

## 8. Database Integration

### Q39. How do you integrate SQLAlchemy (sync) with FastAPI?
```python
# database.py
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, declarative_base

SQLALCHEMY_DATABASE_URL = "postgresql://user:pass@localhost/mydb"
engine = create_engine(SQLALCHEMY_DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

# models.py
from sqlalchemy import Column, Integer, String
from database import Base

class User(Base):
    __tablename__ = "users"
    id = Column(Integer, primary_key=True, index=True)
    username = Column(String, unique=True, index=True)

# main.py
from fastapi import Depends
from sqlalchemy.orm import Session

@app.get("/users/{user_id}")
async def get_user(user_id: int, db: Session = Depends(get_db)):
    user = db.query(User).filter(User.id == user_id).first()
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    return user
```

### Q40. How do you use async SQLAlchemy (fully async DB access)?
```python
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession, async_sessionmaker

DATABASE_URL = "postgresql+asyncpg://user:pass@localhost/mydb"
engine = create_async_engine(DATABASE_URL, echo=False)
AsyncSessionLocal = async_sessionmaker(engine, expire_on_commit=False)

async def get_db():
    async with AsyncSessionLocal() as session:
        yield session

@app.get("/users/{user_id}")
async def get_user(user_id: int, db: AsyncSession = Depends(get_db)):
    result = await db.execute(select(User).where(User.id == user_id))
    user = result.scalar_one_or_none()
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    return user
```
Fully async DB access (via `asyncpg` for PostgreSQL, `aiomysql` for MySQL) avoids blocking the event loop during DB I/O — important under high concurrency. Sync SQLAlchemy sessions used inside `async def` routes will block the event loop unless run in a threadpool (i.e., prefer plain `def` routes for sync DB code, or fully async DB code for `async def` routes).

### Q41. How does Pydantic + SQLAlchemy integration typically work (ORM to schema conversion)?
```python
class UserSchema(BaseModel):
    id: int
    username: str
    email: str

    class Config:
        from_attributes = True     # (Pydantic v2; was orm_mode in v1)

@app.get("/users/{user_id}", response_model=UserSchema)
async def get_user(user_id: int, db: Session = Depends(get_db)):
    user = db.query(User).filter(User.id == user_id).first()
    return user     # SQLAlchemy ORM object -> auto-converted to UserSchema via response_model
```
`from_attributes = True` tells Pydantic it's OK to read data from object attributes (not just dict keys), enabling direct conversion from ORM model instances to Pydantic schemas.

### Q42. How do you manage database migrations for a FastAPI + SQLAlchemy project?
```bash
pip install alembic
alembic init alembic
# edit alembic/env.py to import your Base.metadata

alembic revision --autogenerate -m "create users table"
alembic upgrade head
```
**Alembic** is the standard migration tool for SQLAlchemy-based projects (analogous to Django's `makemigrations`/`migrate`) — it diffs your models against the DB schema and generates versioned migration scripts.

### Q43. How do you use MongoDB (NoSQL) with FastAPI?
```python
from motor.motor_asyncio import AsyncIOMotorClient

client = AsyncIOMotorClient("mongodb://localhost:27017")
db = client["blogdb"]

@app.post("/articles/")
async def create_article(article: dict):
    result = await db.articles.insert_one(article)
    return {"id": str(result.inserted_id)}

@app.get("/articles/{article_id}")
async def get_article(article_id: str):
    from bson import ObjectId
    article = await db.articles.find_one({"_id": ObjectId(article_id)})
    if not article:
        raise HTTPException(status_code=404, detail="Not found")
    article["_id"] = str(article["_id"])
    return article
```
`motor` is the official async MongoDB driver, integrating naturally with FastAPI's async route handlers (as opposed to the synchronous `pymongo` driver, which would need to run in a threadpool).

---

## 9. Background Tasks, WebSockets & File Uploads

### Q44. What are `BackgroundTasks`, and when should you use them?
```python
from fastapi import BackgroundTasks

def send_welcome_email(email: str):
    # simulate a slow email-sending operation
    time.sleep(3)
    print(f"Email sent to {email}")

@app.post("/signup")
async def signup(email: str, background_tasks: BackgroundTasks):
    create_user(email)
    background_tasks.add_task(send_welcome_email, email)   # runs AFTER the response is sent
    return {"message": "Signed up successfully"}
```
`BackgroundTasks` runs a function after the response has already been returned to the client — good for lightweight, fire-and-forget tasks (sending a notification email, logging, cache invalidation). For heavier or more reliable async job processing (retries, scheduling, distributed workers), use a dedicated task queue like **Celery** or **arq** with a broker (Redis/RabbitMQ) instead — `BackgroundTasks` runs in-process and doesn't survive a server crash/restart.

### Q45. How do you implement WebSockets in FastAPI?
```python
from fastapi import WebSocket, WebSocketDisconnect

class ConnectionManager:
    def __init__(self):
        self.active_connections: list[WebSocket] = []

    async def connect(self, websocket: WebSocket):
        await websocket.accept()
        self.active_connections.append(websocket)

    def disconnect(self, websocket: WebSocket):
        self.active_connections.remove(websocket)

    async def broadcast(self, message: str):
        for connection in self.active_connections:
            await connection.send_text(message)

manager = ConnectionManager()

@app.websocket("/ws/{client_id}")
async def websocket_endpoint(websocket: WebSocket, client_id: str):
    await manager.connect(websocket)
    try:
        while True:
            data = await websocket.receive_text()
            await manager.broadcast(f"Client #{client_id}: {data}")
    except WebSocketDisconnect:
        manager.disconnect(websocket)
```
WebSockets enable persistent, full-duplex connections — used for chat apps, live notifications, and real-time dashboards. FastAPI's native WebSocket support comes from Starlette.

### Q46. How do you handle file uploads?
```python
from fastapi import File, UploadFile

@app.post("/upload/")
async def upload_file(file: UploadFile = File(...)):
    contents = await file.read()      # bytes, read asynchronously
    with open(f"uploads/{file.filename}", "wb") as f:
        f.write(contents)
    return {"filename": file.filename, "content_type": file.content_type}

@app.post("/upload-multiple/")
async def upload_multiple(files: list[UploadFile] = File(...)):
    return {"filenames": [f.filename for f in files]}
```
`UploadFile` (vs plain `bytes`) is preferred for larger files — it's backed by a spooled temporary file on disk (not fully loaded into memory) and provides async read methods, avoiding excessive memory usage.

---

## 10. Testing FastAPI Applications

### Q47. How do you write tests for FastAPI endpoints?
```python
# test_main.py
from fastapi.testclient import TestClient
from main import app

client = TestClient(app)

def test_read_root():
    response = client.get("/")
    assert response.status_code == 200
    assert response.json() == {"message": "Hello World"}

def test_create_item():
    response = client.post("/items/", json={"name": "Widget", "price": 9.99})
    assert response.status_code == 201
    data = response.json()
    assert data["name"] == "Widget"
```
`TestClient` (built on `httpx`) lets you test your app synchronously without running a live server — it calls into the ASGI app directly.

### Q48. How do you test async endpoints and use async test clients?
```python
import pytest
from httpx import AsyncClient, ASGITransport
from main import app

@pytest.mark.asyncio
async def test_async_endpoint():
    async with AsyncClient(transport=ASGITransport(app=app), base_url="http://test") as ac:
        response = await ac.get("/items/1")
    assert response.status_code == 200
```
Requires `pytest-asyncio`. Useful when your tests themselves need to `await` other async operations (e.g., seeding an async database directly).

### Q49. How do you test protected/authenticated endpoints?
```python
def test_protected_route():
    # obtain a token first
    login_resp = client.post("/token", data={"username": "test", "password": "test123"})
    token = login_resp.json()["access_token"]

    response = client.get(
        "/users/me",
        headers={"Authorization": f"Bearer {token}"}
    )
    assert response.status_code == 200
```

### Q50. How do you mock dependencies (like the database) in FastAPI tests?
```python
from main import app, get_db

def override_get_db():
    db = TestingSessionLocal()
    try:
        yield db
    finally:
        db.close()

app.dependency_overrides[get_db] = override_get_db

def test_with_test_db():
    response = client.post("/users/", json={"username": "alice"})
    assert response.status_code == 201
```
This uses FastAPI's `dependency_overrides` dict to swap real dependencies (production DB, external API clients, auth) with test doubles — no monkey-patching or complex mocking libraries required.

### Q51. What testing setup would you recommend for a real project (fixtures, isolated test DB)?
```python
# conftest.py
import pytest
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from main import app, Base, get_db
from fastapi.testclient import TestClient

SQLALCHEMY_TEST_DATABASE_URL = "sqlite:///./test.db"
engine = create_engine(SQLALCHEMY_TEST_DATABASE_URL, connect_args={"check_same_thread": False})
TestingSessionLocal = sessionmaker(bind=engine)

@pytest.fixture(scope="function")
def db_session():
    Base.metadata.create_all(bind=engine)
    session = TestingSessionLocal()
    yield session
    session.close()
    Base.metadata.drop_all(bind=engine)   # clean slate for every test

@pytest.fixture(scope="function")
def client(db_session):
    def override_get_db():
        yield db_session
    app.dependency_overrides[get_db] = override_get_db
    yield TestClient(app)
    app.dependency_overrides.clear()
```
Using a separate SQLite (or dedicated test Postgres) database per test run, with tables created/dropped per test, keeps tests isolated and repeatable.

---

## 11. Deployment & Production

### Q52. How do you run FastAPI in production?
```bash
# Simple: uvicorn with multiple workers
uvicorn main:app --host 0.0.0.0 --port 8000 --workers 4

# Recommended: gunicorn managing uvicorn worker processes (better process management)
gunicorn main:app -w 4 -k uvicorn.workers.UvicornWorker --bind 0.0.0.0:8000
```
`gunicorn` adds robust process management (worker restarts on crash, graceful reloads, pre-fork model) on top of uvicorn's ASGI serving — a common production combo, often placed behind an **nginx** reverse proxy that handles TLS termination, static files, and load balancing.

### Q53. How do you containerize a FastAPI app with Docker?
```dockerfile
FROM python:3.12-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

EXPOSE 8000
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
```
```bash
docker build -t myapp .
docker run -p 8000:8000 myapp
```

### Q54. How many workers should you run, and how do you decide?
A common rule of thumb: `workers = (2 x CPU cores) + 1` for CPU-bound-ish workloads, though for I/O-bound async apps, fewer workers (each handling many concurrent connections via the event loop) can suffice — always benchmark under realistic load rather than trusting a blanket formula. Container orchestrators (Kubernetes) often run **one worker per container** and scale horizontally with more container replicas instead, which simplifies resource limits and rolling deployments.

### Q55. How do you set up environment-specific configuration for dev/staging/production?
```python
from pydantic_settings import BaseSettings
from functools import lru_cache

class Settings(BaseSettings):
    environment: str = "development"
    database_url: str
    debug: bool = False

    class Config:
        env_file = ".env"

@lru_cache
def get_settings():
    return Settings()      # cached singleton, read once per process

@app.get("/info")
async def info(settings: Settings = Depends(get_settings)):
    return {"env": settings.environment}
```
`@lru_cache` ensures settings are parsed from environment variables only once per process, not on every request — combined with `.env` files per environment (`.env.dev`, `.env.prod`) or actual environment variables injected by the deployment platform.

### Q56. How do you add health checks and observability?
```python
@app.get("/health")
async def health_check():
    return {"status": "ok"}

@app.get("/health/db")
async def db_health(db: Session = Depends(get_db)):
    try:
        db.execute(text("SELECT 1"))
        return {"database": "connected"}
    except Exception:
        raise HTTPException(status_code=503, detail="Database unavailable")
```
Health check endpoints are used by load balancers/orchestrators (Kubernetes liveness/readiness probes) to determine if an instance should receive traffic. For deeper observability, integrate structured logging, `prometheus-fastapi-instrumentator` for metrics, and distributed tracing (OpenTelemetry) in larger systems.

---

## 12. Advanced / Architecture Questions

### Q57. How do you version a FastAPI API?
```python
# URL-path versioning (most common, explicit)
from fastapi import APIRouter

v1_router = APIRouter(prefix="/api/v1")
v2_router = APIRouter(prefix="/api/v2")

@v1_router.get("/items/")
async def list_items_v1():
    return {"version": 1, "items": []}

@v2_router.get("/items/")
async def list_items_v2():
    return {"version": 2, "items": [], "meta": {}}

app.include_router(v1_router)
app.include_router(v2_router)
```
Alternatives: header-based versioning (`Accept: application/vnd.myapi.v2+json`) or separate FastAPI sub-apps mounted at different paths (`app.mount("/v2", v2_app)`). URL-path versioning is simplest and most discoverable for API consumers.

### Q58. How do you structure a large, production-grade FastAPI project?
```
myapp/
├── app/
│   ├── main.py                 # FastAPI() instance, includes routers
│   ├── core/
│   │   ├── config.py            # Settings (BaseSettings)
│   │   └── security.py          # password hashing, JWT helpers
│   ├── api/
│   │   ├── deps.py               # shared dependencies (get_db, get_current_user)
│   │   └── v1/
│   │       ├── users.py           # APIRouter for /users
│   │       └── items.py           # APIRouter for /items
│   ├── models/                  # SQLAlchemy ORM models
│   ├── schemas/                 # Pydantic request/response models
│   ├── crud/                     # DB access functions (separate from route logic)
│   └── db/
│       └── session.py            # engine, SessionLocal
├── tests/
├── alembic/                      # migrations
├── requirements.txt / pyproject.toml
└── Dockerfile
```
Separating **schemas** (Pydantic, API contract) from **models** (SQLAlchemy, DB schema) from **crud** (DB access logic) from **routers** (HTTP layer) keeps concerns cleanly separated and testable in isolation — a common "layered architecture" pattern.

### Q59. What is the "N+1 query problem" and how do you address it in an async SQLAlchemy + FastAPI context?
```python
# BAD: triggers one query per author while serializing (N+1)
@app.get("/articles")
async def list_articles(db: AsyncSession = Depends(get_db)):
    result = await db.execute(select(Article))
    articles = result.scalars().all()
    return [{"title": a.title, "author": a.author.name} for a in articles]  # lazy-loads author each time

# GOOD: eager load with selectinload/joinedload
from sqlalchemy.orm import selectinload

@app.get("/articles")
async def list_articles(db: AsyncSession = Depends(get_db)):
    result = await db.execute(select(Article).options(selectinload(Article.author)))
    articles = result.scalars().all()
    return [{"title": a.title, "author": a.author.name} for a in articles]  # single extra query
```

### Q60. How do you implement pagination properly?
```python
from pydantic import BaseModel

class PaginatedResponse(BaseModel):
    items: list[ItemSchema]
    total: int
    page: int
    page_size: int

@app.get("/items/", response_model=PaginatedResponse)
async def list_items(
    page: int = Query(1, ge=1),
    page_size: int = Query(20, ge=1, le=100),
    db: Session = Depends(get_db),
):
    total = db.query(Item).count()
    items = db.query(Item).offset((page - 1) * page_size).limit(page_size).all()
    return {"items": items, "total": total, "page": page, "page_size": page_size}
```
For very large datasets, **cursor-based pagination** (using an indexed column like `id` or `created_at` as a cursor instead of `OFFSET`) scales better than offset/limit, which gets slower as the offset grows since the DB must still scan/skip prior rows.

### Q61. How do you rate-limit a FastAPI endpoint?
```python
from slowapi import Limiter
from slowapi.util import get_remote_address

limiter = Limiter(key_func=get_remote_address)
app.state.limiter = limiter

@app.post("/login")
@limiter.limit("5/minute")
async def login(request: Request, ...):
    ...
```
Rate limiting protects against brute-force attacks and abuse; for distributed multi-instance deployments, back the limiter with Redis rather than in-memory state so limits are enforced consistently across instances.

### Q62. What's the difference between `Response` model filtering and manually constructing dicts, in terms of correctness/performance?
Using `response_model` lets FastAPI validate and filter the output declaratively (single source of truth for your API contract, reflected in OpenAPI docs). Manually building dicts is more error-prone (easy to accidentally leak a sensitive field) and doesn't self-document in Swagger. The tradeoff is a small serialization overhead from re-validating output through Pydantic — usually negligible, and can be tuned via `response_model_exclude_unset`, `response_model_exclude_none`, etc. for finer control.

### Q63. How would you explain FastAPI's OpenAPI generation to a non-technical stakeholder, and why does it matter for a team?
FastAPI reads your code — the type hints, Pydantic models, and route definitions — and automatically produces a standardized, machine-readable contract (OpenAPI/Swagger) describing every endpoint, its inputs, and outputs. This means the documentation **can never drift out of sync with the actual code** (unlike hand-written docs), frontend teams can generate typed API clients directly from it, and QA/API-testing tools can consume the spec automatically — reducing miscommunication and integration bugs across teams.

---

# Part B — Complete Theory

## 13. FastAPI Theoretical Deep Dive

This section consolidates the conceptual foundations of FastAPI into one reference — useful for building genuine understanding beyond memorized interview answers.

### 13.1 The Technology Stack

```
┌─────────────────────────────────────┐
│              FastAPI                 │   <- validation, DI, OpenAPI, routing sugar
├───────────────────────┬───────────────┤
│      Starlette         │   Pydantic     │   <- ASGI toolkit    <- data validation/serialization
├───────────────────────┴───────────────┤
│                 ASGI spec               │   <- the async interface standard
├───────────────────────────────────────┤
│         uvicorn (ASGI server)           │   <- uvloop + httptools under the hood
├───────────────────────────────────────┤
│    Operating System / Event Loop        │
└───────────────────────────────────────┘
```
- **ASGI** is a *specification*, not a library — it defines how an async Python web application communicates with a server (analogous to how WSGI worked for sync apps, but supporting async, WebSockets, and long-lived connections).
- **uvicorn** is the reference ASGI server that actually listens on a socket, parses HTTP, and invokes your ASGI application (FastAPI) for each connection.
- **Starlette** implements the ASGI application interface, routing, middleware chain, and low-level `Request`/`Response`/`WebSocket` primitives.
- **FastAPI** sits on top, adding the developer-facing ergonomics: type-hint-driven validation, automatic docs, and the dependency injection system.

### 13.2 The Request Lifecycle (What Actually Happens)

1. A TCP connection arrives at uvicorn; uvicorn's HTTP parser (`httptools`) parses the raw bytes into an HTTP request.
2. Uvicorn constructs an ASGI `scope` (a dict describing the connection: method, path, headers, etc.) and calls into the Starlette/FastAPI ASGI application.
3. The request passes through the **middleware stack** (outermost to innermost) — e.g., CORS middleware, custom logging middleware.
4. Starlette's router matches the path to a registered route (path operation).
5. FastAPI resolves the function signature: extracts path/query params, parses/validates the request body against any Pydantic model, and **resolves the dependency tree** (recursively satisfying every `Depends()`, including nested dependencies, with caching per request).
6. Your path operation function executes (either directly on the event loop if `async def`, or in a thread pool if plain `def`).
7. The return value is validated/serialized against `response_model` (if declared) and converted to JSON (or another response type).
8. The response passes back out through the middleware stack (innermost to outermost).
9. Uvicorn sends the HTTP response bytes back over the socket.

### 13.3 Type Hints as the Single Source of Truth

FastAPI's central design philosophy: **the same type hints you'd write anyway (for editor autocompletion and self-documentation) are reused as the mechanism for validation, serialization, and documentation** — you don't maintain a separate schema definition file.

```python
async def create_item(item: Item) -> ItemOut:
    ...
```
This one signature simultaneously tells FastAPI: (a) parse and validate the JSON body as an `Item`, (b) the OpenAPI schema for the request body is `Item`'s JSON Schema, (c) the response should be validated/filtered against `ItemOut`, and (d) your editor should autocomplete `item.` with `Item`'s fields.

### 13.4 The Dependency Injection Graph

FastAPI's `Depends()` system builds a **directed acyclic graph** of dependencies per request. Dependencies can depend on other dependencies, and FastAPI resolves them bottom-up, executing each **at most once per request** (unless `use_cache=False`). This is conceptually similar to dependency injection frameworks in other ecosystems (Spring in Java, Angular's DI) but implemented purely through plain function calls and type hints — no XML config or decorators-heavy magic beyond `Depends()` itself.

### 13.5 Sync vs Async Execution Model

FastAPI inspects whether your path operation (and each dependency) is a coroutine function. Plain `def` functions are automatically dispatched to an external thread pool (`anyio`'s `to_thread.run_sync` under the hood via Starlette) so they don't block the event loop — this is what allows FastAPI to mix synchronous legacy code with async code safely, at the cost of thread-pool context-switch overhead. This is a deliberate design tradeoff: FastAPI optimizes for developer flexibility (mixing sync/async freely) rather than forcing an all-or-nothing async architecture.

### 13.6 OpenAPI & Automatic Documentation

FastAPI generates an OpenAPI 3.x JSON schema (`/openapi.json`) by introspecting every route's parameters, request body model, response model, and status codes. This schema powers two built-in UIs:
- **Swagger UI** (`/docs`) — interactive, lets you execute requests directly from the browser.
- **ReDoc** (`/redoc`) — clean, reference-style documentation.

Because the schema is derived from live code (not hand-maintained), it's guaranteed accurate as long as your type hints are accurate — a major reliability advantage over manually written API docs.

### 13.7 Pydantic's Role: Validation, Serialization, and Schema Generation
Pydantic models serve three distinct purposes simultaneously:
1. **Validation** — checking incoming data matches expected types/constraints, raising clear errors otherwise.
2. **Serialization** — converting Python objects (including ORM model instances) into JSON-compatible structures for the response.
3. **Schema generation** — Pydantic models can emit JSON Schema, which FastAPI feeds into the OpenAPI spec.

Since Pydantic v2, the core validation/serialization logic runs in a compiled Rust extension (`pydantic-core`), making this validation layer fast enough to not be a meaningful bottleneck compared to raw dict-based approaches, while still providing strong guarantees.

### 13.8 Where FastAPI Fits in the Broader Ecosystem
- **vs Flask**: Flask is synchronous-first and unopinionated about validation; FastAPI is async-native and validation-first via type hints.
- **vs Django**: Django is a full-stack framework (ORM, admin, templating, auth all built-in); FastAPI is API-focused and composes with whichever ORM/auth libraries you choose.
- **vs Node.js/Express**: FastAPI offers comparable raw throughput for I/O-bound workloads (thanks to ASGI + uvloop) while keeping Python's ecosystem (data science, ML model serving, type safety via Pydantic) — a common reason FastAPI is popular for serving ML models and building modern microservices.

---

# Part C — Full Tutorial

## 14. Complete Tutorial: Building a Production-Style Web App

We'll build a **Task Manager API** — a realistic app with user registration, JWT login, CRUD for tasks scoped to each user, database persistence via SQLAlchemy, and tests. This mirrors real production patterns rather than a toy example.

### 14.1 Project Setup

```bash
mkdir taskmanager && cd taskmanager
python -m venv venv
source venv/bin/activate        # Windows: venv\Scripts\activate

pip install fastapi "uvicorn[standard]" sqlalchemy pydantic-settings \
            "passlib[bcrypt]" "python-jose[cryptography]" python-multipart \
            pytest httpx

pip freeze > requirements.txt
```

Project structure we'll build:
```
taskmanager/
├── app/
│   ├── __init__.py
│   ├── main.py
│   ├── database.py
│   ├── models.py
│   ├── schemas.py
│   ├── security.py
│   ├── deps.py
│   └── routers/
│       ├── __init__.py
│       ├── auth.py
│       └── tasks.py
├── tests/
│   ├── __init__.py
│   ├── conftest.py
│   └── test_tasks.py
├── requirements.txt
└── .env
```

### 14.2 Configuration & Database Setup

```python
# app/database.py
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, declarative_base

SQLALCHEMY_DATABASE_URL = "sqlite:///./taskmanager.db"
# For Postgres in production: "postgresql://user:pass@localhost/taskmanager"

engine = create_engine(
    SQLALCHEMY_DATABASE_URL,
    connect_args={"check_same_thread": False},   # needed only for SQLite
)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
```

### 14.3 Database Models

```python
# app/models.py
from sqlalchemy import Column, Integer, String, Boolean, ForeignKey, DateTime
from sqlalchemy.orm import relationship
from sqlalchemy.sql import func
from app.database import Base

class User(Base):
    __tablename__ = "users"

    id = Column(Integer, primary_key=True, index=True)
    username = Column(String, unique=True, index=True, nullable=False)
    email = Column(String, unique=True, index=True, nullable=False)
    hashed_password = Column(String, nullable=False)
    created_at = Column(DateTime(timezone=True), server_default=func.now())

    tasks = relationship("Task", back_populates="owner", cascade="all, delete-orphan")

class Task(Base):
    __tablename__ = "tasks"

    id = Column(Integer, primary_key=True, index=True)
    title = Column(String, nullable=False)
    description = Column(String, nullable=True)
    is_completed = Column(Boolean, default=False)
    owner_id = Column(Integer, ForeignKey("users.id"), nullable=False)
    created_at = Column(DateTime(timezone=True), server_default=func.now())

    owner = relationship("User", back_populates="tasks")
```

### 14.4 Pydantic Schemas (API Contracts)

```python
# app/schemas.py
from pydantic import BaseModel, EmailStr, ConfigDict
from datetime import datetime
from typing import Optional

# --- User schemas ---
class UserCreate(BaseModel):
    username: str
    email: EmailStr
    password: str

class UserOut(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    id: int
    username: str
    email: EmailStr
    created_at: datetime

# --- Auth schemas ---
class Token(BaseModel):
    access_token: str
    token_type: str = "bearer"

class TokenData(BaseModel):
    username: Optional[str] = None

# --- Task schemas ---
class TaskCreate(BaseModel):
    title: str
    description: Optional[str] = None

class TaskUpdate(BaseModel):
    title: Optional[str] = None
    description: Optional[str] = None
    is_completed: Optional[bool] = None

class TaskOut(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    id: int
    title: str
    description: Optional[str]
    is_completed: bool
    created_at: datetime
```

### 14.5 Security Utilities (Password Hashing + JWT)

```python
# app/security.py
from passlib.context import CryptContext
from jose import jwt
from datetime import datetime, timedelta
import os

SECRET_KEY = os.getenv("SECRET_KEY", "dev-secret-change-in-production")
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 60

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

def hash_password(password: str) -> str:
    return pwd_context.hash(password)

def verify_password(plain_password: str, hashed_password: str) -> bool:
    return pwd_context.verify(plain_password, hashed_password)

def create_access_token(data: dict, expires_delta: timedelta | None = None) -> str:
    to_encode = data.copy()
    expire = datetime.utcnow() + (expires_delta or timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES))
    to_encode.update({"exp": expire})
    return jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)
```

### 14.6 Shared Dependencies (Auth Guard)

```python
# app/deps.py
from fastapi import Depends, HTTPException, status
from fastapi.security import OAuth2PasswordBearer
from sqlalchemy.orm import Session
from jose import JWTError, jwt

from app.database import get_db
from app.security import SECRET_KEY, ALGORITHM
from app import models

oauth2_scheme = OAuth2PasswordBearer(tokenUrl="/auth/token")

def get_current_user(
    token: str = Depends(oauth2_scheme),
    db: Session = Depends(get_db),
) -> models.User:
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
        username: str = payload.get("sub")
        if username is None:
            raise credentials_exception
    except JWTError:
        raise credentials_exception

    user = db.query(models.User).filter(models.User.username == username).first()
    if user is None:
        raise credentials_exception
    return user
```

### 14.7 Auth Router (Register + Login)

```python
# app/routers/auth.py
from fastapi import APIRouter, Depends, HTTPException, status
from fastapi.security import OAuth2PasswordRequestForm
from sqlalchemy.orm import Session

from app.database import get_db
from app import models, schemas
from app.security import hash_password, verify_password, create_access_token

router = APIRouter(prefix="/auth", tags=["auth"])

@router.post("/register", response_model=schemas.UserOut, status_code=status.HTTP_201_CREATED)
def register(user: schemas.UserCreate, db: Session = Depends(get_db)):
    existing = db.query(models.User).filter(
        (models.User.username == user.username) | (models.User.email == user.email)
    ).first()
    if existing:
        raise HTTPException(status_code=400, detail="Username or email already registered")

    new_user = models.User(
        username=user.username,
        email=user.email,
        hashed_password=hash_password(user.password),
    )
    db.add(new_user)
    db.commit()
    db.refresh(new_user)
    return new_user

@router.post("/token", response_model=schemas.Token)
def login(form_data: OAuth2PasswordRequestForm = Depends(), db: Session = Depends(get_db)):
    user = db.query(models.User).filter(models.User.username == form_data.username).first()
    if not user or not verify_password(form_data.password, user.hashed_password):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect username or password",
            headers={"WWW-Authenticate": "Bearer"},
        )
    access_token = create_access_token(data={"sub": user.username})
    return {"access_token": access_token, "token_type": "bearer"}
```

### 14.8 Tasks Router (Protected CRUD, Scoped Per User)

```python
# app/routers/tasks.py
from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.orm import Session
from typing import List

from app.database import get_db
from app.deps import get_current_user
from app import models, schemas

router = APIRouter(prefix="/tasks", tags=["tasks"])

@router.post("/", response_model=schemas.TaskOut, status_code=status.HTTP_201_CREATED)
def create_task(
    task: schemas.TaskCreate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
):
    new_task = models.Task(**task.model_dump(), owner_id=current_user.id)
    db.add(new_task)
    db.commit()
    db.refresh(new_task)
    return new_task

@router.get("/", response_model=List[schemas.TaskOut])
def list_tasks(
    skip: int = 0,
    limit: int = 20,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
):
    return (
        db.query(models.Task)
        .filter(models.Task.owner_id == current_user.id)
        .offset(skip)
        .limit(limit)
        .all()
    )

def _get_owned_task_or_404(task_id: int, db: Session, current_user: models.User) -> models.Task:
    task = db.query(models.Task).filter(
        models.Task.id == task_id, models.Task.owner_id == current_user.id
    ).first()
    if not task:
        raise HTTPException(status_code=404, detail="Task not found")
    return task

@router.get("/{task_id}", response_model=schemas.TaskOut)
def get_task(task_id: int, db: Session = Depends(get_db),
             current_user: models.User = Depends(get_current_user)):
    return _get_owned_task_or_404(task_id, db, current_user)

@router.patch("/{task_id}", response_model=schemas.TaskOut)
def update_task(
    task_id: int,
    task_update: schemas.TaskUpdate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
):
    task = _get_owned_task_or_404(task_id, db, current_user)
    for field, value in task_update.model_dump(exclude_unset=True).items():
        setattr(task, field, value)
    db.commit()
    db.refresh(task)
    return task

@router.delete("/{task_id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_task(task_id: int, db: Session = Depends(get_db),
                 current_user: models.User = Depends(get_current_user)):
    task = _get_owned_task_or_404(task_id, db, current_user)
    db.delete(task)
    db.commit()
    return None
```

### 14.9 Wiring It All Together

```python
# app/main.py
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.database import Base, engine
from app.routers import auth, tasks

Base.metadata.create_all(bind=engine)   # creates tables if they don't exist
                                          # (use Alembic migrations in real production instead)

app = FastAPI(
    title="Task Manager API",
    description="A production-style task manager built with FastAPI",
    version="1.0.0",
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(auth.router)
app.include_router(tasks.router)

@app.get("/health", tags=["health"])
async def health_check():
    return {"status": "ok"}
```

### 14.10 Running the App

```bash
uvicorn app.main:app --reload
```
Visit:
- `http://127.0.0.1:8000/docs` — interactive Swagger UI (try requests directly!)
- `http://127.0.0.1:8000/redoc` — reference documentation

**Manual walkthrough using `curl`:**
```bash
# 1. Register a user
curl -X POST http://127.0.0.1:8000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "email": "alice@example.com", "password": "secret123"}'

# 2. Log in to get a token
curl -X POST http://127.0.0.1:8000/auth/token \
  -F "username=alice" -F "password=secret123"
# -> {"access_token": "eyJ...", "token_type": "bearer"}

# 3. Create a task (using the token)
curl -X POST http://127.0.0.1:8000/tasks/ \
  -H "Authorization: Bearer eyJ..." \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy groceries", "description": "Milk, eggs, bread"}'

# 4. List your tasks
curl http://127.0.0.1:8000/tasks/ -H "Authorization: Bearer eyJ..."
```

### 14.11 Writing Tests

```python
# tests/conftest.py
import pytest
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from fastapi.testclient import TestClient

from app.main import app
from app.database import Base, get_db

TEST_DB_URL = "sqlite:///./test.db"
engine = create_engine(TEST_DB_URL, connect_args={"check_same_thread": False})
TestingSessionLocal = sessionmaker(bind=engine)

@pytest.fixture(scope="function")
def db_session():
    Base.metadata.create_all(bind=engine)
    session = TestingSessionLocal()
    yield session
    session.close()
    Base.metadata.drop_all(bind=engine)

@pytest.fixture(scope="function")
def client(db_session):
    def override_get_db():
        yield db_session
    app.dependency_overrides[get_db] = override_get_db
    with TestClient(app) as c:
        yield c
    app.dependency_overrides.clear()

@pytest.fixture
def auth_headers(client):
    client.post("/auth/register", json={
        "username": "testuser", "email": "test@example.com", "password": "pass1234"
    })
    resp = client.post("/auth/token", data={"username": "testuser", "password": "pass1234"})
    token = resp.json()["access_token"]
    return {"Authorization": f"Bearer {token}"}
```

```python
# tests/test_tasks.py
def test_register_and_login(client):
    resp = client.post("/auth/register", json={
        "username": "bob", "email": "bob@example.com", "password": "bobpass1"
    })
    assert resp.status_code == 201
    assert resp.json()["username"] == "bob"

    login_resp = client.post("/auth/token", data={"username": "bob", "password": "bobpass1"})
    assert login_resp.status_code == 200
    assert "access_token" in login_resp.json()

def test_create_and_list_tasks(client, auth_headers):
    create_resp = client.post("/tasks/", json={"title": "Test Task"}, headers=auth_headers)
    assert create_resp.status_code == 201
    assert create_resp.json()["title"] == "Test Task"

    list_resp = client.get("/tasks/", headers=auth_headers)
    assert list_resp.status_code == 200
    assert len(list_resp.json()) == 1

def test_cannot_access_tasks_without_auth(client):
    resp = client.get("/tasks/")
    assert resp.status_code == 401

def test_update_and_delete_task(client, auth_headers):
    create_resp = client.post("/tasks/", json={"title": "To Update"}, headers=auth_headers)
    task_id = create_resp.json()["id"]

    update_resp = client.patch(
        f"/tasks/{task_id}", json={"is_completed": True}, headers=auth_headers
    )
    assert update_resp.status_code == 200
    assert update_resp.json()["is_completed"] is True

    delete_resp = client.delete(f"/tasks/{task_id}", headers=auth_headers)
    assert delete_resp.status_code == 204

    get_resp = client.get(f"/tasks/{task_id}", headers=auth_headers)
    assert get_resp.status_code == 404
```
```bash
pytest -v
```

### 14.12 Taking It Further (Production Checklist)

To evolve this into a real production app, layer in:
1. **Alembic migrations** instead of `Base.metadata.create_all()`.
2. **Async SQLAlchemy + asyncpg** for higher concurrency under load.
3. **Environment-based settings** via `pydantic_settings.BaseSettings` (`.env` for dev, real env vars in prod), instead of the hardcoded `SECRET_KEY` fallback shown above.
4. **Refresh tokens** alongside short-lived access tokens for better session security.
5. **Rate limiting** on `/auth/token` to prevent brute-force login attempts.
6. **Structured logging** (`structlog` or Python's `logging` with JSON formatting) for observability.
7. **Dockerfile + docker-compose** (app + Postgres + Redis) for consistent environments.
8. **CI pipeline** (GitHub Actions) running `pytest`, `ruff`, and `mypy` on every push.
9. **Pagination, filtering, and sorting** query parameters on the `GET /tasks/` list endpoint for scale.
10. **Global exception handlers** for consistent error payloads across the whole API (see Q32/Q33 above).

This tutorial covers the same architectural shape used in real FastAPI production services: layered structure (routers → schemas → models → security), JWT auth, dependency-injected DB sessions, and a working test suite with dependency overrides — the exact patterns interviewers expect you to be able to discuss and reproduce.

