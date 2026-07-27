# Rate Limiter — LLD

## 1. Requirements

**Functional**
- Limit number of requests a client (user/IP/API-key) can make within a time window.
- Support multiple algorithms: Token Bucket, Leaky Bucket, Fixed Window, Sliding Window Log, Sliding Window Counter.
- Different rules per client, per API endpoint, or per user tier (e.g., free vs premium).
- Reject requests over the limit (HTTP 429) or queue them, depending on algorithm.
- Pluggable — new algorithms can be added without touching existing code.

**Non-functional**
- Thread-safe (concurrent requests hitting the same client bucket).
- Low latency — decision must be O(1) or close to it.
- Extensible to distributed mode (multi-server, backed by Redis) without redesigning the API.

---

## 2. Key design decisions & patterns used

| Pattern | Where | Why |
|---|---|---|
| **Strategy** | `RateLimitingAlgorithm` interface with `TokenBucket`, `LeakyBucket`, `FixedWindow`, `SlidingWindowLog` implementations | The "how do I decide to allow/deny" logic varies independently of everything else. Strategy lets the client swap algorithms at runtime/config without changing calling code. |
| **Factory Method** | `RateLimiterAlgorithmFactory` | Decouples algorithm *creation* (which needs config-specific constructor args) from algorithm *usage*. Adding a new algorithm = new class + one factory branch, nothing else changes. |
| **Singleton** | `RateLimiterRegistry` | One central place tracks all active limiters (per client/endpoint) across the app. Avoids duplicate state and gives a single point to plug in distributed storage later. |
| **Decorator / Proxy** | `RateLimitedInterceptor` wrapping the actual request handler | Rate limiting is a cross-cutting concern. Wrapping the handler keeps the business logic class untouched (Single Responsibility) — the interceptor adds behavior without modifying it. |
| **Builder** | `RateLimitConfig.Builder` | Config has many optional parameters (capacity, refillRate, windowSize, burst). Builder avoids telescoping constructors. |
| **Composite-ish rule resolution** | `RateLimitRule` + `RuleResolver` | Allows layered rules: global default → per-tier → per-client override, resolved in priority order. |

**SOLID mapping**
- **S**: Each algorithm class does one thing — decide allow/deny for its strategy. Config, storage, and rule-resolution are separate classes.
- **O**: New algorithm → implement interface + register in factory. No existing class is modified.
- **L**: Any `RateLimitingAlgorithm` implementation is substitutable wherever the interface is expected — same contract (`boolean tryAcquire(String key)`).
- **I**: `RateLimitingAlgorithm` exposes only `tryAcquire`; storage backends implement a narrow `RateLimiterStore` interface rather than one bloated interface.
- **D**: `RateLimiter` (context) depends on the `RateLimitingAlgorithm` abstraction, not concrete classes. Storage is also injected as an interface (`RateLimiterStore`), so swapping in-memory → Redis needs no change to algorithm logic.

---

## 3. Class Diagram (textual)

```
                     ┌────────────────────────┐
                     │  RateLimitingAlgorithm  │  (Strategy interface)
                     │  + tryAcquire(key): bool│
                     └───────────▲─────────────┘
                                  │
        ┌───────────────┬────────┼─────────────────┬───────────────────┐
        │                │                          │                   │
┌───────────────┐ ┌──────────────┐  ┌───────────────────────┐ ┌──────────────────┐
│ TokenBucket    │ │ LeakyBucket  │  │ FixedWindowCounter     │ │ SlidingWindowLog  │
│ RateLimiter    │ │ RateLimiter  │  │ RateLimiter             │ │ RateLimiter       │
└───────────────┘ └──────────────┘  └───────────────────────┘ └──────────────────┘

┌───────────────────────┐        ┌───────────────────────┐
│ RateLimiterAlgorithm   │──────▶│  RateLimitConfig        │
│ Factory (Factory Method)│       │  (Builder)              │
└───────────────────────┘        └───────────────────────┘

┌────────────────────┐   uses   ┌──────────────────────────┐
│ RateLimiter (Context)│───────▶│ RateLimitingAlgorithm     │
│ + isAllowed(key)     │        └──────────────────────────┘
└─────────▲───────────┘
          │ managed by
┌─────────┴────────────┐        ┌──────────────────────┐
│ RateLimiterRegistry   │◀──────▶│ RateLimiterStore      │ (interface)
│ (Singleton)           │        │ InMemoryStore / Redis │
└───────────────────────┘        └──────────────────────┘

┌───────────────────────┐
│ RateLimitRule          │  (endpoint / tier / client → config)
└───────────────────────┘

┌───────────────────────┐
│ RateLimitedInterceptor │  (Decorator around request handler)
└───────────────────────┘
```

