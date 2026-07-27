# Pub-Sub Messaging System — LLD

## 1. Requirements

**Functional**
- Publishers publish messages to named topics; multiple subscribers can subscribe to a topic.
- Support both **push** delivery (broker calls subscriber's callback) and **pull** delivery (subscriber polls for messages).
- Support **consumer groups** — within a group, each message goes to only one subscriber (queue semantics); across groups, every group gets every message (broadcast semantics). This is exactly how Kafka consumer groups work.
- Message acknowledgment — unacked messages are redelivered after a timeout.
- Retry failed deliveries with backoff; move to dead-letter topic after max retries.
- Message filtering — subscriber only receives messages matching certain criteria.
- Ordering guarantee within a partition/topic (at least for single-partition case).
- Message persistence for replay (new subscriber can read from beginning, or from an offset).

**Non-functional**
- Decouple publishers completely from subscribers — publisher shouldn't know who/how many are listening.
- New delivery mechanisms (push/pull/webhook) pluggable without touching core broker logic.
- New retry/ack policies pluggable independently.
- Message filtering/transformation composable without modifying core delivery path.
- Adding a subscriber shouldn't require touching `Topic` or `Publisher` code.

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **Observer** | `Topic` (Subject) maintains `Subscriber` list, notifies on `publish()` | This *is* pub-sub — it's the textbook Observer pattern at system scale. Publisher fires an event; `Topic` fans it out to all interested subscribers without either side knowing about the other. |
| **Strategy** | `DeliveryStrategy` (`PushDelivery`, `PullDelivery`); `AckPolicy` (`AutoAck`, `ManualAck`); `RetryPolicy` (`FixedDelayRetry`, `ExponentialBackoffRetry`) | How a message gets to a subscriber (push vs pull), how acknowledgment is handled, and how retries are timed are all independent axes of variation — isolating them avoids one broker class riddled with mode-specific conditionals. |
| **State** | `MessageState`: `PublishedState`, `DeliveredState`, `AckedState`, `RetryingState`, `DeadLetteredState` | A message's legal next actions depend entirely on its current delivery state (can't ack an already-acked message, can't retry a message that's been acked). |
| **Chain of Responsibility** | `MessageInterceptor` chain: `FilterInterceptor` → `TransformInterceptor` → `DeduplicationInterceptor` | Message processing before delivery is a sequence of independent, addable/removable steps — new interceptor (e.g., PII redaction) = new link, nothing else touched. |
| **Singleton** | `MessageBroker` | Single central registry of all topics and subscriptions — must be one source of truth for routing, exactly like `MatchRegistry`/`JobScheduler` in earlier designs. |
| **Command** | `Message` wraps payload + metadata and is queued/retried/replayed as a unit | A message needs to be stored, redelivered, and retried without the broker caring what's inside — Command gives it a uniform `deliverTo(subscriber)` contract. |
| **Factory Method** | `SubscriptionFactory.create(topic, mode, ...)` | Encapsulates which `DeliveryStrategy` + `AckPolicy` + `RetryPolicy` combination corresponds to a given subscription mode. |
| **Builder** | `Message.Builder`, `Subscription.Builder` | Many optional fields (headers, partition key, filter criteria). |

**SOLID**
- **S**: `Topic` only routes; `DeliveryStrategy` only delivers; `RetryPolicy` only computes retry timing; `MessageInterceptor` only transforms/filters.
- **O**: New delivery mode → new `DeliveryStrategy`. New retry policy → new `RetryPolicy`. New processing step → new interceptor link. Nothing existing changes.
- **L**: Any `DeliveryStrategy`/`AckPolicy`/`RetryPolicy` substitutable wherever used; any `MessageState` substitutable in delegation.
- **I**: `Subscriber` interface exposes only `onMessage`; `MessageInterceptor` exposes only `intercept` — narrow, focused contracts.
- **D**: `Topic`/`MessageBroker` depend on `DeliveryStrategy`, `Subscriber`, `RetryPolicy` abstractions injected at subscription time, never concrete classes.

---

## 3. Class Diagram (textual)

