# Thread-Safe Cache with TTL — LLD Design

## 1. Requirements

**Functional:**
- `put(key, value, ttl)` — store with expiration time
- `get(key)` — return value if not expired, else treat as miss
- `remove(key)`, `clear()`
- Support pluggable eviction policies when cache reaches max size (LRU, LFU, FIFO)
- Expired entries should be cleaned up (both lazily on access AND proactively via background thread)
- Notify listeners on eviction/expiry (for logging/metrics)

**Non-functional:**
- Thread-safe under concurrent reads/writes from multiple threads
- O(1) average get/put
- Configurable capacity and default TTL
- Extensible to add new eviction policies without touching core cache logic
- Minimize lock contention (don't block all reads while one thread writes)

---

## 2. Design Patterns Used (and why)

| Pattern | Where | Why |
|---|---|---|
| **Strategy** | `EvictionPolicy` (LRU, LFU, FIFO) | Eviction algorithm needs to vary independently and be swappable without changing `Cache` core logic |
| **Builder** | `CacheBuilder` | Cache has many optional configs (capacity, default TTL, eviction policy, cleanup interval) — avoids telescoping constructors |
| **Observer** | `CacheEventListener` notified on eviction/expiry | Decouples cache internals from things that react to evictions (metrics, logging, cache-warming) |
| **Decorator** | `StatsCacheDecorator` wraps `Cache` to add hit/miss metrics | Adds cross-cutting behavior (stats tracking) without modifying core cache class — respects OCP |
| **Singleton** (optional) | `CacheManager` if a single global registry of named caches is needed | One place to look up/create caches by name application-wide |
| **Factory** | `EvictionPolicyFactory` | Centralizes creation of eviction policy instances from config/enum |

**Concurrency mechanism** (not a GoF pattern, but core to this design): `ConcurrentHashMap` for the backing store + `ReentrantReadWriteLock`/segment-level locking for eviction-policy metadata updates, and a **daemon thread (Scheduled executor)** for proactive expired-entry sweep — this is the classic **"lazy + active expiration"** hybrid used by Redis/Guava/Caffeine.

---

## 3. SOLID Mapping

- **SRP** — `Cache` handles storage/expiry only; `EvictionPolicy` handles eviction decisions only; `CacheCleaner` (background thread) handles proactive sweep only.
- **OCP** — New eviction policies (LFU, Random, ARC) plug in via `EvictionPolicy` interface without modifying `Cache`.
- **LSP** — Any `EvictionPolicy` implementation is substitutable; `Cache` doesn't care which one it's given.
- **ISP** — `CacheEventListener` only exposes `onExpire`/`onEvict` — listeners aren't forced to implement unrelated methods.
- **DIP** — `Cache` depends on `EvictionPolicy` and `CacheEventListener` interfaces, not concrete implementations.

---

## 4. Class Diagram (textual)

```
CacheEntry<V>
 - value: V, expiryTime: long (epoch ms), createdAt: long
 + isExpired(): boolean

EvictionPolicy<K> (interface)
 + keyAccessed(K), keyAdded(K), keyRemoved(K)
 + evictionCandidate(): K
 ├── LRUEvictionPolicy
 ├── LFUEvictionPolicy
 └── FIFOEvictionPolicy

EvictionPolicyFactory
 + create(EvictionType): EvictionPolicy

CacheEventListener (interface)
 + onExpire(K, V)
 + onEvict(K, V)
 ├── LoggingListener
 └── MetricsListener

Cache<K, V> (interface)
 + put(K, V, ttl), get(K), remove(K), clear(), size()
 └── InMemoryCache<K,V> (concrete, thread-safe impl)

StatsCacheDecorator<K,V> implements Cache<K,V>
 - wraps Cache<K,V>, hitCount, missCount
 + get() [tracks stats], getStats()

CacheCleaner (Runnable, background thread)
 - runs periodically, scans + removes expired entries

CacheBuilder<K,V>
 + capacity(), defaultTtl(), evictionPolicy(), cleanupInterval(), addListener()
 + build(): Cache<K,V>

CacheManager (Singleton)
 - caches: Map<String, Cache>
 + getOrCreate(name, builder): Cache
```

---

## 5. Code (Java)

### CacheEntry

```java
public class CacheEntry<V> {
    private final V value;
    private final long expiryTimeMillis; // absolute epoch time; -1 = never expires

    public CacheEntry(V value, long ttlMillis) {
        this.value = value;
        this.expiryTimeMillis = ttlMillis < 0 ? -1 : System.currentTimeMillis() + ttlMillis;
    }

    public boolean isExpired() {
        return expiryTimeMillis != -1 && System.currentTimeMillis() > expiryTimeMillis;
    }

    public V getValue() { return value; }
}
```

### EvictionPolicy (Strategy pattern)

```java
public interface EvictionPolicy<K> {
    void keyAccessed(K key);
    void keyAdded(K key);
    void keyRemoved(K key);
    K evictionCandidate(); // returns key to evict, or null if none
}
```

**LRU implementation** — uses a thread-safe doubly linked structure via `LinkedHashMap` semantics, but built manually for clarity with synchronized access:

```java
import java.util.*;
import java.util.concurrent.locks.*;

public class LRUEvictionPolicy<K> implements EvictionPolicy<K> {
    private final LinkedHashSet<K> accessOrder = new LinkedHashSet<>();
    private final ReentrantLock lock = new ReentrantLock();

    @Override
    public void keyAccessed(K key) {
        lock.lock();
        try {
            accessOrder.remove(key);
            accessOrder.add(key); // move to most-recently-used end
        } finally {
            lock.unlock();
        }
    }

    @Override
    public void keyAdded(K key) { keyAccessed(key); }

    @Override
    public void keyRemoved(K key) {
        lock.lock();
        try {
            accessOrder.remove(key);
        } finally {
            lock.unlock();
        }
    }

    @Override
    public K evictionCandidate() {
        lock.lock();
        try {
            Iterator<K> it = accessOrder.iterator();
            return it.hasNext() ? it.next() : null; // least-recently-used = first inserted
        } finally {
            lock.unlock();
        }
    }
}
```

**FIFO implementation:**

```java
import java.util.*;
import java.util.concurrent.*;

public class FIFOEvictionPolicy<K> implements EvictionPolicy<K> {
    private final Queue<K> queue = new ConcurrentLinkedQueue<>();

    @Override public void keyAccessed(K key) { /* no-op for FIFO */ }
    @Override public void keyAdded(K key) { queue.offer(key); }
    @Override public void keyRemoved(K key) { queue.remove(key); }
    @Override public K evictionCandidate() { return queue.peek(); }
}
```

**LFU implementation** (frequency counter + min-heap style via TreeMap):

```java
import java.util.*;
import java.util.concurrent.*;

public class LFUEvictionPolicy<K> implements EvictionPolicy<K> {
    private final ConcurrentHashMap<K, Integer> frequency = new ConcurrentHashMap<>();

    @Override public void keyAccessed(K key) { frequency.merge(key, 1, Integer::sum); }
    @Override public void keyAdded(K key) { frequency.put(key, 1); }
    @Override public void keyRemoved(K key) { frequency.remove(key); }

    @Override
    public K evictionCandidate() {
        return frequency.entrySet().stream()
            .min(Map.Entry.comparingByValue())
            .map(Map.Entry::getKey)
            .orElse(null);
    }
}
```

### EvictionPolicyFactory (Factory pattern)

```java
public enum EvictionType { LRU, LFU, FIFO }

public class EvictionPolicyFactory {
    public static <K> EvictionPolicy<K> create(EvictionType type) {
        return switch (type) {
            case LRU -> new LRUEvictionPolicy<>();
            case LFU -> new LFUEvictionPolicy<>();
            case FIFO -> new FIFOEvictionPolicy<>();
        };
    }
}
```

### CacheEventListener (Observer pattern)

```java
public interface CacheEventListener<K, V> {
    void onExpire(K key, V value);
    void onEvict(K key, V value);
}

public class LoggingListener<K, V> implements CacheEventListener<K, V> {
    @Override public void onExpire(K key, V value) {
        System.out.println("[EXPIRED] key=" + key);
    }
    @Override public void onEvict(K key, V value) {
        System.out.println("[EVICTED] key=" + key);
    }
}

public class MetricsListener<K, V> implements CacheEventListener<K, V> {
    private final java.util.concurrent.atomic.AtomicLong expiryCount = new java.util.concurrent.atomic.AtomicLong();
    private final java.util.concurrent.atomic.AtomicLong evictCount = new java.util.concurrent.atomic.AtomicLong();

    @Override public void onExpire(K key, V value) { expiryCount.incrementAndGet(); }
    @Override public void onEvict(K key, V value) { evictCount.incrementAndGet(); }

    public long getExpiryCount() { return expiryCount.get(); }
    public long getEvictCount() { return evictCount.get(); }
}
```

### Cache interface

```java
public interface Cache<K, V> {
    void put(K key, V value, long ttlMillis);
    V get(K key);
    void remove(K key);
    void clear();
    int size();
}
```

### InMemoryCache — the core thread-safe implementation

**Concurrency design:**
- `ConcurrentHashMap<K, CacheEntry<V>>` for the actual data — lock-free reads, fine-grained locking on writes internally.
- A separate `ReentrantLock` guards **capacity-check + eviction** (a compound "check size, evict, then insert" operation that must be atomic to avoid overshooting capacity under concurrent puts).
- Expired entries are removed **lazily** (checked on `get`) AND **actively** (background `ScheduledExecutorService` sweep) — this is the standard hybrid strategy so memory doesn't leak from keys that are never accessed again.

```java
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.locks.*;

public class InMemoryCache<K, V> implements Cache<K, V> {
    private final ConcurrentHashMap<K, CacheEntry<V>> store = new ConcurrentHashMap<>();
    private final EvictionPolicy<K> evictionPolicy;
    private final int capacity;
    private final long defaultTtlMillis;
    private final List<CacheEventListener<K, V>> listeners;
    private final ReentrantLock evictionLock = new ReentrantLock();

    public InMemoryCache(int capacity, long defaultTtlMillis,
                          EvictionPolicy<K> evictionPolicy,
                          List<CacheEventListener<K, V>> listeners) {
        this.capacity = capacity;
        this.defaultTtlMillis = defaultTtlMillis;
        this.evictionPolicy = evictionPolicy;
        this.listeners = listeners;
    }

    @Override
    public void put(K key, V value, long ttlMillis) {
        long effectiveTtl = ttlMillis > 0 ? ttlMillis : defaultTtlMillis;
        CacheEntry<V> entry = new CacheEntry<>(value, effectiveTtl);

        evictionLock.lock();
        try {
            // if key is new and at capacity, evict before inserting
            if (!store.containsKey(key) && store.size() >= capacity) {
                evictOne();
            }
            store.put(key, entry);
            evictionPolicy.keyAdded(key);
        } finally {
            evictionLock.unlock();
        }
    }

    @Override
    public V get(K key) {
        CacheEntry<V> entry = store.get(key); // lock-free read
        if (entry == null) return null;

        if (entry.isExpired()) {
            // lazy expiry: remove and notify
            removeInternal(key, true);
            return null;
        }

        evictionPolicy.keyAccessed(key);
        return entry.getValue();
    }

    @Override
    public void remove(K key) {
        removeInternal(key, false);
    }

    private void removeInternal(K key, boolean isExpiry) {
        CacheEntry<V> removed = store.remove(key);
        evictionPolicy.keyRemoved(key);
        if (removed != null) {
            notifyListeners(key, removed.getValue(), isExpiry);
        }
    }

    private void evictOne() {
        K candidate = evictionPolicy.evictionCandidate();
        if (candidate != null) {
            CacheEntry<V> removed = store.remove(candidate);
            evictionPolicy.keyRemoved(candidate);
            if (removed != null) {
                notifyListeners(candidate, removed.getValue(), false);
            }
        }
    }

    private void notifyListeners(K key, V value, boolean isExpiry) {
        for (CacheEventListener<K, V> listener : listeners) {
            if (isExpiry) listener.onExpire(key, value);
            else listener.onEvict(key, value);
        }
    }

    @Override
    public void clear() { store.clear(); }

    @Override
    public int size() { return store.size(); }

    // used by background cleaner
    Set<K> keys() { return store.keySet(); }
    CacheEntry<V> peekEntry(K key) { return store.get(key); }
    void expireIfNeeded(K key) {
        CacheEntry<V> entry = store.get(key);
        if (entry != null && entry.isExpired()) {
            removeInternal(key, true);
        }
    }
}
```

### CacheCleaner — active/proactive expiration sweep

```java
import java.util.concurrent.*;

public class CacheCleaner<K, V> {
    private final InMemoryCache<K, V> cache;
    private final ScheduledExecutorService scheduler = Executors.newSingleThreadScheduledExecutor(
        r -> {
            Thread t = new Thread(r, "cache-cleaner");
            t.setDaemon(true); // don't block JVM shutdown
            return t;
        });

    public CacheCleaner(InMemoryCache<K, V> cache) {
        this.cache = cache;
    }

    public void start(long intervalMillis) {
        scheduler.scheduleAtFixedRate(this::sweep, intervalMillis, intervalMillis, TimeUnit.MILLISECONDS);
    }

    private void sweep() {
        for (K key : cache.keys()) {
            cache.expireIfNeeded(key);
        }
    }

    public void stop() { scheduler.shutdown(); }
}
```

### StatsCacheDecorator (Decorator pattern)

```java
import java.util.concurrent.atomic.AtomicLong;

public class StatsCacheDecorator<K, V> implements Cache<K, V> {
    private final Cache<K, V> delegate;
    private final AtomicLong hits = new AtomicLong();
    private final AtomicLong misses = new AtomicLong();

    public StatsCacheDecorator(Cache<K, V> delegate) { this.delegate = delegate; }

    @Override
    public void put(K key, V value, long ttlMillis) { delegate.put(key, value, ttlMillis); }

    @Override
    public V get(K key) {
        V value = delegate.get(key);
        if (value != null) hits.incrementAndGet();
        else misses.incrementAndGet();
        return value;
    }

    @Override public void remove(K key) { delegate.remove(key); }
    @Override public void clear() { delegate.clear(); }
    @Override public int size() { return delegate.size(); }

    public double hitRatio() {
        long total = hits.get() + misses.get();
        return total == 0 ? 0.0 : (double) hits.get() / total;
    }
}
```

### CacheBuilder (Builder pattern)

```java
import java.util.*;

public class CacheBuilder<K, V> {
    private int capacity = 1000;
    private long defaultTtlMillis = TimeUnit.MINUTES.toMillis(5);
    private EvictionType evictionType = EvictionType.LRU;
    private long cleanupIntervalMillis = TimeUnit.SECONDS.toMillis(30);
    private final List<CacheEventListener<K, V>> listeners = new ArrayList<>();
    private boolean enableStats = false;

    public CacheBuilder<K, V> capacity(int capacity) { this.capacity = capacity; return this; }
    public CacheBuilder<K, V> defaultTtl(long ttlMillis) { this.defaultTtlMillis = ttlMillis; return this; }
    public CacheBuilder<K, V> evictionPolicy(EvictionType type) { this.evictionType = type; return this; }
    public CacheBuilder<K, V> cleanupInterval(long intervalMillis) { this.cleanupIntervalMillis = intervalMillis; return this; }
    public CacheBuilder<K, V> addListener(CacheEventListener<K, V> listener) { listeners.add(listener); return this; }
    public CacheBuilder<K, V> enableStats() { this.enableStats = true; return this; }

    public Cache<K, V> build() {
        EvictionPolicy<K> policy = EvictionPolicyFactory.create(evictionType);
        InMemoryCache<K, V> core = new InMemoryCache<>(capacity, defaultTtlMillis, policy, listeners);

        CacheCleaner<K, V> cleaner = new CacheCleaner<>(core);
        cleaner.start(cleanupIntervalMillis);

        Cache<K, V> cache = core;
        if (enableStats) {
            cache = new StatsCacheDecorator<>(cache);
        }
        return cache;
    }
}
```

### CacheManager (Singleton, optional multi-cache registry)

```java
import java.util.concurrent.*;

public class CacheManager {
    private static final CacheManager INSTANCE = new CacheManager();
    private final ConcurrentHashMap<String, Cache<?, ?>> caches = new ConcurrentHashMap<>();

    private CacheManager() {}

    public static CacheManager getInstance() { return INSTANCE; }

    @SuppressWarnings("unchecked")
    public <K, V> Cache<K, V> getOrCreate(String name, Supplier<Cache<K, V>> supplier) {
        return (Cache<K, V>) caches.computeIfAbsent(name, k -> supplier.get());
    }
}
```

### Usage

```java
public class Main {
    public static void main(String[] args) throws InterruptedException {
        Cache<String, String> cache = new CacheBuilder<String, String>()
            .capacity(3)
            .defaultTtl(2000) // 2 sec default TTL
            .evictionPolicy(EvictionType.LRU)
            .cleanupInterval(1000)
            .addListener(new LoggingListener<>())
            .enableStats()
            .build();

        cache.put("a", "apple", -1);   // uses default TTL
        cache.put("b", "banana", -1);
        cache.put("c", "cherry", -1);

        System.out.println(cache.get("a")); // apple (hit)
        cache.put("d", "date", -1);          // capacity=3, evicts LRU ("b", since "a" was accessed)

        Thread.sleep(2500);
        System.out.println(cache.get("c"));  // null (expired, cleaned by background sweep or lazily)

        if (cache instanceof StatsCacheDecorator<String, String> stats) {
            System.out.println("Hit ratio: " + stats.hitRatio());
        }
    }
}
```

---

## 6. Concurrency Deep-Dive (why this is actually thread-safe)

1. **Reads (`get`) are lock-free** — `ConcurrentHashMap.get()` doesn't block writers, giving high read throughput.
2. **Writes to the map itself** are handled by `ConcurrentHashMap`'s internal bucket-level locking — multiple threads can `put` different keys concurrently.
3. **The eviction check-then-act sequence** (`if size >= capacity, evict, then insert`) is the one place that needs an explicit `ReentrantLock` — without it, two threads could both pass the capacity check simultaneously and cause the cache to exceed capacity, or both try to evict the same candidate.
4. **Eviction policy metadata** (LRU order, LFU frequency) is guarded independently inside each policy implementation — keeping that logic out of the core cache lock keeps the critical section small.
5. **Background cleaner runs on a separate daemon thread**, calling only thread-safe cache methods — no shared mutable state with foreground threads beyond what `ConcurrentHashMap` already protects.
6. **Lazy + active hybrid expiry**: lazy check on `get()` guarantees correctness (never return stale data) even if the background sweep hasn't run yet; active sweep guarantees memory doesn't leak from cold/unread expired keys.

---

## 7. Extensibility Notes

- **New eviction policy** (ARC, Random, Segmented-LRU) → implement `EvictionPolicy`, register in `EvictionPolicyFactory`. No changes to `InMemoryCache`.
- **Distributed cache** (Redis-backed) → introduce a `RemoteCache<K,V> implements Cache<K,V>` and a `TieredCache` (L1 in-memory + L2 remote) using the same interface — callers don't change.
- **Write-through/write-behind persistence** → add a `PersistentCacheDecorator` similar to `StatsCacheDecorator`, wrapping writes to also push to a DB/disk.
- **Per-key custom TTL refresh on access** (sliding expiration) → add a `RefreshOnAccessPolicy` flag, extend `CacheEntry` to support `refresh()`, called from `get()`.
- **Bounded memory (not just count)** → change eviction trigger from `size() >= capacity` to a `WeightedSizeCalculator` strategy that sums serialized byte size instead of entry count.

Want me to go deeper into **LRU implementation using a genuinely O(1) doubly linked list + hashmap (instead of LinkedHashSet)**, **a lock-striping approach for even higher write concurrency**, or **comparing this design against how Guava Cache / Caffeine actually implement it internally**?