---

## 4. Code (Java)

### 4.1 Strategy interface

```java
public interface RateLimitingAlgorithm {
    /**
     * @param key unique client identifier (userId, apiKey, IP)
     * @return true if request is allowed, false if it should be rejected
     */
    boolean tryAcquire(String key);
}
```

### 4.2 Config (Builder pattern)

```java
public final class RateLimitConfig {
    private final AlgorithmType type;
    private final int capacity;        // bucket size / max requests
    private final int refillTokens;    // tokens added per interval (token bucket)
    private final long refillIntervalMs;
    private final long windowSizeMs;   // fixed/sliding window size

    private RateLimitConfig(Builder b) {
        this.type = b.type;
        this.capacity = b.capacity;
        this.refillTokens = b.refillTokens;
        this.refillIntervalMs = b.refillIntervalMs;
        this.windowSizeMs = b.windowSizeMs;
    }

    // getters omitted for brevity

    public static class Builder {
        private AlgorithmType type = AlgorithmType.TOKEN_BUCKET;
        private int capacity = 10;
        private int refillTokens = 1;
        private long refillIntervalMs = 1000;
        private long windowSizeMs = 1000;

        public Builder type(AlgorithmType t) { this.type = t; return this; }
        public Builder capacity(int c) { this.capacity = c; return this; }
        public Builder refillTokens(int r) { this.refillTokens = r; return this; }
        public Builder refillIntervalMs(long ms) { this.refillIntervalMs = ms; return this; }
        public Builder windowSizeMs(long ms) { this.windowSizeMs = ms; return this; }
        public RateLimitConfig build() { return new RateLimitConfig(this); }
    }
}

public enum AlgorithmType {
    TOKEN_BUCKET, LEAKY_BUCKET, FIXED_WINDOW, SLIDING_WINDOW_LOG, SLIDING_WINDOW_COUNTER
}
```

### 4.3 Token Bucket implementation (most common in practice)

```java
public class TokenBucketRateLimiter implements RateLimitingAlgorithm {

    private static class Bucket {
        double tokens;
        long lastRefillTimestamp;
        final Object lock = new Object();
    }

    private final RateLimitConfig config;
    private final ConcurrentHashMap<String, Bucket> buckets = new ConcurrentHashMap<>();

    public TokenBucketRateLimiter(RateLimitConfig config) {
        this.config = config;
    }

    @Override
    public boolean tryAcquire(String key) {
        Bucket bucket = buckets.computeIfAbsent(key, k -> {
            Bucket b = new Bucket();
            b.tokens = config.getCapacity();
            b.lastRefillTimestamp = System.currentTimeMillis();
            return b;
        });

        synchronized (bucket.lock) {
            refill(bucket);
            if (bucket.tokens >= 1) {
                bucket.tokens -= 1;
                return true;
            }
            return false;
        }
    }

    private void refill(Bucket bucket) {
        long now = System.currentTimeMillis();
        long elapsed = now - bucket.lastRefillTimestamp;
        if (elapsed <= 0) return;

        double tokensToAdd = (elapsed / (double) config.getRefillIntervalMs()) * config.getRefillTokens();
        if (tokensToAdd > 0) {
            bucket.tokens = Math.min(config.getCapacity(), bucket.tokens + tokensToAdd);
            bucket.lastRefillTimestamp = now;
        }
    }
}
```