```
┌───────────────────┐         ┌──────────────────────────┐
│  Subscriber            │◀────────│  Topic (Observer Subject)    │
│ (Observer interface)     │        │ - subscribers: List<Subscription>│
│ + onMessage(Message)      │       │ - partitions / message log        │
└───────────────────┘         │ + publish(Message)                  │
                                │ + subscribe(Subscription)             │
                                └──────────────────────────┘

┌────────────────────┐       ┌────────────────────────┐
│  DeliveryStrategy       │     │  AckPolicy                    │
│ (Strategy interface)      │   │ (Strategy interface)             │
│ + deliver(msg, subscriber)  │ │ + onDelivered(msg)                 │
└──────────▲───────────┘     │ + isAcked(msg): bool                │
    ┌──────┼──────┐          └──────────▲───────────┘
PushDelivery  PullDelivery       ┌───────┼────────┐
                                AutoAck          ManualAck

┌────────────────────┐       ┌────────────────────────┐
│  RetryPolicy            │     │  MessageInterceptor          │
│ (Strategy interface)      │   │ (Chain of Responsibility)      │
│ + shouldRetry(attempt)      │ │ + intercept(msg): Message|null   │
│ + nextRetryDelay(attempt)     │ │ + setNext(interceptor)           │
└──────────▲───────────┘     └──────────▲───────────┘
   ┌───────┼────────┐           ┌───────┼────────┬─────────────┐
FixedDelay  Exponential      FilterInterceptor TransformInterceptor DedupInterceptor
 Retry        BackoffRetry

┌───────────────────┐        ┌────────────────────────┐
│  MessageState          │      │  Message (Command)          │
│ (State interface)        │◀────│ - id, payload, headers        │
│ + deliver()/ack()/         │   │ - state: MessageState           │
│   retry()/deadLetter()       │  │ - partitionKey, timestamp         │
└────────▲──────────────┘     └────────────────────────┘
  ┌──────┼──────┬───────────┬──────────────┐
Published Delivered Acked   Retrying   DeadLettered
 State      State    State    State       State

┌───────────────────┐        ┌────────────────────────┐
│  Subscription           │     │  MessageBroker              │
│ - subscriber              │   │  (Singleton)                   │
│ - deliveryStrategy          │  │  + createTopic(name)             │
│ - ackPolicy                  │ │  + getTopic(name)                  │
│ - retryPolicy                  │ │  - topics: Map<String, Topic>       │
│ - consumerGroup (nullable)      │└────────────────────────┘
│ - filterCriteria
└───────────────────┘

┌───────────────────┐
│  SubscriptionFactory     │
└───────────────────┘
```

---

## 4. Code (Java)

### 4.1 Message — the Command/data unit

```java
public class Message {
    private final String id;
    private final String topicName;
    private final Object payload;
    private final Map<String, String> headers;
    private final String partitionKey; // for ordering/consumer-group routing
    private final long timestamp;
    private MessageState state = new PublishedState();
    private int attemptCount = 0;

    private Message(Builder b) {
        this.id = b.id; this.topicName = b.topicName; this.payload = b.payload;
        this.headers = b.headers; this.partitionKey = b.partitionKey;
        this.timestamp = System.currentTimeMillis();
    }

    void setState(MessageState s) { this.state = s; }
    public MessageState getState() { return state; }
    public String getStateName() { return state.name(); }
    int incrementAttempt() { return ++attemptCount; }
    public int getAttemptCount() { return attemptCount; }

    public String getId() { return id; }
    public Object getPayload() { return payload; }
    public Map<String, String> getHeaders() { return headers; }
    public String getPartitionKey() { return partitionKey; }
    public String getTopicName() { return topicName; }

    public static class Builder {
        private String id = UUID.randomUUID().toString();
        private String topicName; private Object payload;
        private Map<String, String> headers = new HashMap<>();
        private String partitionKey;

        public Builder topic(String t) { this.topicName = t; return this; }
        public Builder payload(Object p) { this.payload = p; return this; }
        public Builder header(String k, String v) { this.headers.put(k, v); return this; }
        public Builder partitionKey(String k) { this.partitionKey = k; return this; }
        public Message build() { return new Message(this); }
    }
}
```

### 4.2 State pattern — Message delivery lifecycle

