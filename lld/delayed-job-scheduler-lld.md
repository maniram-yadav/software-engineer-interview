# Delayed Job Scheduler — LLD

## 1. Requirements

**Functional**
- Schedule a job to run once at a specific time, or after a delay (`runAt`).
- Schedule recurring jobs (cron-like: "every 5 min", "daily at 2 AM").
- Jobs have priority — higher priority jobs picked first when multiple are due.
- Worker pool picks up due jobs and executes them concurrently.
- Retry failed jobs with configurable backoff (fixed, exponential); move to dead-letter after max retries.
- Job lifecycle tracking: Scheduled → Queued → Running → Completed / Failed → Retrying → DeadLettered / Cancelled.
- Support job cancellation and pausing before execution.
- Notify on job completion/failure (for monitoring/alerting).
- Prevent the same job from being picked up by two workers simultaneously (single-execution guarantee).

**Non-functional**
- Efficient "what's due right now" lookup — not a linear scan over all jobs.
- New job types (email job, report job, cleanup job) pluggable without touching the scheduler core.
- New retry/backoff policies pluggable independently of job type.
- Execution logic separated from scheduling/queuing logic (single responsibility).
- Must degrade gracefully under worker crash — a job picked up but not completed shouldn't vanish silently.

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **Command** | `Job` implements `execute()` — every schedulable unit of work is a command object | Scheduler needs to store, queue, retry, and re-execute "a unit of work" without knowing its internals. Command decouples *what to run* from *when/how it's run*. |
| **State** | `JobState`: `ScheduledState`, `QueuedState`, `RunningState`, `CompletedState`, `FailedState`, `RetryingState`, `CancelledState`, `DeadLetteredState` | Legal actions depend entirely on current state (can't cancel a running job the same way as a scheduled one; can't retry a completed job). Prevents illegal transitions. |
| **Strategy** | `TriggerStrategy` (`OneTimeTrigger`, `RecurringTrigger`/cron); `RetryPolicy` (`FixedDelayRetry`, `ExponentialBackoffRetry`, `NoRetry`) | *When a job next fires* and *how failures are retried* both vary independently of the job's actual work and of each other — isolating them avoids a monolithic scheduler with type-checks for every combination. |
| **Priority Queue (Delay Queue)** | `JobQueue` backed by a `DelayQueue`/min-heap keyed by `(nextRunTime, priority)` | Core data-structure decision, not a GoF pattern, but critical: gives O(log n) "what's due next" instead of scanning all jobs — this is the actual engine of the scheduler. |
| **Observer** | `Job` (Subject) notifies `JobObserver`: `JobMetricsCollector`, `AlertingNotifier`, `JobAuditLogger` | One execution outcome → multiple independent reactions (metrics, alerts, audit trail) without `JobExecutor` knowing about any of them. |
| **Singleton** | `JobScheduler` | Single source of truth for all scheduled jobs and the single owner of the delay queue + worker pool — there must be exactly one scheduling authority per process. |
| **Template Method** | `AbstractJob.run()` defines skeleton: `beforeExecute()` → `execute()` → `afterExecute()`/`onFailure()` | Every job needs the same execution envelope (state transitions, timing, exception capture) regardless of what the job actually does — subclasses only override the actual work. |
| **Factory Method** | `TriggerFactory.create(scheduleSpec)` | Encapsulates parsing a schedule spec ("once at T", "cron expr") into the right `TriggerStrategy`. |
| **Builder** | `Job.Builder` | Many optional fields (priority, retry policy, trigger, metadata). |

**SOLID**
- **S**: `JobQueue` only manages ordering; `JobExecutor` only runs jobs; `RetryPolicy` only computes next retry time; `TriggerStrategy` only computes next fire time.
- **O**: New job type → subclass `AbstractJob`, override `execute()`. New retry policy / trigger type → new strategy implementation. New post-execution reaction → new observer. Nothing existing changes.
- **L**: Any `JobState` substitutable wherever `Job` delegates; any `RetryPolicy`/`TriggerStrategy` substitutable at their call sites.
- **I**: `JobObserver` exposes only `onJobEvent`; `RetryPolicy` exposes only `nextRetryDelay`/`shouldRetry` — narrow, focused interfaces.
- **D**: `JobScheduler`/`JobExecutor` depend on `TriggerStrategy`, `RetryPolicy`, `JobState` abstractions, never concrete implementations.

