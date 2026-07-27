# Multithreaded Server — LLD

## 1. Requirements

**Functional**
- Server accepts incoming client connections and handles requests concurrently.
- A bounded **thread pool** processes requests — don't spawn unbounded threads (thread-per-request doesn't scale).
- Task queue buffers incoming work when all worker threads are busy.
- **Backpressure/rejection policy** when queue is full — configurable (reject, block caller, discard, run-on-caller-thread).
- Graceful shutdown: stop accepting new work, let in-flight tasks finish, then terminate.
- Dynamic pool sizing (core threads always alive, extra threads spun up under load, idle threads reaped after timeout).
- Support both **I/O-bound** (many connections, mostly waiting) and **CPU-bound** (few connections, heavy compute) workloads — different threading models suit each.
- Monitor pool health: active threads, queue depth, completed/rejected task counts.

**Non-functional**
- No single global lock bottlenecking every task submission.
- New rejection policies pluggable without touching the pool's core logic.
- New I/O handling models (blocking thread-per-connection vs non-blocking reactor) pluggable independently.
- Thread lifecycle (create/reap) managed safely — no race conditions on pool size.

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **Thread Pool / Worker pattern** | `CustomThreadPoolExecutor` with fixed `WorkerThread`s pulling from a shared queue | Core of the whole design — reuses a bounded set of threads instead of creating one per task, avoiding the overhead/instability of unbounded thread creation. |
| **Producer-Consumer** | `BlockingTaskQueue` — clients (producers) submit tasks, worker threads (consumers) pull and execute | Decouples task submission rate from task execution rate; the queue is the buffer that absorbs bursts without blocking submitters synchronously (up to capacity). |
| **Command** | `Task` (essentially `Runnable`) wraps a unit of work | The pool needs to store, queue, and execute "work" without knowing what that work actually does — Command gives a uniform `run()` contract regardless of task type. |
| **Strategy** | `RejectionPolicy`: `AbortPolicy`, `CallerRunsPolicy`, `DiscardPolicy`, `DiscardOldestPolicy` | What happens when the queue is full is a genuinely independent policy decision — isolating it means the core submit/execute path never has an if/else per policy. |
| **Reactor** | `Reactor`/`Acceptor`/`EventDispatcher` for the I/O layer (non-blocking, single-threaded event loop demultiplexing I/O events to handlers) | For I/O-bound workloads (many idle connections), a small number of reactor threads handling readiness events vastly outperforms one thread blocked per connection — this is how Netty/Node.js/nginx scale to tens of thousands of connections. |
| **Strategy (again)** | `ConcurrencyModel`: `ThreadPerConnectionModel`, `ReactorModel` | The connection-handling model itself (blocking thread-per-connection vs non-blocking reactor) is swappable depending on workload shape (CPU-bound vs I/O-bound) — isolating it lets `Server` stay agnostic to which model is active. |
| **Singleton** | `ServerMetricsRegistry` | Single, central place aggregating pool/connection metrics for monitoring — one source of truth process-wide. |
| **Observer** | `ThreadPoolExecutor` (Subject) notifies `PoolLifecycleObserver`: `MetricsCollector`, `AlertingObserver` | Pool events (task rejected, thread created/reaped, queue high-watermark) → independent reactions without the pool itself knowing about metrics/alerting. |
| **Builder** | `ServerConfig.Builder`, `ThreadPoolConfig.Builder` | Many optional tunables (core size, max size, queue capacity, keep-alive, rejection policy). |
| **Template Method** | `WorkerThread.run()` defines the fixed loop: take task → execute → handle exception → repeat/exit | Every worker thread needs the identical lifecycle skeleton (pull-execute-catch-loop); this shouldn't be reimplemented per worker type. |