### 4.4 Fixed Window Counter (simplest, has burst-at-boundary issue — mention as tradeoff)

```java
public class FixedWindowCounterRateLimiter implements RateLimitingAlgorithm {

    private static class Window {
        long windowStart;
        AtomicInteger count = new AtomicInteger(0);
    }

    private final RateLimitConfig config;
    private final ConcurrentHashMap<String, Window> windows = new ConcurrentHashMap<>();

    public FixedWindowCounterRateLimiter(RateLimitConfig config) {
        this.config = config;
    }

    @Override
    public boolean tryAcquire(String key) {
        long now = System.currentTimeMillis();
        long currentWindowStart = now - (now % config.getWindowSizeMs());

        Window window = windows.compute(key, (k, w) -> {
            if (w == null || w.windowStart != currentWindowStart) {
                Window nw = new Window();
                nw.windowStart = currentWindowStart;
                return nw;
            }
            return w;
        });

        return window.count.incrementAndGet() <= config.getCapacity();
    }
}
```

### 4.5 Sliding Window Log (accurate, more memory)

```java
public class SlidingWindowLogRateLimiter implements RateLimitingAlgorithm {

    private final RateLimitConfig config;
    private final ConcurrentHashMap<String, Deque<Long>> logs = new ConcurrentHashMap<>();

    public SlidingWindowLogRateLimiter(RateLimitConfig config) {
        this.config = config;
    }

    @Override
    public boolean tryAcquire(String key) {
        long now = System.currentTimeMillis();
        Deque<Long> timestamps = logs.computeIfAbsent(key, k -> new ArrayDeque<>());

        synchronized (timestamps) {
            long windowStart = now - config.getWindowSizeMs();
            while (!timestamps.isEmpty() && timestamps.peekFirst() < windowStart) {
                timestamps.pollFirst();
            }
            if (timestamps.size() < config.getCapacity()) {
                timestamps.addLast(now);
                return true;
            }
            return false;
        }
    }
}
```

### 4.6 Factory Method

```java
public class RateLimiterAlgorithmFactory {
    public static RateLimitingAlgorithm create(RateLimitConfig config) {
        switch (config.getType()) {
            case TOKEN_BUCKET:
                return new TokenBucketRateLimiter(config);
            case FIXED_WINDOW:
                return new FixedWindowCounterRateLimiter(config);
            case SLIDING_WINDOW_LOG:
                return new SlidingWindowLogRateLimiter(config);
            case LEAKY_BUCKET:
                return new LeakyBucketRateLimiter(config); // similar structure, omitted
            default:
                throw new IllegalArgumentException("Unsupported algorithm: " + config.getType());
        }
    }
}
```

### 4.7 Context class (what callers actually use)

```java
public class RateLimiter {
    private final RateLimitingAlgorithm algorithm;

    public RateLimiter(RateLimitConfig config) {
        this.algorithm = RateLimiterAlgorithmFactory.create(config);
    }

    public boolean isAllowed(String clientKey) {
        return algorithm.tryAcquire(clientKey);
    }
}
```

Notice: the caller never knows or cares which concrete algorithm runs — pure Strategy pattern benefit, and it satisfies **Dependency Inversion** (context depends on the interface, not concretions).

### 4.8 Rule resolution — different limits per endpoint/tier

```java
public class RateLimitRule {
    private final String endpoint;      // e.g. "/api/orders"
    private final UserTier tier;        // FREE, PREMIUM
    private final RateLimitConfig config;

    public RateLimitRule(String endpoint, UserTier tier, RateLimitConfig config) {
        this.endpoint = endpoint;
        this.tier = tier;
        this.config = config;
    }
    // getters omitted
}

public enum UserTier { FREE, PREMIUM, ADMIN }

public class RuleResolver {
    private final List<RateLimitRule> rules;

    public RuleResolver(List<RateLimitRule> rules) {
        this.rules = rules;
    }

    public RateLimitConfig resolve(String endpoint, UserTier tier) {
        return rules.stream()
                .filter(r -> r.getEndpoint().equals(endpoint) && r.getTier() == tier)
                .findFirst()
                .map(RateLimitRule::getConfig)
                .orElseGet(this::defaultConfig);
    }

    private RateLimitConfig defaultConfig() {
        return new RateLimitConfig.Builder().capacity(100).build();
    }
}
```

