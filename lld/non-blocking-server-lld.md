# Non-Blocking Server — Full Implementation (Java NIO)

This is a working, runnable implementation of a **multi-reactor non-blocking server** (the architecture Netty/nginx use internally): one **boss reactor** accepts connections, and hands each off to one of several **worker reactors**, each running its own event loop on its own thread. This gets the connection-handling architecture right, which is the part people usually get wrong when implementing this from scratch.

## 1. What "non-blocking" actually requires (the parts people get wrong)

Before the code — three correctness issues that separate a working NIO server from a broken one:

1. **A `Selector` can only be safely touched by the thread that owns it.** You can't just grab a worker reactor's selector from the boss thread and call `channel.register(selector, ...)` — that races with the worker's `select()` call. You must queue a registration task and call `selector.wakeup()` to interrupt the blocked `select()`, then have the worker thread perform the registration itself.
2. **`SocketChannel.write()` can write fewer bytes than you gave it** (the OS send buffer fills up). You cannot just call `write()` once and assume it's done — you must track how much of the buffer remains unwritten, register `OP_WRITE` interest, and keep writing on subsequent write-ready events until the buffer drains. This is the single most common bug in from-scratch NIO servers.
3. **The reactor thread must never block.** Any slow/CPU-heavy work discovered during a read event must be handed off to a worker pool — not processed inline — or every other connection on that reactor thread stalls.

## 2. Patterns used

| Pattern | Where | Why |
|---|---|---|
| **Reactor** | `SubReactor` — single-threaded event loop demultiplexing I/O readiness via `Selector` | Core architecture: one thread handles readiness for many connections instead of one thread per connection. |
| **Multi-Reactor (Boss/Worker)** | `BossReactor` accepts only; `SubReactor[]` handle read/write, chosen round-robin | Separates the accept path from the read/write path and spreads connection load across multiple event-loop threads — this is exactly Netty's `EventLoopGroup` boss/worker split. |
| **Command** | Cross-thread registration is done via a `Runnable` task queue drained at the top of each reactor's loop | Solves problem #1 above — registration must run *on* the owning thread; queuing a command and waking the selector is the safe way to cross that thread boundary. |
| **Strategy** | `ProtocolHandler` interface — `EchoProtocolHandler`, could add `HttpProtocolHandler`, etc. | What the server actually *does* with bytes read is independent of the reactor mechanics; new protocols plug in without touching the reactor. |
| **State (lightweight)** | `ConnectionContext` tracks per-channel read/write buffers and pending-write state | Each connection's partial-write progress must persist across multiple `OP_WRITE` events — this is per-channel mutable state, not global. |

---

## 3. Code

### 3.1 ConnectionContext — per-channel state (solves partial writes)

```java
import java.nio.ByteBuffer;
import java.nio.channels.SocketChannel;
import java.util.ArrayDeque;
import java.util.Deque;

public class ConnectionContext {
    final SocketChannel channel;
    final ByteBuffer readBuffer = ByteBuffer.allocate(4096);
    // Queue of buffers waiting to be written — a write() call may only drain some of the head buffer.
    final Deque<ByteBuffer> pendingWrites = new ArrayDeque<>();

    ConnectionContext(SocketChannel channel) {
        this.channel = channel;
    }

    void queueWrite(ByteBuffer buffer) {
        pendingWrites.addLast(buffer);
    }

    boolean hasPendingWrites() {
        return !pendingWrites.isEmpty();
    }
}
```

### 3.2 ProtocolHandler — Strategy for what to do with data read