---

## 3. Class Diagram (textual)

```
┌──────────────────┐        ┌─────────────────────────┐
│   JobState            │◀──────│  Job (Command, Context, Subject)│
│ (State interface)      │       │ - state: JobState                │
│ + queue()/start()/       │     │ - triggerStrategy                  │
│   complete()/fail()/      │    │ - retryPolicy                       │
│   cancel()/deadLetter()    │   │ - observers: List<Obs>               │
└────────▲──────────────┘      │ + execute() [abstract, Template Method]│
  ┌──────┼───────┬─────────┬───┴─────────────┬──────────┬────────────┐
Scheduled Queued Running  Completed        Failed     Retrying   Cancelled  DeadLettered
 State    State   State     State            State       State      State       State

┌──────────────────────┐      ┌──────────────────────┐
│  TriggerStrategy         │    │  RetryPolicy              │
│ (Strategy interface)       │  │ (Strategy interface)         │
│ + nextFireTime(from)        │ │ + shouldRetry(attempt)         │
└──────────▲───────────┘      │ + nextRetryDelay(attempt)        │
   ┌───────┼────────┐         └──────────▲───────────┘
OneTimeTrigger RecurringTrigger  ┌────────┼────────┐
   (cron-based)               FixedDelayRetry ExponentialBackoffRetry NoRetry

┌──────────────────┐        ┌───────────────────────┐
│  JobObserver          │      │  JobScheduler              │
│ + onJobEvent(job,evt)   │     │  (Singleton)                  │
└──────────▲───────────┘      │  + schedule(job)                │
    ┌──────┼───────┬────────┐ │  + cancel(jobId)                 │
JobMetricsCollector AlertingNotifier JobAuditLogger  │  - jobQueue: JobQueue    │
                                                       │  - workerPool: ExecutorService│
                                                       └───────────────────────┘

┌──────────────────┐        ┌───────────────────────┐
│  JobQueue              │     │  JobExecutor (Worker)     │
│  (DelayQueue/min-heap)   │    │  + run(job)                  │
│  + offer(job)              │  └───────────────────────┘
│  + take(): Job (blocks     │
│      until due)             │
└──────────────────┘

┌──────────────────┐   extends   ┌──────────────────┐
│  AbstractJob            │────────────▶│  EmailJob / ReportJob / CleanupJob│
│  (Template Method)        │            │  (concrete work in execute())      │
└──────────────────┘                └──────────────────┘

┌──────────────────┐
│  JobFactory / TriggerFactory│
└──────────────────┘
```

---

## 4. Code (Java)

### 4.1 Trigger Strategy — when does a job next fire

```java
public interface TriggerStrategy {
    /** @return next fire time in epoch millis, or -1 if no more firings (one-time job already fired) */
    long nextFireTime(long fromEpochMillis);
    boolean isRecurring();
}

public class OneTimeTrigger implements TriggerStrategy {
    private final long runAt;
    private boolean fired = false;

    public OneTimeTrigger(long runAt) { this.runAt = runAt; }

    @Override
    public long nextFireTime(long fromEpochMillis) {
        if (fired) return -1;
        fired = true;
        return runAt;
    }
    @Override
    public boolean isRecurring() { return false; }
}

public class RecurringTrigger implements TriggerStrategy {
    private final long intervalMillis; // simplified — real impl would parse cron expressions

    public RecurringTrigger(long intervalMillis) { this.intervalMillis = intervalMillis; }

    @Override
    public long nextFireTime(long fromEpochMillis) {
        return fromEpochMillis + intervalMillis;
    }
    @Override
    public boolean isRecurring() { return true; }
}
```

### 4.2 Retry Policy — how failures are retried