**SOLID**
- **S**: `BlockingTaskQueue` only queues; `WorkerThread` only executes; `RejectionPolicy` only decides overflow behavior; `Reactor` only demultiplexes I/O events.
- **O**: New rejection policy → new `RejectionPolicy` implementation. New concurrency model → new `ConcurrencyModel` implementation. Nothing existing changes.
- **L**: Any `RejectionPolicy`/`ConcurrencyModel` substitutable wherever used.
- **I**: `PoolLifecycleObserver` exposes only `onEvent`; `Task` exposes only `run()` — no bloated interfaces.
- **D**: `CustomThreadPoolExecutor` depends on `RejectionPolicy`, `BlockingTaskQueue` abstractions injected at construction, never concrete implementations.

---

## 3. Class Diagram (textual)

```
┌────────────────┐        ┌──────────────────────────────┐
│  Task               │◀───────│  CustomThreadPoolExecutor         │
│ (Command interface)   │       │ - workers: List<WorkerThread>       │
│ + run()                 │     │ - taskQueue: BlockingTaskQueue        │
└────────────────┘        │ - rejectionPolicy: RejectionPolicy      │
                            │ - corePoolSize, maxPoolSize, keepAlive    │
                            │ + submit(Task)                              │
                            │ + shutdown() / shutdownNow()                  │
                            └──────────────────────────────┘

┌────────────────┐        ┌──────────────────────────────┐
│  BlockingTaskQueue    │    │  WorkerThread (extends Thread)    │
│  (Producer-Consumer)     │  │ + run() [Template Method]           │
│  + offer(Task): bool       │ │   - take from queue                    │
│  + take(): Task               │ │   - execute task                       │
│  + poll(timeout): Task          │ │   - handle exceptions, loop/exit         │
└────────────────┘        └──────────────────────────────┘

┌────────────────────┐    ┌──────────────────────────────┐
│  RejectionPolicy         │  │  PoolLifecycleObserver            │
│ (Strategy interface)       │ │ + onEvent(evt, data)                │
│ + reject(task, executor)     │└──────────▲───────────┘
└──────────▲───────────┘      ┌────────────┼────────────┐
   ┌───────┼────┬───────┐ MetricsCollector      AlertingObserver
AbortPolicy CallerRuns DiscardPolicy DiscardOldest
              Policy                 Policy

┌────────────────────┐    ┌──────────────────────────────┐
│  ConcurrencyModel        │  │  Server                            │
│ (Strategy interface)       │  │ - concurrencyModel: ConcurrencyModel│
│ + start(port)                │ │ + start()                            │
│ + handleConnection(conn)       │└──────────────────────────────┘
└──────────▲───────────┘
   ┌───────┼──────────────┐
ThreadPerConnectionModel  ReactorModel
                            │
                     ┌──────┴──────┐
                  Acceptor      EventDispatcher (Reactor pattern)

┌────────────────────┐    ┌──────────────────────────────┐
│  ServerMetricsRegistry    │  │  ServerConfig (Builder)             │
│  (Singleton)                 │  └──────────────────────────────┘
└────────────────────┘
```

---

## 4. Code (Java)

### 4.1 Task — Command

```java
@FunctionalInterface
public interface Task {
    void run() throws Exception;
}
```

### 4.2 Producer-Consumer — BlockingTaskQueue

```java
public class BlockingTaskQueue {
    private final Queue<Task> queue = new ArrayDeque<>();
    private final int capacity;
    private final Object lock = new Object();

    public BlockingTaskQueue(int capacity) { this.capacity = capacity; }

    /** Non-blocking offer — returns false if full (caller/policy decides what to do). */
    public boolean offer(Task task) {
        synchronized (lock) {
            if (queue.size() >= capacity) return false;
            queue.offer(task);
            lock.notify(); // wake one waiting worker
            return true;
        }
    }

    /** Blocks until a task is available. */
    public Task take() throws InterruptedException {
        synchronized (lock) {
            while (queue.isEmpty()) {
                lock.wait();
            }
            return queue.poll();
        }
    }

    /** Blocks up to timeout waiting for a task; returns null if none arrived (used for idle-thread reaping). */
    public Task poll(long timeoutMs) throws InterruptedException {
        synchronized (lock) {
            long deadline = System.currentTimeMillis() + timeoutMs;
            while (queue.isEmpty()) {
                long remaining = deadline - System.currentTimeMillis();
                if (remaining <= 0) return null;
                lock.wait(remaining);
            }
            return queue.poll();
        }
    }

    public Task pollOldestForDiscard() {
        synchronized (lock) { return queue.poll(); }
    }

    public int size() { synchronized (lock) { return queue.size(); } }
}
```