```java
import java.nio.ByteBuffer;

public interface ProtocolHandler {
    /**
     * Called when bytes have been read from a connection.
     * @return the response to write back, or null if no response yet (e.g. waiting for more data)
     */
    ByteBuffer onData(ConnectionContext ctx, ByteBuffer data);

    default void onConnect(ConnectionContext ctx) {}
    default void onDisconnect(ConnectionContext ctx) {}
}

public class EchoProtocolHandler implements ProtocolHandler {
    @Override
    public ByteBuffer onData(ConnectionContext ctx, ByteBuffer data) {
        // echo back exactly what was received
        ByteBuffer response = ByteBuffer.allocate(data.remaining());
        response.put(data);
        response.flip();
        return response;
    }

    @Override
    public void onConnect(ConnectionContext ctx) {
        System.out.println("[Connect] " + describeRemote(ctx));
    }

    @Override
    public void onDisconnect(ConnectionContext ctx) {
        System.out.println("[Disconnect] " + describeRemote(ctx));
    }

    private String describeRemote(ConnectionContext ctx) {
        try {
            return ctx.channel.getRemoteAddress().toString();
        } catch (Exception e) {
            return "unknown";
        }
    }
}
```

### 3.3 SubReactor — the worker event loop (this is the core piece)

```java
import java.io.IOException;
import java.net.StandardSocketOptions;
import java.nio.ByteBuffer;
import java.nio.channels.*;
import java.util.Iterator;
import java.util.Queue;
import java.util.concurrent.ConcurrentLinkedQueue;

public class SubReactor implements Runnable {
    private final Selector selector;
    private final ProtocolHandler protocolHandler;
    private final Queue<Runnable> pendingTasks = new ConcurrentLinkedQueue<>();
    private volatile boolean running = true;
    private final String name;

    public SubReactor(String name, ProtocolHandler protocolHandler) throws IOException {
        this.name = name;
        this.selector = Selector.open();
        this.protocolHandler = protocolHandler;
    }

    /**
     * Called from ANOTHER thread (the boss reactor) to hand off a newly-accepted channel.
     * Registration itself must happen on this reactor's own thread — so we queue it
     * and wake the selector, rather than touching the selector directly here.
     */
    public void registerChannel(SocketChannel channel) {
        pendingTasks.offer(() -> {
            try {
                channel.configureBlocking(false);
                SelectionKey key = channel.register(selector, SelectionKey.OP_READ);
                ConnectionContext ctx = new ConnectionContext(channel);
                key.attach(ctx);
                protocolHandler.onConnect(ctx);
            } catch (IOException e) {
                closeQuietly(channel);
            }
        });
        selector.wakeup(); // interrupt the blocked select() so it picks up the pending task
    }

    @Override
    public void run() {
        while (running) {
            try {
                selector.select(); // blocks until a channel is ready, or wakeup() is called
                drainPendingTasks();

                Iterator<SelectionKey> keys = selector.selectedKeys().iterator();
                while (keys.hasNext()) {
                    SelectionKey key = keys.next();
                    keys.remove();
                    if (!key.isValid()) continue;

                    try {
                        if (key.isReadable()) handleRead(key);
                        else if (key.isWritable()) handleWrite(key);
                    } catch (IOException e) {
                        closeConnection(key);
                    }
                }
            } catch (IOException e) {
                if (running) System.err.println("[" + name + "] Selector error: " + e.getMessage());
            }
        }
    }

    private void drainPendingTasks() {
        Runnable task;
        while ((task = pendingTasks.poll()) != null) {
            task.run();
        }
    }

    private void handleRead(SelectionKey key) throws IOException {
        SocketChannel channel = (SocketChannel) key.channel();
        ConnectionContext ctx = (ConnectionContext) key.attachment();

        ctx.readBuffer.clear();
        int bytesRead = channel.read(ctx.readBuffer);

        if (bytesRead == -1) {
            closeConnection(key); // client closed connection
            return;
        }
        if (bytesRead == 0) return; // spurious wakeup, nothing to do

        ctx.readBuffer.flip();
        ByteBuffer response = protocolHandler.onData(ctx, ctx.readBuffer);

        if (response != null) {
            ctx.queueWrite(response);
            // OP_WRITE only registered when there's actually something to write —
            // leaving it always-on would cause the selector to constantly fire (busy-loop),
            // since a socket is almost always "writable" when idle.
            key.interestOps(SelectionKey.OP_READ | SelectionKey.OP_WRITE);
        }
    }

    private void handleWrite(SelectionKey key) throws IOException {
        SocketChannel channel = (SocketChannel) key.channel();
        ConnectionContext ctx = (ConnectionContext) key.attachment();

        while (ctx.hasPendingWrites()) {
            ByteBuffer buffer = ctx.pendingWrites.peekFirst();
            channel.write(buffer); // may write fewer bytes than buffer.remaining()

            if (buffer.hasRemaining()) {
                // socket buffer is full — stop for now, we'll resume on the next OP_WRITE event
                return;
            }
            ctx.pendingWrites.pollFirst(); // this buffer is fully flushed, move to the next
        }

        // everything queued has been written — stop listening for writability
        // (otherwise the reactor spins: an idle-but-writable socket fires OP_WRITE constantly)
        key.interestOps(SelectionKey.OP_READ);
    }

    private void closeConnection(SelectionKey key) {
        SocketChannel channel = (SocketChannel) key.channel();
        ConnectionContext ctx = (ConnectionContext) key.attachment();
        if (ctx != null) protocolHandler.onDisconnect(ctx);
        key.cancel();
        closeQuietly(channel);
    }

    private void closeQuietly(SocketChannel channel) {
        try { channel.close(); } catch (IOException ignored) {}
    }

    public void shutdown() {
        running = false;
        selector.wakeup();
    }
}
```