```java
public interface RetryPolicy {
    boolean shouldRetry(int attemptNumber);
    long nextRetryDelay(int attemptNumber); // millis
}

public class NoRetry implements RetryPolicy {
    public boolean shouldRetry(int attemptNumber) { return false; }
    public long nextRetryDelay(int attemptNumber) { return 0; }
}

public class FixedDelayRetry implements RetryPolicy {
    private final int maxAttempts;
    private final long delayMillis;

    public FixedDelayRetry(int maxAttempts, long delayMillis) {
        this.maxAttempts = maxAttempts; this.delayMillis = delayMillis;
    }
    public boolean shouldRetry(int attemptNumber) { return attemptNumber < maxAttempts; }
    public long nextRetryDelay(int attemptNumber) { return delayMillis; }
}

public class ExponentialBackoffRetry implements RetryPolicy {
    private final int maxAttempts;
    private final long baseDelayMillis;

    public ExponentialBackoffRetry(int maxAttempts, long baseDelayMillis) {
        this.maxAttempts = maxAttempts; this.baseDelayMillis = baseDelayMillis;
    }
    public boolean shouldRetry(int attemptNumber) { return attemptNumber < maxAttempts; }
    public long nextRetryDelay(int attemptNumber) {
        return baseDelayMillis * (long) Math.pow(2, attemptNumber - 1); // 1x, 2x, 4x, 8x...
    }
}
```

### 4.3 State pattern — Job lifecycle

```java
public interface JobState {
    void queue(Job job);
    void start(Job job);
    void complete(Job job);
    void fail(Job job, Exception e);
    void cancel(Job job);
    String name();
}

public class ScheduledState implements JobState {
    public void queue(Job job) { job.setState(new QueuedState()); }
    public void start(Job job) { throw new IllegalStateException("Job must be queued first"); }
    public void complete(Job job) { throw new IllegalStateException("Job hasn't started"); }
    public void fail(Job job, Exception e) { throw new IllegalStateException("Job hasn't started"); }
    public void cancel(Job job) { job.setState(new CancelledState()); job.notifyObservers("CANCELLED"); }
    public String name() { return "SCHEDULED"; }
}

public class QueuedState implements JobState {
    public void queue(Job job) { throw new IllegalStateException("Already queued"); }
    public void start(Job job) { job.setState(new RunningState()); job.notifyObservers("STARTED"); }
    public void complete(Job job) { throw new IllegalStateException("Job hasn't started"); }
    public void fail(Job job, Exception e) { throw new IllegalStateException("Job hasn't started"); }
    public void cancel(Job job) { job.setState(new CancelledState()); job.notifyObservers("CANCELLED"); }
    public String name() { return "QUEUED"; }
}

public class RunningState implements JobState {
    public void queue(Job job) { throw new IllegalStateException("Job is running"); }
    public void start(Job job) { throw new IllegalStateException("Already running"); }
    public void complete(Job job) {
        job.setState(new CompletedState());
        job.notifyObservers("COMPLETED");
        if (job.getTriggerStrategy().isRecurring()) job.rescheduleNextRun();
    }
    public void fail(Job job, Exception e) {
        int attempt = job.incrementAttempt();
        if (job.getRetryPolicy().shouldRetry(attempt)) {
            job.setState(new RetryingState());
            job.notifyObservers("RETRYING");
            long delay = job.getRetryPolicy().nextRetryDelay(attempt);
            job.rescheduleAt(System.currentTimeMillis() + delay);
        } else {
            job.setState(new DeadLetteredState());
            job.notifyObservers("DEAD_LETTERED");
        }
    }
    public void cancel(Job job) { throw new IllegalStateException("Cannot cancel a running job"); }
    public String name() { return "RUNNING"; }
}

public class CompletedState implements JobState {
    public void queue(Job job) { throw new IllegalStateException("Job completed"); }
    public void start(Job job) { throw new IllegalStateException("Job completed"); }
    public void complete(Job job) { throw new IllegalStateException("Already completed"); }
    public void fail(Job job, Exception e) { throw new IllegalStateException("Job completed"); }
    public void cancel(Job job) { throw new IllegalStateException("Job completed"); }
    public String name() { return "COMPLETED"; }
}

public class RetryingState implements JobState {
    public void queue(Job job) { job.setState(new QueuedState()); }
    public void start(Job job) { throw new IllegalStateException("Not queued yet"); }
    public void complete(Job job) { throw new IllegalStateException("Not running"); }
    public void fail(Job job, Exception e) { throw new IllegalStateException("Not running"); }
    public void cancel(Job job) { job.setState(new CancelledState()); job.notifyObservers("CANCELLED"); }
    public String name() { return "RETRYING"; }
}

public class CancelledState implements JobState {
    public void queue(Job job) { throw new IllegalStateException("Job cancelled"); }
    public void start(Job job) { throw new IllegalStateException("Job cancelled"); }
    public void complete(Job job) { throw new IllegalStateException("Job cancelled"); }
    public void fail(Job job, Exception e) { throw new IllegalStateException("Job cancelled"); }
    public void cancel(Job job) { throw new IllegalStateException("Already cancelled"); }
    public String name() { return "CANCELLED"; }
}

public class DeadLetteredState implements JobState {
    public void queue(Job job) { throw new IllegalStateException("Job dead-lettered"); }
    public void start(Job job) { throw new IllegalStateException("Job dead-lettered"); }
    public void complete(Job job) { throw new IllegalStateException("Job dead-lettered"); }
    public void fail(Job job, Exception e) { throw new IllegalStateException("Already dead-lettered"); }
    public void cancel(Job job) { throw new IllegalStateException("Job dead-lettered"); }
    public String name() { return "DEAD_LETTERED"; }
}
```