### 4.9 Registry (Singleton) — central place holding all active limiters

```java
public class RateLimiterRegistry {
    private static volatile RateLimiterRegistry instance;
    private final ConcurrentHashMap<String, RateLimiter> limiters = new ConcurrentHashMap<>();
    private final RuleResolver ruleResolver;

    private RateLimiterRegistry(RuleResolver resolver) {
        this.ruleResolver = resolver;
    }

    public static RateLimiterRegistry getInstance(RuleResolver resolver) {
        if (instance == null) {
            synchronized (RateLimiterRegistry.class) {
                if (instance == null) {
                    instance = new RateLimiterRegistry(resolver);
                }
            }
        }
        return instance;
    }

    public boolean isRequestAllowed(String endpoint, UserTier tier, String clientKey) {
        String ruleKey = endpoint + ":" + tier;
        RateLimiter limiter = limiters.computeIfAbsent(ruleKey,
                k -> new RateLimiter(ruleResolver.resolve(endpoint, tier)));
        return limiter.isAllowed(clientKey);
    }
}
```

### 4.10 Decorator — plugging into request handling without touching business logic

```java
public interface RequestHandler {
    Response handle(Request request);
}

public class RateLimitedInterceptor implements RequestHandler {
    private final RequestHandler delegate;
    private final RateLimiterRegistry registry;

    public RateLimitedInterceptor(RequestHandler delegate, RateLimiterRegistry registry) {
        this.delegate = delegate;
        this.registry = registry;
    }

    @Override
    public Response handle(Request request) {
        boolean allowed = registry.isRequestAllowed(
                request.getEndpoint(), request.getUserTier(), request.getClientKey());

        if (!allowed) {
            return Response.tooManyRequests(); // HTTP 429
        }
        return delegate.handle(request); // untouched business logic
    }
}
```

---

## 5. Extending to distributed / multi-server setup

Introduce a `RateLimiterStore` interface so state isn't tied to local JVM memory:

```java
public interface RateLimiterStore {
    long incrementAndGet(String key, long windowMs);
    long get(String key);
}

public class RedisRateLimiterStore implements RateLimiterStore {
    // Lua script executed atomically in Redis: INCR + EXPIRE
    // guarantees correctness across multiple app instances
}
```

Algorithm classes would take a `RateLimiterStore` instead of managing `ConcurrentHashMap` directly — this is a one-line constructor change per algorithm because they already depend on an abstraction (Dependency Inversion again), not on `ConcurrentHashMap` concretely being "the" storage.

---

## 6. Algorithm trade-off summary (worth stating in an interview)

| Algorithm | Memory | Accuracy | Burst handling | Notes |
|---|---|---|---|---|
| Token Bucket | O(1)/client | Good | Allows controlled bursts | Industry default (AWS, Stripe) |
| Leaky Bucket | O(1)/client | Good | Smooths bursts into constant rate | Good for queue-like outbound traffic |
| Fixed Window | O(1)/client | Poor at boundaries | Can allow 2x burst at edge | Simplest, cheapest |
| Sliding Window Log | O(N) requests/client | Exact | No boundary issue | Memory-heavy at high volume |
| Sliding Window Counter | O(1)/client | Approximate, good enough | Smooths boundary issue | Good compromise — weighted average of two fixed windows |

---

Want me to extend this with the **Sliding Window Counter** implementation, a **distributed Redis+Lua** version, or move on to a different LLD problem (e.g., Parking Lot, Elevator, BookMyShow)?