### 3.4 BossReactor — accepts connections only, hands off to workers

```java
import java.io.IOException;
import java.net.InetSocketAddress;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.ServerSocketChannel;
import java.nio.channels.SocketChannel;
import java.util.Iterator;
import java.util.concurrent.atomic.AtomicInteger;

public class BossReactor implements Runnable {
    private final int port;
    private final SubReactor[] workerReactors;
    private final AtomicInteger roundRobinIndex = new AtomicInteger(0);
    private Selector selector;
    private volatile boolean running = true;

    public BossReactor(int port, SubReactor[] workerReactors) {
        this.port = port;
        this.workerReactors = workerReactors;
    }

    @Override
    public void run() {
        try {
            ServerSocketChannel serverChannel = ServerSocketChannel.open();
            serverChannel.bind(new InetSocketAddress(port));
            serverChannel.configureBlocking(false);

            selector = Selector.open();
            serverChannel.register(selector, SelectionKey.OP_ACCEPT);
            System.out.println("[Boss] Listening on port " + port);

            while (running) {
                selector.select();
                Iterator<SelectionKey> keys = selector.selectedKeys().iterator();
                while (keys.hasNext()) {
                    SelectionKey key = keys.next();
                    keys.remove();
                    if (key.isAcceptable()) accept(key);
                }
            }
        } catch (IOException e) {
            if (running) System.err.println("[Boss] Error: " + e.getMessage());
        }
    }

    private void accept(SelectionKey key) throws IOException {
        ServerSocketChannel serverChannel = (ServerSocketChannel) key.channel();
        SocketChannel client = serverChannel.accept();
        if (client == null) return;

        // round-robin dispatch across worker reactors — spreads connections evenly
        int idx = roundRobinIndex.getAndIncrement() % workerReactors.length;
        workerReactors[idx].registerChannel(client);
    }

    public void shutdown() {
        running = false;
        if (selector != null) selector.wakeup();
    }
}
```

### 3.5 NonBlockingServer — bootstrap