### 4.4 Observer — job event reactions

```java
public interface JobObserver {
    void onJobEvent(Job job, String eventType);
}

public class JobMetricsCollector implements JobObserver {
    public void onJobEvent(Job job, String eventType) {
        // increment counters: jobs_completed, jobs_failed, jobs_retried, etc.
    }
}

public class AlertingNotifier implements JobObserver {
    public void onJobEvent(Job job, String eventType) {
        if (eventType.equals("DEAD_LETTERED")) {
            System.out.println("[ALERT] Job " + job.getId() + " exhausted retries and was dead-lettered");
        }
    }
}

public class JobAuditLogger implements JobObserver {
    public void onJobEvent(Job job, String eventType) {
        System.out.println("[Audit] " + job.getId() + " -> " + eventType + " @ " + Instant.now());
    }
}
```

### 4.5 Command + Template Method — Job as an executable command

`AbstractJob` fixes the *envelope* (state transitions, exception capture, timing) via Template Method; concrete jobs only implement `doExecute()`.

```java
public abstract class AbstractJob implements Comparable<AbstractJob> {
    private final String id;
    private final int priority; // higher = more urgent
    private final TriggerStrategy triggerStrategy;
    private final RetryPolicy retryPolicy;
    private final List<JobObserver> observers = new ArrayList<>();

    private JobState state = new ScheduledState();
    private int attemptCount = 0;
    private long nextRunTime;

    protected AbstractJob(String id, int priority, TriggerStrategy trigger, RetryPolicy retryPolicy) {
        this.id = id; this.priority = priority;
        this.triggerStrategy = trigger; this.retryPolicy = retryPolicy;
        this.nextRunTime = trigger.nextFireTime(System.currentTimeMillis());
    }

    // ---- Template Method: fixed execution skeleton ----
    public final void run() {
        try {
            beforeExecute();
            doExecute();           // <-- subclass-specific work
            afterExecute();
            complete();
        } catch (Exception e) {
            onFailure(e);
            fail(e);
        }
    }

    protected void beforeExecute() { /* hook, default no-op */ }
    protected abstract void doExecute() throws Exception; // the actual job logic
    protected void afterExecute() { /* hook, default no-op */ }
    protected void onFailure(Exception e) { /* hook, default logs nothing extra */ }

    // ---- State delegation ----
    public void markQueued() { state.queue(this); }
    public void markStarted() { state.start(this); }
    public void complete() { state.complete(this); }
    public void fail(Exception e) { state.fail(this, e); }
    public void cancel() { state.cancel(this); }
    void setState(JobState s) { this.state = s; }
    public String getStateName() { return state.name(); }

    public void subscribe(JobObserver o) { observers.add(o); }
    void notifyObservers(String eventType) {
        for (JobObserver o : observers) o.onJobEvent(this, eventType);
    }

    int incrementAttempt() { return ++attemptCount; }
    void rescheduleAt(long time) { this.nextRunTime = time; markQueuedInternal(); }
    void rescheduleNextRun() {
        long next = triggerStrategy.nextFireTime(System.currentTimeMillis());
        if (next != -1) { this.nextRunTime = next; setState(new ScheduledState()); JobScheduler.getInstance().schedule(this); }
    }
    private void markQueuedInternal() { setState(new ScheduledState()); JobScheduler.getInstance().schedule(this); }

    public TriggerStrategy getTriggerStrategy() { return triggerStrategy; }
    public RetryPolicy getRetryPolicy() { return retryPolicy; }
    public String getId() { return id; }
    public int getPriority() { return priority; }
    public long getNextRunTime() { return nextRunTime; }

    @Override
    public int compareTo(AbstractJob other) {
        int timeCompare = Long.compare(this.nextRunTime, other.nextRunTime);
        if (timeCompare != 0) return timeCompare;
        return Integer.compare(other.priority, this.priority); // higher priority first for same time
    }
}
```