```java
public interface MessageState {
    void deliver(Message msg);
    void ack(Message msg);
    void retry(Message msg);
    void deadLetter(Message msg);
    String name();
}

public class PublishedState implements MessageState {
    public void deliver(Message msg) { msg.setState(new DeliveredState()); }
    public void ack(Message msg) { throw new IllegalStateException("Not yet delivered"); }
    public void retry(Message msg) { throw new IllegalStateException("Not yet delivered"); }
    public void deadLetter(Message msg) { throw new IllegalStateException("Not yet delivered"); }
    public String name() { return "PUBLISHED"; }
}

public class DeliveredState implements MessageState {
    public void deliver(Message msg) { throw new IllegalStateException("Already delivered"); }
    public void ack(Message msg) { msg.setState(new AckedState()); }
    public void retry(Message msg) { msg.setState(new RetryingState()); }
    public void deadLetter(Message msg) { msg.setState(new DeadLetteredState()); }
    public String name() { return "DELIVERED"; }
}

public class AckedState implements MessageState {
    public void deliver(Message msg) { throw new IllegalStateException("Already acked"); }
    public void ack(Message msg) { throw new IllegalStateException("Already acked"); }
    public void retry(Message msg) { throw new IllegalStateException("Already acked"); }
    public void deadLetter(Message msg) { throw new IllegalStateException("Already acked"); }
    public String name() { return "ACKED"; }
}

public class RetryingState implements MessageState {
    public void deliver(Message msg) { msg.setState(new DeliveredState()); }
    public void ack(Message msg) { msg.setState(new AckedState()); }
    public void retry(Message msg) { throw new IllegalStateException("Already retrying"); }
    public void deadLetter(Message msg) { msg.setState(new DeadLetteredState()); }
    public String name() { return "RETRYING"; }
}

public class DeadLetteredState implements MessageState {
    public void deliver(Message msg) { throw new IllegalStateException("Dead-lettered"); }
    public void ack(Message msg) { throw new IllegalStateException("Dead-lettered"); }
    public void retry(Message msg) { throw new IllegalStateException("Dead-lettered"); }
    public void deadLetter(Message msg) { throw new IllegalStateException("Already dead-lettered"); }
    public String name() { return "DEAD_LETTERED"; }
}
```

### 4.3 Subscriber interface (Observer)

```java
@FunctionalInterface
public interface Subscriber {
    void onMessage(Message message);
}
```

### 4.4 Strategy — Delivery mode (push vs pull)

```java
public interface DeliveryStrategy {
    void deliver(Message message, Subscription subscription);
}

public class PushDelivery implements DeliveryStrategy {
    private final RetryManager retryManager;
    public PushDelivery(RetryManager retryManager) { this.retryManager = retryManager; }

    @Override
    public void deliver(Message message, Subscription subscription) {
        try {
            message.getState().deliver(message);
            subscription.getSubscriber().onMessage(message);
            if (subscription.getAckPolicy() instanceof AutoAck) {
                message.getState().ack(message);
            }
            // ManualAck: subscriber must call subscription.ack(message.getId()) explicitly
            retryManager.scheduleAckTimeoutCheck(message, subscription);
        } catch (Exception e) {
            retryManager.handleFailure(message, subscription);
        }
    }
}

public class PullDelivery implements DeliveryStrategy {
    // messages sit in a per-subscription buffer; subscriber calls poll() to retrieve
    @Override
    public void deliver(Message message, Subscription subscription) {
        message.getState().deliver(message);
        subscription.enqueueForPull(message); // buffered, not pushed
    }
}
```

### 4.5 Strategy — Ack Policy

```java
public interface AckPolicy {
    boolean requiresExplicitAck();
}

public class AutoAck implements AckPolicy {
    public boolean requiresExplicitAck() { return false; }
}

public class ManualAck implements AckPolicy {
    public boolean requiresExplicitAck() { return true; }
}
```

### 4.6 Strategy — Retry Policy + RetryManager