```java
public class NonBlockingServer {
    private final BossReactor bossReactor;
    private final SubReactor[] workerReactors;
    private final Thread bossThread;
    private final Thread[] workerThreads;

    public NonBlockingServer(int port, int numWorkers, ProtocolHandler protocolHandler) throws Exception {
        workerReactors = new SubReactor[numWorkers];
        workerThreads = new Thread[numWorkers];
        for (int i = 0; i < numWorkers; i++) {
            workerReactors[i] = new SubReactor("worker-" + i, protocolHandler);
        }

        bossReactor = new BossReactor(port, workerReactors);
        bossThread = new Thread(bossReactor, "boss-reactor");
    }

    public void start() {
        for (int i = 0; i < workerReactors.length; i++) {
            workerThreads[i] = new Thread(workerReactors[i], "worker-reactor-" + i);
            workerThreads[i].start();
        }
        bossThread.start();
    }

    public void shutdown() {
        bossReactor.shutdown();
        for (SubReactor r : workerReactors) r.shutdown();
    }

    public static void main(String[] args) throws Exception {
        int port = 8080;
        int numWorkers = Runtime.getRuntime().availableProcessors();

        NonBlockingServer server = new NonBlockingServer(port, numWorkers, new EchoProtocolHandler());
        server.start();

        Runtime.getRuntime().addShutdownHook(new Thread(server::shutdown));

        System.out.println("Non-blocking server started with " + numWorkers + " worker reactors");
        // test with: telnet localhost 8080  (or `nc localhost 8080`)
    }
}
```

---

## 4. How to verify it actually works

```bash
javac *.java
java NonBlockingServer
```

In another terminal:
```bash
nc localhost 8080
hello    # server echoes back "hello"
```

Open several `nc` sessions simultaneously — you'll see connections distributed round-robin across worker reactors (add a log line in `registerChannel` printing the reactor name to confirm), and all connections stay responsive concurrently despite each reactor being single-threaded, because no reactor thread ever blocks on I/O.

To stress the partial-write path specifically: have `EchoProtocolHandler.onData` return a very large buffer (a few MB) in response to a tiny input — on a slow/congested client, you'll observe `handleWrite` getting called multiple times as the socket buffer drains, proving the partial-write handling is load-bearing and not just defensive dead code.

---

## 5. Offloading blocking/CPU-heavy work (the piece that connects back to the thread pool design)

If `onData` needs to do something slow (DB call, heavy computation), doing it inline inside `handleRead` would stall that entire reactor thread — freezing every other connection assigned to it. The fix: hand the work to the `CustomThreadPoolExecutor` from the earlier design, and have the worker thread write the response back onto the reactor thread via the same pending-task-queue + `wakeup()` mechanism used for registration:

```java
public class OffloadingProtocolHandler implements ProtocolHandler {
    private final CustomThreadPoolExecutor businessLogicPool;
    private final SubReactor ownerReactor; // the reactor this connection is registered on

    public OffloadingProtocolHandler(CustomThreadPoolExecutor pool, SubReactor ownerReactor) {
        this.businessLogicPool = pool;
        this.ownerReactor = ownerReactor;
    }

    @Override
    public ByteBuffer onData(ConnectionContext ctx, ByteBuffer data) {
        byte[] payload = new byte[data.remaining()];
        data.get(payload);

        businessLogicPool.submit(() -> {
            ByteBuffer response = doSlowWork(payload); // heavy work happens off the reactor thread
            ownerReactor.registerChannel(null); // conceptually: queue a "write this response" task + wakeup()
            // (in a full implementation, add a queueWriteTask(ctx, response) method to SubReactor
            //  mirroring registerChannel's pending-task-queue + wakeup pattern)
        });

        return null; // no immediate response — it'll be written asynchronously once ready
    }

    private ByteBuffer doSlowWork(byte[] payload) { /* ... */ return ByteBuffer.wrap(payload); }
}
```

This is exactly why `SubReactor.registerChannel` was built around a **generic pending-task-queue + wakeup**, rather than a method specific to registration — the same cross-thread-handoff mechanism generalizes to "queue a write" once you add that second task type.

---

Want me to extend this with the **write-task queueing addition sketched above (fully wired, not just conceptual)**, a **length-prefixed or HTTP protocol handler** to replace the echo example, **backpressure on reads** (pausing `OP_READ` when downstream can't keep up), or **benchmarking code comparing this against the thread-per-connection model** from the earlier design?