### 4.6 Concrete job types

```java
public class EmailJob extends AbstractJob {
    private final String recipient, subject;

    public EmailJob(String id, int priority, TriggerStrategy trigger, RetryPolicy retryPolicy,
                     String recipient, String subject) {
        super(id, priority, trigger, retryPolicy);
        this.recipient = recipient; this.subject = subject;
    }

    @Override
    protected void doExecute() throws Exception {
        System.out.println("Sending email to " + recipient + ": " + subject);
        // actual email-sending logic; may throw on failure
    }
}

public class CleanupJob extends AbstractJob {
    public CleanupJob(String id, int priority, TriggerStrategy trigger, RetryPolicy retryPolicy) {
        super(id, priority, trigger, retryPolicy);
    }
    @Override
    protected void doExecute() throws Exception {
        System.out.println("Running cleanup of stale records...");
    }
}
```

### 4.7 JobQueue — delay + priority ordering

```java
public class JobQueue {
    // DelayQueue requires elements to implement Delayed; wrap AbstractJob
    private final DelayQueue<DelayedJobWrapper> queue = new DelayQueue<>();

    public void offer(AbstractJob job) {
        job.markQueued();
        queue.offer(new DelayedJobWrapper(job));
    }

    /** Blocks until a job is due, then returns it (highest priority among due jobs). */
    public AbstractJob take() throws InterruptedException {
        return queue.take().getJob();
    }

    private static class DelayedJobWrapper implements Delayed {
        private final AbstractJob job;
        DelayedJobWrapper(AbstractJob job) { this.job = job; }

        public AbstractJob getJob() { return job; }

        @Override
        public long getDelay(TimeUnit unit) {
            long diff = job.getNextRunTime() - System.currentTimeMillis();
            return unit.convert(diff, TimeUnit.MILLISECONDS);
        }
        @Override
        public int compareTo(Delayed o) {
            if (o instanceof DelayedJobWrapper) {
                return this.job.compareTo(((DelayedJobWrapper) o).job);
            }
            return Long.compare(getDelay(TimeUnit.MILLISECONDS), o.getDelay(TimeUnit.MILLISECONDS));
        }
    }
}
```

### 4.8 Singleton — JobScheduler (single scheduling authority + worker pool)

```java
public class JobScheduler {
    private static volatile JobScheduler instance;

    private final JobQueue jobQueue = new JobQueue();
    private final ExecutorService workerPool = Executors.newFixedThreadPool(8);
    private final ConcurrentHashMap<String, AbstractJob> allJobs = new ConcurrentHashMap<>();
    private volatile boolean running = false;

    private JobScheduler() {}

    public static JobScheduler getInstance() {
        if (instance == null) {
            synchronized (JobScheduler.class) {
                if (instance == null) instance = new JobScheduler();
            }
        }
        return instance;
    }

    public void schedule(AbstractJob job) {
        allJobs.put(job.getId(), job);
        jobQueue.offer(job);
    }

    public void cancel(String jobId) {
        AbstractJob job = allJobs.get(jobId);
        if (job != null) job.cancel();
        // DelayQueue doesn't support O(1) removal; worker checks state before executing (see below)
    }

    public void start() {
        running = true;
        for (int i = 0; i < 8; i++) {
            workerPool.submit(this::workerLoop);
        }
    }

    private void workerLoop() {
        while (running) {
            try {
                AbstractJob job = jobQueue.take();
                if (job.getStateName().equals("CANCELLED")) continue; // skip cancelled jobs picked up from queue
                job.markStarted();
                job.run(); // Template Method handles success/failure + state transitions
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }

    public void shutdown() {
        running = false;
        workerPool.shutdownNow();
    }
}
```