### 4.3 Strategy — Rejection Policy

```java
public interface RejectionPolicy {
    void reject(Task task, CustomThreadPoolExecutor executor);
}

public class AbortPolicy implements RejectionPolicy {
    public void reject(Task task, CustomThreadPoolExecutor executor) {
        throw new RejectedExecutionException("Task rejected — queue full, pool exhausted");
    }
}

public class CallerRunsPolicy implements RejectionPolicy {
    public void reject(Task task, CustomThreadPoolExecutor executor) {
        // executes on the submitting thread itself — natural backpressure, slows the producer down
        try {
            task.run();
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}

public class DiscardPolicy implements RejectionPolicy {
    public void reject(Task task, CustomThreadPoolExecutor executor) {
        // silently drop — acceptable for best-effort/non-critical workloads (e.g. metrics pings)
    }
}

public class DiscardOldestPolicy implements RejectionPolicy {
    public void reject(Task task, CustomThreadPoolExecutor executor) {
        executor.getTaskQueue().pollOldestForDiscard(); // evict oldest queued task
        executor.getTaskQueue().offer(task);              // make room for the new one
    }
}

class RejectedExecutionException extends RuntimeException {
    public RejectedExecutionException(String msg) { super(msg); }
}
```

### 4.4 Observer — pool lifecycle events

```java
public interface PoolLifecycleObserver {
    void onEvent(String eventType, Map<String, Object> data);
}

public class MetricsCollector implements PoolLifecycleObserver {
    public void onEvent(String eventType, Map<String, Object> data) {
        ServerMetricsRegistry.getInstance().record(eventType, data);
    }
}

public class AlertingObserver implements PoolLifecycleObserver {
    public void onEvent(String eventType, Map<String, Object> data) {
        if (eventType.equals("TASK_REJECTED")) {
            System.out.println("[ALERT] Task rejected — pool may be saturated: " + data);
        }
        if (eventType.equals("QUEUE_HIGH_WATERMARK")) {
            System.out.println("[ALERT] Task queue nearing capacity: " + data);
        }
    }
}
```

### 4.5 Template Method — WorkerThread

```java
public class WorkerThread extends Thread {
    private final CustomThreadPoolExecutor pool;
    private volatile boolean isCore; // core threads block forever; extra threads time out when idle
    private volatile boolean running = true;

    public WorkerThread(CustomThreadPoolExecutor pool, boolean isCore, String name) {
        super(name);
        this.pool = pool;
        this.isCore = isCore;
    }

    @Override
    public void run() {
        while (running && !pool.isShutdown()) {
            try {
                Task task = isCore
                        ? pool.getTaskQueue().take()                       // core threads: block indefinitely
                        : pool.getTaskQueue().poll(pool.getKeepAliveMs());  // extra threads: idle-timeout

                if (task == null) {
                    // idle timeout expired on a non-core thread — self-terminate
                    pool.retireWorker(this);
                    return;
                }

                pool.notifyObservers("TASK_STARTED", Map.of("thread", getName()));
                executeTask(task);

            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                return;
            }
        }
    }

    private void executeTask(Task task) {
        try {
            task.run();
            pool.notifyObservers("TASK_COMPLETED", Map.of("thread", getName()));
        } catch (Exception e) {
            pool.notifyObservers("TASK_FAILED", Map.of("thread", getName(), "error", e.getMessage()));
            // worker survives a failed task and continues the loop — one bad task shouldn't kill the thread
        }
    }

    void stopWorker() { running = false; interrupt(); }
}
```