```java
public interface RetryPolicy {
    boolean shouldRetry(int attemptNumber);
    long nextRetryDelay(int attemptNumber);
}

public class FixedDelayRetry implements RetryPolicy {
    private final int maxAttempts; private final long delayMs;
    public FixedDelayRetry(int maxAttempts, long delayMs) { this.maxAttempts = maxAttempts; this.delayMs = delayMs; }
    public boolean shouldRetry(int attempt) { return attempt < maxAttempts; }
    public long nextRetryDelay(int attempt) { return delayMs; }
}

public class ExponentialBackoffRetry implements RetryPolicy {
    private final int maxAttempts; private final long baseDelayMs;
    public ExponentialBackoffRetry(int maxAttempts, long baseDelayMs) { this.maxAttempts = maxAttempts; this.baseDelayMs = baseDelayMs; }
    public boolean shouldRetry(int attempt) { return attempt < maxAttempts; }
    public long nextRetryDelay(int attempt) { return baseDelayMs * (long) Math.pow(2, attempt - 1); }
}

public class RetryManager {
    private final ScheduledExecutorService scheduler = Executors.newScheduledThreadPool(4);
    private final Map<String, ScheduledFuture<?>> pendingAckChecks = new ConcurrentHashMap<>();

    public void handleFailure(Message message, Subscription subscription) {
        int attempt = message.incrementAttempt();
        RetryPolicy policy = subscription.getRetryPolicy();
        if (policy.shouldRetry(attempt)) {
            message.getState().retry(message);
            long delay = policy.nextRetryDelay(attempt);
            scheduler.schedule(() -> subscription.getDeliveryStrategy().deliver(message, subscription),
                    delay, TimeUnit.MILLISECONDS);
        } else {
            message.getState().deadLetter(message);
            DeadLetterTopic.getInstance().publish(message);
        }
    }

    // used with ManualAck: if not acked within timeout, treat as failure and retry
    public void scheduleAckTimeoutCheck(Message message, Subscription subscription) {
        if (!subscription.getAckPolicy().requiresExplicitAck()) return;
        ScheduledFuture<?> future = scheduler.schedule(() -> {
            if (!message.getStateName().equals("ACKED")) {
                handleFailure(message, subscription);
            }
        }, 30, TimeUnit.SECONDS); // ack timeout
        pendingAckChecks.put(message.getId(), future);
    }

    public void cancelAckTimeoutCheck(String messageId) {
        ScheduledFuture<?> f = pendingAckChecks.remove(messageId);
        if (f != null) f.cancel(false);
    }
}
```

### 4.7 Chain of Responsibility — Message Interceptors

```java
public abstract class MessageInterceptor {
    protected MessageInterceptor next;
    public MessageInterceptor setNext(MessageInterceptor next) { this.next = next; return next; }

    /** @return processed message, or null to drop it (e.g., filtered out / duplicate) */
    public final Message intercept(Message message) {
        Message result = doIntercept(message);
        if (result == null) return null; // dropped — stop the chain
        return next != null ? next.intercept(result) : result;
    }
    protected abstract Message doIntercept(Message message);
}

public class FilterInterceptor extends MessageInterceptor {
    private final Predicate<Message> criteria;
    public FilterInterceptor(Predicate<Message> criteria) { this.criteria = criteria; }
    @Override
    protected Message doIntercept(Message message) {
        return criteria.test(message) ? message : null;
    }
}

public class DeduplicationInterceptor extends MessageInterceptor {
    private final Set<String> seenIds = ConcurrentHashMap.newKeySet();
    @Override
    protected Message doIntercept(Message message) {
        return seenIds.add(message.getId()) ? message : null; // null if already seen
    }
}

public class TransformInterceptor extends MessageInterceptor {
    private final Function<Message, Message> transformer;
    public TransformInterceptor(Function<Message, Message> transformer) { this.transformer = transformer; }
    @Override
    protected Message doIntercept(Message message) { return transformer.apply(message); }
}
```

### 4.8 Subscription — binds a subscriber to its delivery/ack/retry behavior