> Note on cancellation: `DelayQueue` doesn't support efficient arbitrary removal, so cancellation is handled lazily — mark the job `CANCELLED` via state, and the worker skips it when dequeued. This keeps `cancel()` O(1) instead of forcing an O(n) queue scan.

### 4.9 Factory Method — TriggerFactory

```java
public class TriggerFactory {
    public static TriggerStrategy oneTimeAt(long epochMillis) {
        return new OneTimeTrigger(epochMillis);
    }
    public static TriggerStrategy recurringEvery(long intervalMillis) {
        return new RecurringTrigger(intervalMillis);
    }
    // real system: public static TriggerStrategy fromCron(String cronExpr) { ... }
}
```

### 4.10 Putting it together

```java
public class DelayedJobSchedulerDemo {
    public static void main(String[] args) throws InterruptedException {
        JobScheduler scheduler = JobScheduler.getInstance();
        scheduler.start();

        // one-time job, runs in 5 seconds, retries 3 times with exponential backoff
        EmailJob welcomeEmail = new EmailJob(
                "job-1", 5,
                TriggerFactory.oneTimeAt(System.currentTimeMillis() + 5000),
                new ExponentialBackoffRetry(3, 1000),
                "user@example.com", "Welcome!");
        welcomeEmail.subscribe(new JobMetricsCollector());
        welcomeEmail.subscribe(new AlertingNotifier());
        welcomeEmail.subscribe(new JobAuditLogger());

        // recurring job, every 10 seconds, no retry
        CleanupJob cleanup = new CleanupJob(
                "job-2", 1,
                TriggerFactory.recurringEvery(10000),
                new NoRetry());
        cleanup.subscribe(new JobAuditLogger());

        scheduler.schedule(welcomeEmail);
        scheduler.schedule(cleanup);

        Thread.sleep(30000);
        scheduler.shutdown();
    }
}
```

---

## 5. Concurrency & distributed considerations (worth raising in an interview)

- **Single-execution guarantee across multiple scheduler instances**: the in-JVM `DelayQueue` only works for a single process. In a distributed deployment, replace `JobQueue`'s backing store with a DB/Redis-backed queue and acquire a **distributed lock per job** (e.g., `SELECT ... FOR UPDATE` on the job row, or Redis `SETNX` with TTL) before `markStarted()` — same seam as `SeatLockManager` in the BookMyShow design: callers depend on an abstraction (`JobQueue.take()`), only the implementation changes.
- **Crash recovery**: if a worker crashes mid-execution, a job stuck in `RUNNING` needs a **visibility timeout** (like SQS) — a background sweeper requeues jobs whose `RUNNING` state has exceeded an expected duration. This is a natural extension of `RunningState.fail()`.
- **Idempotency**: since retries and crash-recovery can cause a job to run more than once, `doExecute()` implementations should be idempotent (or the scheduler should track an idempotency key) — worth calling out as an assumption/limitation of this design.

---

## 6. Why this shape holds up under follow-ups

- **"Add cron-expression scheduling"** → extend `RecurringTrigger` or add `CronTrigger` implementing `TriggerStrategy`; nothing else changes.
- **"Add job priority preemption (urgent job runs before a queued lower-priority one)"** → already handled by `compareTo` in `AbstractJob` combined with the `DelayQueue`'s ordering.
- **"Add a dashboard showing job history"** → new `JobObserver` implementation writing to a persistence store; zero changes to scheduler core.
- **"Support job dependencies (job B runs only after job A completes)"** → extend `CompletedState.complete()` to check a dependency graph and schedule dependent jobs — Observer hook already fires exactly at that point.
- **"Rate-limit how many jobs of a certain type run concurrently"** → wrap `workerLoop`/`JobExecutor` with the **Rate Limiter** design from earlier — these two systems compose cleanly since both were built around clean abstraction boundaries.

---

Want me to extend this with **a distributed lock implementation (Redis-based) for multi-instance schedulers, cron-expression parsing, a dead-letter-queue replay mechanism, or persistence (DB schema) for job durability across restarts**, or move to a different LLD problem?