### 4.6 CustomThreadPoolExecutor — the pool itself

```java
public class CustomThreadPoolExecutor {
    private final int corePoolSize;
    private final int maxPoolSize;
    private final long keepAliveMs;
    private final BlockingTaskQueue taskQueue;
    private final RejectionPolicy rejectionPolicy;
    private final List<WorkerThread> workers = new CopyOnWriteArrayList<>();
    private final List<PoolLifecycleObserver> observers = new ArrayList<>();
    private final AtomicInteger activeThreadCount = new AtomicInteger(0);
    private volatile boolean shutdown = false;

    private CustomThreadPoolExecutor(Builder b) {
        this.corePoolSize = b.corePoolSize;
        this.maxPoolSize = b.maxPoolSize;
        this.keepAliveMs = b.keepAliveMs;
        this.taskQueue = new BlockingTaskQueue(b.queueCapacity);
        this.rejectionPolicy = b.rejectionPolicy;

        for (int i = 0; i < corePoolSize; i++) {
            spawnWorker(true);
        }
    }

    public void subscribe(PoolLifecycleObserver o) { observers.add(o); }
    void notifyObservers(String eventType, Map<String, Object> data) {
        for (PoolLifecycleObserver o : observers) o.onEvent(eventType, data);
    }

    public void submit(Task task) {
        if (shutdown) throw new IllegalStateException("Pool is shut down, cannot accept new tasks");

        boolean queued = taskQueue.offer(task);
        if (queued) {
            if (taskQueue.size() > taskQueue.size() * 0.8) { // simplified high-watermark check
                notifyObservers("QUEUE_HIGH_WATERMARK", Map.of("depth", taskQueue.size()));
            }
            maybeGrowPool();
            return;
        }

        // queue full — try growing pool before invoking rejection policy
        if (workers.size() < maxPoolSize) {
            spawnWorker(false);
            if (taskQueue.offer(task)) return;
        }

        notifyObservers("TASK_REJECTED", Map.of("queueDepth", taskQueue.size(), "poolSize", workers.size()));
        rejectionPolicy.reject(task, this);
    }

    private void maybeGrowPool() {
        if (taskQueue.size() > 0 && workers.size() < maxPoolSize) {
            spawnWorker(false); // spin up an extra (non-core) thread under load
        }
    }

    private void spawnWorker(boolean isCore) {
        WorkerThread worker = new WorkerThread(this, isCore, "worker-" + workers.size());
        workers.add(worker);
        activeThreadCount.incrementAndGet();
        worker.start();
        notifyObservers("THREAD_CREATED", Map.of("isCore", isCore, "poolSize", workers.size()));
    }

    void retireWorker(WorkerThread worker) {
        workers.remove(worker);
        activeThreadCount.decrementAndGet();
        notifyObservers("THREAD_REAPED", Map.of("poolSize", workers.size()));
    }

    /** Graceful shutdown: stop accepting new work, let queued/in-flight tasks finish. */
    public void shutdown() {
        shutdown = true;
        notifyObservers("SHUTDOWN_INITIATED", Map.of("pendingTasks", taskQueue.size()));
        // workers naturally drain the queue and exit once shutdown=true and queue empties
    }

    /** Immediate shutdown: interrupt all workers, abandon queued tasks. */
    public void shutdownNow() {
        shutdown = true;
        for (WorkerThread w : workers) w.stopWorker();
        notifyObservers("SHUTDOWN_FORCED", Map.of("abandonedTasks", taskQueue.size()));
    }

    public boolean isShutdown() { return shutdown; }
    BlockingTaskQueue getTaskQueue() { return taskQueue; }
    long getKeepAliveMs() { return keepAliveMs; }
    public int getActiveThreadCount() { return activeThreadCount.get(); }
    public int getQueueDepth() { return taskQueue.size(); }

    public static class Builder {
        private int corePoolSize = 4;
        private int maxPoolSize = 16;
        private long keepAliveMs = 60_000;
        private int queueCapacity = 100;
        private RejectionPolicy rejectionPolicy = new AbortPolicy();

        public Builder corePoolSize(int n) { this.corePoolSize = n; return this; }
        public Builder maxPoolSize(int n) { this.maxPoolSize = n; return this; }
        public Builder keepAliveMs(long ms) { this.keepAliveMs = ms; return this; }
        public Builder queueCapacity(int n) { this.queueCapacity = n; return this; }
        public Builder rejectionPolicy(RejectionPolicy p) { this.rejectionPolicy = p; return this; }
        public CustomThreadPoolExecutor build() { return new CustomThreadPoolExecutor(this); }
    }
}
```