```java
public class Subscription {
    private final String id;
    private final Subscriber subscriber;
    private final DeliveryStrategy deliveryStrategy;
    private final AckPolicy ackPolicy;
    private final RetryPolicy retryPolicy;
    private final String consumerGroup; // null = independent subscriber (broadcast to this one too)
    private final MessageInterceptor interceptorChain; // nullable
    private final BlockingQueue<Message> pullBuffer = new LinkedBlockingQueue<>(); // for PullDelivery

    private Subscription(Builder b) {
        this.id = b.id; this.subscriber = b.subscriber; this.deliveryStrategy = b.deliveryStrategy;
        this.ackPolicy = b.ackPolicy; this.retryPolicy = b.retryPolicy;
        this.consumerGroup = b.consumerGroup; this.interceptorChain = b.interceptorChain;
    }

    public void ack(String messageId) {
        // in a real impl, look up the in-flight Message by id; simplified here
        RetryManager.getInstance().cancelAckTimeoutCheck(messageId);
    }

    void enqueueForPull(Message message) { pullBuffer.offer(message); }
    public Message poll() { return pullBuffer.poll(); } // subscriber-driven retrieval

    public Subscriber getSubscriber() { return subscriber; }
    public DeliveryStrategy getDeliveryStrategy() { return deliveryStrategy; }
    public AckPolicy getAckPolicy() { return ackPolicy; }
    public RetryPolicy getRetryPolicy() { return retryPolicy; }
    public String getConsumerGroup() { return consumerGroup; }
    public MessageInterceptor getInterceptorChain() { return interceptorChain; }
    public String getId() { return id; }

    public static class Builder {
        private String id = UUID.randomUUID().toString();
        private Subscriber subscriber; private DeliveryStrategy deliveryStrategy;
        private AckPolicy ackPolicy = new AutoAck();
        private RetryPolicy retryPolicy = new FixedDelayRetry(3, 1000);
        private String consumerGroup; private MessageInterceptor interceptorChain;

        public Builder subscriber(Subscriber s) { this.subscriber = s; return this; }
        public Builder deliveryStrategy(DeliveryStrategy d) { this.deliveryStrategy = d; return this; }
        public Builder ackPolicy(AckPolicy a) { this.ackPolicy = a; return this; }
        public Builder retryPolicy(RetryPolicy r) { this.retryPolicy = r; return this; }
        public Builder consumerGroup(String g) { this.consumerGroup = g; return this; }
        public Builder interceptorChain(MessageInterceptor i) { this.interceptorChain = i; return this; }
        public Subscription build() { return new Subscription(this); }
    }
}
```

### 4.9 Topic — the Observer Subject, with consumer-group-aware fan-out

```java
public class Topic {
    private final String name;
    // subscriptions grouped by consumerGroup; null-group subscriptions are independent broadcast receivers
    private final Map<String, List<Subscription>> groupSubscriptions = new ConcurrentHashMap<>();
    private final AtomicInteger roundRobinCounter = new AtomicInteger(0);
    private final List<Message> messageLog = new CopyOnWriteArrayList<>(); // enables replay

    public Topic(String name) { this.name = name; }

    public void subscribe(Subscription subscription) {
        String groupKey = subscription.getConsumerGroup() != null
                ? subscription.getConsumerGroup()
                : "indep-" + subscription.getId(); // each independent subscriber is its own "group of one"
        groupSubscriptions.computeIfAbsent(groupKey, k -> new CopyOnWriteArrayList<>()).add(subscription);
    }

    public void publish(Message message) {
        messageLog.add(message); // persisted for replay

        for (List<Subscription> group : groupSubscriptions.values()) {
            if (group.isEmpty()) continue;

            Subscription target = selectSubscriberFromGroup(group); // queue semantics within a group
            Message processed = applyInterceptors(message, target);
            if (processed == null) continue; // filtered/deduped out for this subscriber

            target.getDeliveryStrategy().deliver(processed, target);
        }
    }

    private Subscription selectSubscriberFromGroup(List<Subscription> group) {
        // round-robin load balancing within a consumer group (like Kafka)
        int idx = roundRobinCounter.getAndIncrement() % group.size();
        return group.get(idx);
    }

    private Message applyInterceptors(Message message, Subscription sub) {
        return sub.getInterceptorChain() != null ? sub.getInterceptorChain().intercept(message) : message;
    }

    public List<Message> replayFrom(long timestamp) {
        return messageLog.stream().filter(m -> m.getTimestamp() >= timestamp).collect(Collectors.toList());
    }

    public String getName() { return name; }
}
```

Wait — `Message` doesn't expose `getTimestamp()` publicly above; add a getter. (Noted for completeness — trivial fix.)

### 4.10 Singleton — MessageBroker (central registry)

```java
public class MessageBroker {
    private static volatile MessageBroker instance;
    private final ConcurrentHashMap<String, Topic> topics = new ConcurrentHashMap<>();

    private MessageBroker() {}

    public static MessageBroker getInstance() {
        if (instance == null) {
            synchronized (MessageBroker.class) {
                if (instance == null) instance = new MessageBroker();
            }
        }
        return instance;
    }

    public Topic createTopic(String name) {
        return topics.computeIfAbsent(name, Topic::new);
    }

    public Topic getTopic(String name) {
        Topic t = topics.get(name);
        if (t == null) throw new IllegalArgumentException("No such topic: " + name);
        return t;
    }

    public void publish(String topicName, Message message) {
        getTopic(topicName).publish(message);
    }
}
```