### 4.7 Singleton — ServerMetricsRegistry

```java
public class ServerMetricsRegistry {
    private static volatile ServerMetricsRegistry instance;
    private final ConcurrentHashMap<String, AtomicLong> counters = new ConcurrentHashMap<>();

    private ServerMetricsRegistry() {}

    public static ServerMetricsRegistry getInstance() {
        if (instance == null) {
            synchronized (ServerMetricsRegistry.class) {
                if (instance == null) instance = new ServerMetricsRegistry();
            }
        }
        return instance;
    }

    public void record(String eventType, Map<String, Object> data) {
        counters.computeIfAbsent(eventType, k -> new AtomicLong()).incrementAndGet();
    }

    public long getCount(String eventType) {
        return counters.getOrDefault(eventType, new AtomicLong()).get();
    }
}
```

### 4.8 Strategy — Concurrency Model (connection handling)

```java
public interface ConcurrencyModel {
    void start(int port) throws IOException;
    void handleConnection(SocketChannel connection);
}
```

**Thread-per-connection** (simple; good for low connection counts / CPU-heavy work per connection):

```java
public class ThreadPerConnectionModel implements ConcurrencyModel {
    private final CustomThreadPoolExecutor pool;
    private final RequestHandler requestHandler;

    public ThreadPerConnectionModel(CustomThreadPoolExecutor pool, RequestHandler requestHandler) {
        this.pool = pool; this.requestHandler = requestHandler;
    }

    @Override
    public void start(int port) throws IOException {
        ServerSocket serverSocket = new ServerSocket(port);
        System.out.println("Listening on port " + port + " (thread-per-connection model)");
        while (true) {
            Socket clientSocket = serverSocket.accept(); // blocking accept
            pool.submit(() -> requestHandler.handle(clientSocket)); // hand off to pool, don't block acceptor
        }
    }

    @Override
    public void handleConnection(SocketChannel connection) {
        throw new UnsupportedOperationException("Not used in blocking model");
    }
}
```

**Reactor model** (non-blocking; scales to many concurrent, mostly-idle connections):

```java
public class ReactorModel implements ConcurrencyModel {
    private final CustomThreadPoolExecutor workerPool; // for handing off actual request processing (avoid blocking reactor thread)
    private final RequestHandler requestHandler;
    private Selector selector;

    public ReactorModel(CustomThreadPoolExecutor workerPool, RequestHandler requestHandler) {
        this.workerPool = workerPool; this.requestHandler = requestHandler;
    }

    @Override
    public void start(int port) throws IOException {
        ServerSocketChannel serverChannel = ServerSocketChannel.open();
        serverChannel.bind(new InetSocketAddress(port));
        serverChannel.configureBlocking(false);

        selector = Selector.open();
        serverChannel.register(selector, SelectionKey.OP_ACCEPT);
        System.out.println("Listening on port " + port + " (reactor model)");

        while (true) {
            selector.select(); // blocks until at least one channel is ready
            Iterator<SelectionKey> keys = selector.selectedKeys().iterator();
            while (keys.hasNext()) {
                SelectionKey key = keys.next();
                keys.remove();
                if (!key.isValid()) continue;

                if (key.isAcceptable()) acceptConnection(key);
                else if (key.isReadable()) readFromConnection(key);
            }
        }
    }

    private void acceptConnection(SelectionKey key) throws IOException {
        ServerSocketChannel serverChannel = (ServerSocketChannel) key.channel();
        SocketChannel client = serverChannel.accept();
        client.configureBlocking(false);
        client.register(selector, SelectionKey.OP_READ);
    }

    private void readFromConnection(SelectionKey key) {
        SocketChannel channel = (SocketChannel) key.channel();
        // hand off actual (potentially slow) request processing to the worker pool —
        // the reactor thread itself must never block, or every connection stalls
        workerPool.submit(() -> requestHandler.handleChannel(channel));
    }

    @Override
    public void handleConnection(SocketChannel connection) {
        requestHandler.handleChannel(connection);
    }
}
```

### 4.9 RequestHandler — actual application logic (Command executed by workers)

```java
public interface RequestHandler {
    void handle(Socket socket);
    void handleChannel(SocketChannel channel);
}

public class EchoRequestHandler implements RequestHandler {
    @Override
    public void handle(Socket socket) {
        try (socket; BufferedReader in = new BufferedReader(new InputStreamReader(socket.getInputStream()));
             PrintWriter out = new PrintWriter(socket.getOutputStream(), true)) {
            String line = in.readLine();
            out.println("Echo: " + line);
        } catch (IOException e) {
            System.err.println("Error handling connection: " + e.getMessage());
        }
    }

    @Override
    public void handleChannel(SocketChannel channel) {
        try {
            ByteBuffer buffer = ByteBuffer.allocate(1024);
            int read = channel.read(buffer);
            if (read == -1) { channel.close(); return; }
            buffer.flip();
            channel.write(buffer); // echo back
        } catch (IOException e) {
            try { channel.close(); } catch (IOException ignored) {}
        }
    }
}
```

### 4.10 Server — top-level orchestrator (Builder for config)

```java
public class ServerConfig {
    final int port; final int corePoolSize, maxPoolSize, queueCapacity;
    final long keepAliveMs; final RejectionPolicy rejectionPolicy;

    private ServerConfig(Builder b) {
        this.port = b.port; this.corePoolSize = b.corePoolSize; this.maxPoolSize = b.maxPoolSize;
        this.queueCapacity = b.queueCapacity; this.keepAliveMs = b.keepAliveMs;
        this.rejectionPolicy = b.rejectionPolicy;
    }

    public static class Builder {
        private int port = 8080;
        private int corePoolSize = 4, maxPoolSize = 32, queueCapacity = 200;
        private long keepAliveMs = 60_000;
        private RejectionPolicy rejectionPolicy = new CallerRunsPolicy();

        public Builder port(int p) { this.port = p; return this; }
        public Builder corePoolSize(int n) { this.corePoolSize = n; return this; }
        public Builder maxPoolSize(int n) { this.maxPoolSize = n; return this; }
        public Builder queueCapacity(int n) { this.queueCapacity = n; return this; }
        public Builder keepAliveMs(long ms) { this.keepAliveMs = ms; return this; }
        public Builder rejectionPolicy(RejectionPolicy p) { this.rejectionPolicy = p; return this; }
        public ServerConfig build() { return new ServerConfig(this); }
    }
}

public class Server {
    private final ServerConfig config;
    private final ConcurrencyModel concurrencyModel;

    public Server(ServerConfig config, ConcurrencyModel concurrencyModel) {
        this.config = config; this.concurrencyModel = concurrencyModel;
    }

    public void start() throws IOException {
        concurrencyModel.start(config.port);
    }
}
```

### 4.11 Putting it together