### 4.11 Dead-letter topic (special sink)

```java
public class DeadLetterTopic {
    private static final DeadLetterTopic instance = new DeadLetterTopic();
    private final List<Message> deadLetters = new CopyOnWriteArrayList<>();

    public static DeadLetterTopic getInstance() { return instance; }
    public void publish(Message message) {
        deadLetters.add(message);
        System.out.println("[DLQ] Message " + message.getId() + " moved to dead-letter after max retries");
    }
    public List<Message> getAll() { return deadLetters; }
}
```

### 4.12 Factory Method — SubscriptionFactory

```java
public class SubscriptionFactory {
    public static Subscription createPushSubscriber(Subscriber subscriber, String consumerGroup,
                                                      RetryPolicy retryPolicy, MessageInterceptor chain) {
        return new Subscription.Builder()
                .subscriber(subscriber)
                .deliveryStrategy(new PushDelivery(RetryManager.getInstance()))
                .ackPolicy(new AutoAck())
                .retryPolicy(retryPolicy)
                .consumerGroup(consumerGroup)
                .interceptorChain(chain)
                .build();
    }

    public static Subscription createPullSubscriber(String consumerGroup) {
        return new Subscription.Builder()
                .subscriber(msg -> {}) // unused in pull mode
                .deliveryStrategy(new PullDelivery())
                .ackPolicy(new ManualAck())
                .consumerGroup(consumerGroup)
                .build();
    }
}
```

### 4.13 Putting it together

```java
public class PubSubDemo {
    public static void main(String[] args) {
        MessageBroker broker = MessageBroker.getInstance();
        Topic orderTopic = broker.createTopic("orders");

        // Subscriber A and B in the SAME consumer group -> queue semantics (only one gets each message)
        Subscription subA = SubscriptionFactory.createPushSubscriber(
                msg -> System.out.println("ConsumerA processed: " + msg.getPayload()),
                "order-processors", new ExponentialBackoffRetry(3, 500), null);

        Subscription subB = SubscriptionFactory.createPushSubscriber(
                msg -> System.out.println("ConsumerB processed: " + msg.getPayload()),
                "order-processors", new ExponentialBackoffRetry(3, 500), null);

        // Subscriber C in a DIFFERENT group -> broadcast semantics (gets every message independently)
        MessageInterceptor auditChain = new FilterInterceptor(m -> true);
        Subscription subC = SubscriptionFactory.createPushSubscriber(
                msg -> System.out.println("AuditLogger recorded: " + msg.getPayload()),
                "audit-group", new FixedDelayRetry(2, 1000), auditChain);

        orderTopic.subscribe(subA);
        orderTopic.subscribe(subB);
        orderTopic.subscribe(subC);

        Message msg = new Message.Builder().topic("orders").payload("Order#123 placed").build();
        broker.publish("orders", msg);
        // -> exactly one of subA/subB gets it (round-robin), AND subC also gets it independently
    }
}
```

---

## 5. Why this shape holds up under follow-ups

- **"Add ordering guarantee per partition key"** → extend `Topic` to maintain per-`partitionKey` sub-logs and route `selectSubscriberFromGroup` deterministically by key hash instead of round-robin — the interceptor/delivery pipeline is untouched.
- **"Add webhook delivery for external subscribers"** → new `WebhookDelivery implements DeliveryStrategy`; nothing else changes.
- **"Add message TTL / expiry"** → new `MessageInterceptor` (`TtlInterceptor`) dropping expired messages before delivery — chain-of-responsibility already has the slot.
- **"Add schema validation before publish"** → another interceptor link at the front of the chain.
- **"Scale to multiple broker nodes (Kafka-like partitioned topics)"** → `MessageBroker` singleton's role becomes "coordinator," and `Topic`'s message log gets sharded across partitions/nodes — the public contract (`publish`, `subscribe`) stays the same, so `Subscription`/`DeliveryStrategy`/`RetryPolicy` code is unaffected.
- **"Guarantee exactly-once delivery"** → `DeduplicationInterceptor` already exists as the seam; extend with idempotency keys tracked in persistent storage instead of in-memory `Set`.

---

Want me to extend this with **partition-based ordering + partition assignment for consumer groups, a persistent message log (WAL) design for durability, exactly-once delivery semantics, or a comparison table mapping this design onto real systems (Kafka/RabbitMQ/SQS)**, or move to a different LLD problem?