```java
public class MultithreadedServerDemo {
    public static void main(String[] args) throws IOException {
        CustomThreadPoolExecutor pool = new CustomThreadPoolExecutor.Builder()
                .corePoolSize(4)
                .maxPoolSize(32)
                .queueCapacity(200)
                .keepAliveMs(30_000)
                .rejectionPolicy(new CallerRunsPolicy()) // natural backpressure under overload
                .build();

        pool.subscribe(new MetricsCollector());
        pool.subscribe(new AlertingObserver());

        RequestHandler handler = new EchoRequestHandler();

        ServerConfig config = new ServerConfig.Builder().port(8080).build();

        // choose model based on workload shape: many idle connections -> Reactor; few, heavy -> thread-per-connection
        ConcurrencyModel model = new ReactorModel(pool, handler);
        Server server = new Server(config, model);

        Runtime.getRuntime().addShutdownHook(new Thread(pool::shutdown)); // graceful shutdown on Ctrl+C

        server.start();
    }
}
```

---

## 5. Why this shape holds up under follow-ups

- **"Implement a thread pool from scratch"** — this is often asked standalone; `CustomThreadPoolExecutor` + `BlockingTaskQueue` + `WorkerThread` + `RejectionPolicy` is exactly that sub-answer, cleanly separable from the server/networking layer.
- **"Handle 10,000 concurrent connections (C10K problem)"** → this is precisely why `ReactorModel` exists — a handful of reactor/selector threads demultiplex I/O readiness events instead of one blocked thread per connection; the `ThreadPerConnectionModel` doesn't scale here, and swapping models requires zero changes to `Server` or the thread pool.
- **"Add priority to certain requests (e.g., health checks jump the queue)"** → replace `BlockingTaskQueue`'s internal `ArrayDeque` with a `PriorityQueue` ordered by task priority; the `offer`/`take` contract stays identical, so `WorkerThread`/`CustomThreadPoolExecutor` are untouched.
- **"Prevent one slow/misbehaving client from starving others"** → add a per-connection timeout in `RequestHandler`, or route different request classes to separate pools (bulkheading) — the `ConcurrencyModel`/`RequestHandler` seam already supports this without touching the pool internals.
- **"Add work-stealing between worker threads for better load balancing"** → give each `WorkerThread` its own local deque instead of one shared queue, with idle workers stealing from busy ones' queues — a deeper internal change to `BlockingTaskQueue`'s structure, but the public `submit()`/`Task` contract at the `CustomThreadPoolExecutor` level is unaffected.
- **"Add TLS/SSL support"** → wraps `SocketChannel`/`Socket` handling inside `RequestHandler` with an `SSLEngine` — isolated to the I/O layer, doesn't touch pool or scheduling logic.

---

## 6. Key interview talking points (concurrency correctness)

- **Why `synchronized` + `wait`/`notify` in `BlockingTaskQueue`** instead of just an `ArrayDeque`: without synchronization, two worker threads could both see `queue.isEmpty()` as false, race to poll, and one gets `null`/throws — the monitor lock plus condition wait is what makes producer-consumer handoff safe.
- **Why core vs non-core threads differ** (`take()` blocks forever vs `poll(timeout)`): mirrors Java's real `ThreadPoolExecutor` — core threads are a standing investment, extra threads are elastic capacity that should shrink back down when load drops, avoiding idle thread resource waste.
- **Why the Reactor thread must never block on `workerPool.submit(...)` doing real work directly**: a single slow request would stall the entire event loop and freeze every other connection — this is the most common mistake when implementing Reactor-style servers.
- **Graceful vs immediate shutdown**: `shutdown()` (drain) vs `shutdownNow()` (abandon) is a distinction interviewers frequently probe — make sure to articulate the tradeoff (data loss vs shutdown latency).

---

Want me to extend this with **a work-stealing thread pool implementation, a full Reactor→multi-reactor (Netty-style boss/worker event-loop groups) design, connection pooling for outbound calls, or a bulkhead/circuit-breaker layer for isolating slow downstream dependencies**, or move to a different LLD problem?