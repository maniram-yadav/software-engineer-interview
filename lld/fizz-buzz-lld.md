# FizzBuzz Multithreaded — LLD

## 1. Requirements

**Functional**
- 4 threads run concurrently, each responsible for one output type:
  - Thread A prints `"fizz"` when the number is divisible by 3 (but not 5)
  - Thread B prints `"buzz"` when divisible by 5 (but not 3)
  - Thread C prints `"fizzbuzz"` when divisible by both 3 and 5
  - Thread D prints the number itself otherwise
- Output must be in **strict sequential order 1, 2, 3, ..., n** despite 4 threads running concurrently — no interleaving/reordering.
- Only one thread should be "active" (able to print) at any given moment; the other three must be blocked, not busy-spinning.
- Extensible to more rules (e.g., "fizzbuzzbazz" for divisible by 7 too) without rewriting the coordination mechanism.

**Non-functional**
- No busy-waiting (a naive `while (condition) {}` spin loop burns CPU and is considered incorrect in interviews).
- No global lock serializing *decision-making* in a way that couples all four threads' logic together — the "whose turn is it" rule should be independent of the coordination mechanism.

---

## 2. The core concurrency insight

This is fundamentally a **turn-taking / rendezvous problem**, not a throughput problem — exactly the opposite of the Bloom filter design. Four threads must execute in a strict relay, each waking exactly the right next thread. The standard, correct primitive for "block until someone else says it's your turn" is a **counting semaphore with 0 initial permits** (a *signal*, not a *lock*) — one per thread role, with only the number-thread's semaphore starting at 1 (since it goes first, absent a multiple of 3 or 5).

This is different from a mutex: a mutex protects a critical section any thread can enter; here we want **precisely one specific thread** to wake up next, determined by the current number — which is exactly what per-role semaphores give you.

---

## 3. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **Monitor / semaphore-based rendezvous (concurrency primitive, not GoF)** | Four `Semaphore` objects, one per thread role, passed the baton in sequence | Solves the actual problem — strict ordering with zero busy-waiting. Each thread blocks on `acquire()` until the previous thread explicitly `release()`s its semaphore. |
| **Strategy** | `PrintCondition` interface: `FizzCondition`, `BuzzCondition`, `FizzBuzzCondition`, `NumberCondition` | *Which* condition a number satisfies is independent of *how threads coordinate*. Isolating this means adding a 5th rule (e.g., "Bazz" for divisible by 7) doesn't touch the semaphore/coordination logic at all. |
| **Template Method** | `FizzBuzzWorker.run()` fixes the loop skeleton: acquire → check bounds → print if condition matches → release next semaphore(s) | All four worker threads share an identical execution shape; only their `PrintCondition` and which semaphore(s) they signal next differ. |
| **Command** | `Runnable print` callback passed into each worker (as in the actual LeetCode 1195 API) | Decouples "what to print" from "when to print it" — lets the same coordinator be reused with a different output sink (e.g., append to a list instead of `System.out`, useful for testing). |
| **Builder** | `FizzBuzzCoordinator.Builder` for registering an arbitrary list of `(PrintCondition, output)` rules in priority order | Makes the rule set genuinely pluggable/orderable instead of hardcoded to exactly 4 threads. |

**SOLID**
- **S**: Each `PrintCondition` only decides "does this number match me"; `FizzBuzzWorker` only handles the wait/print/signal loop; `FizzBuzzCoordinator` only owns shared state (`n`, current number) and semaphore wiring.
- **O**: New rule (e.g., divisible-by-7 → "Bazz") → new `PrintCondition` + one more worker registered in the coordinator. No existing worker or condition class is touched.
- **L**: Any `PrintCondition` is substitutable wherever the interface is used — `FizzBuzzWorker` never knows or cares which concrete condition it holds.
- **I**: `PrintCondition` exposes a single `matches(int)` method — no bloated interface.
- **D**: `FizzBuzzWorker` depends on the `PrintCondition` abstraction and a `Runnable` output callback, injected at construction — never on concrete condition logic or `System.out` directly.

---

## 4. Class Diagram (textual)

```
┌────────────────────┐
│  PrintCondition          │  (Strategy interface)
│  + matches(int n): bool     │
└──────────▲───────────┘
   ┌───────┼────────┬─────────────┐
FizzCondition BuzzCondition FizzBuzzCondition NumberCondition

┌────────────────────┐        ┌──────────────────────────┐
│  FizzBuzzWorker         │◀───────│  FizzBuzzCoordinator          │
│ (Template Method)         │       │  - n: int                       │
│ - condition: PrintCondition  │    │  - current: AtomicInteger          │
│ - waitOn: Semaphore            │  │  - semaphores: Map<Role, Semaphore>  │
│ - signalNext: List<Semaphore>    │ │  + start()                             │
│ - output: Runnable                 │└──────────────────────────┘
│ + run() [Template Method]
└────────────────────┘
```

---

## 5. Code (Java)

### 5.1 Strategy — PrintCondition

```java
public interface PrintCondition {
    boolean matches(int number);
}

public class FizzCondition implements PrintCondition {
    public boolean matches(int number) { return number % 3 == 0 && number % 5 != 0; }
}

public class BuzzCondition implements PrintCondition {
    public boolean matches(int number) { return number % 5 == 0 && number % 3 != 0; }
}

public class FizzBuzzCondition implements PrintCondition {
    public boolean matches(int number) { return number % 3 == 0 && number % 5 == 0; }
}

public class NumberCondition implements PrintCondition {
    public boolean matches(int number) { return number % 3 != 0 && number % 5 != 0; }
}
```

### 5.2 Template Method — FizzBuzzWorker

Each worker: block on its own semaphore → check if the game is over → print if its condition matches (it will, always, since the coordinator only wakes the correct worker for each number... except see note below on the general N-rule case) → hand off to the next worker in the ring.

```java
import java.util.List;
import java.util.concurrent.Semaphore;
import java.util.concurrent.atomic.AtomicInteger;

public class FizzBuzzWorker implements Runnable {
    private final PrintCondition condition;
    private final Semaphore waitOn;
    private final List<Semaphore> signalNext; // could be multiple, for extensibility
    private final Runnable onPrint;           // Command: what to actually do with the output
    private final AtomicInteger current;
    private final int n;

    public FizzBuzzWorker(PrintCondition condition, Semaphore waitOn, List<Semaphore> signalNext,
                           Runnable onPrint, AtomicInteger current, int n) {
        this.condition = condition;
        this.waitOn = waitOn;
        this.signalNext = signalNext;
        this.onPrint = onPrint;
        this.current = current;
        this.n = n;
    }

    @Override
    public void run() {
        while (true) {
            try {
                waitOn.acquire(); // block until it's genuinely this worker's turn — no busy-waiting
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                return;
            }

            int number = current.get();
            if (number > n) {
                // propagate shutdown signal so downstream workers also exit instead of blocking forever
                for (Semaphore s : signalNext) s.release();
                return;
            }

            if (condition.matches(number)) {
                onPrint.run();
                current.incrementAndGet();
            }
            // if condition doesn't match (only relevant with >4 chained rules), fall through without printing —
            // the coordinator still advances via the next worker in the chain

            for (Semaphore s : signalNext) s.release(); // hand off the baton
        }
    }
}
```

### 5.3 FizzBuzzCoordinator — wires the semaphore ring

The four semaphores form a **ring**: each worker signals the next one in a fixed order, and the ring always advances by exactly one full rotation per number. Only the very first worker in the ring starts with a permit (1), everyone else starts blocked (0).

```java
import java.util.List;
import java.util.concurrent.Semaphore;
import java.util.concurrent.atomic.AtomicInteger;

public class FizzBuzzCoordinator {
    private final int n;
    private final AtomicInteger current = new AtomicInteger(1);

    // ring order matters: fizz -> buzz -> fizzbuzz -> number -> (back to fizz)
    private final Semaphore fizzSem = new Semaphore(0);
    private final Semaphore buzzSem = new Semaphore(0);
    private final Semaphore fizzBuzzSem = new Semaphore(0);
    private final Semaphore numberSem = new Semaphore(1); // starts the ring

    public FizzBuzzCoordinator(int n) {
        this.n = n;
    }

    public void start() throws InterruptedException {
        Thread fizzThread = new Thread(new FizzBuzzWorker(
                new FizzCondition(), numberSem, List.of(fizzSem),
                () -> System.out.println("fizz"), current, n), "fizz-thread");
        // NOTE: ordering fix below — see corrected wiring

        // Correct ring wiring: each worker WAITS on its own semaphore and SIGNALS the next role's semaphore.
        Thread t1 = new Thread(new FizzBuzzWorker(
                new NumberCondition(), numberSem, List.of(fizzSem),
                () -> System.out.print(current.get() + " "), current, n), "number-thread");

        Thread t2 = new Thread(new FizzBuzzWorker(
                new FizzCondition(), fizzSem, List.of(buzzSem),
                () -> System.out.print("fizz "), current, n), "fizz-thread");

        Thread t3 = new Thread(new FizzBuzzWorker(
                new BuzzCondition(), buzzSem, List.of(fizzBuzzSem),
                () -> System.out.print("buzz "), current, n), "buzz-thread");

        Thread t4 = new Thread(new FizzBuzzWorker(
                new FizzBuzzCondition(), fizzBuzzSem, List.of(numberSem),
                () -> System.out.print("fizzbuzz "), current, n), "fizzbuzz-thread");

        t1.start(); t2.start(); t3.start(); t4.start();
        t1.join(); t2.join(); t3.join(); t4.join();
    }
}
```

> **Important correction to internalize**: with this ring design, only *one* of the four conditions actually matches for any given number — the other three workers wake up, find their condition false, print nothing, and immediately pass the baton onward. This means every number still requires 4 semaphore handoffs (a full ring rotation) regardless of which rule applies. That's correct and matches the standard LeetCode 1195 solution shape, but it's worth explicitly noting in an interview: **the ring always does 4 acquire/release pairs per number**, not "jump directly to the right thread."

### 5.4 Demo

```java
public class FizzBuzzDemo {
    public static void main(String[] args) throws InterruptedException {
        FizzBuzzCoordinator coordinator = new FizzBuzzCoordinator(15);
        coordinator.start();
        // Output: 1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz
    }
}
```

---

## 6. Alternative: Lock + Condition variable (worth mentioning as an alternative)

Instead of 4 semaphores in a fixed ring, a single `Lock` + 4 `Condition`s (or one condition with a shared "current role" state) also works, and generalizes slightly better if the "whose turn" logic becomes data-driven rather than a fixed ring:

```java
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class LockBasedFizzBuzz {
    private final int n;
    private int current = 1;
    private final Lock lock = new ReentrantLock();
    private final Condition condition = lock.newCondition();

    public LockBasedFizzBuzz(int n) { this.n = n; }

    public void fizz(Runnable print) throws InterruptedException {
        runWhen(x -> x % 3 == 0 && x % 5 != 0, print);
    }
    public void buzz(Runnable print) throws InterruptedException {
        runWhen(x -> x % 5 == 0 && x % 3 != 0, print);
    }
    public void fizzbuzz(Runnable print) throws InterruptedException {
        runWhen(x -> x % 3 == 0 && x % 5 == 0, print);
    }
    public void number(java.util.function.IntConsumer print) throws InterruptedException {
        lock.lock();
        try {
            while (current <= n) {
                while (current <= n && (current % 3 == 0 || current % 5 == 0)) {
                    condition.await(); // not this thread's turn — wait
                }
                if (current > n) break;
                print.accept(current);
                current++;
                condition.signalAll(); // wake everyone to re-check whose turn it now is
            }
        } finally { lock.unlock(); }
    }

    private void runWhen(java.util.function.IntPredicate matcher, Runnable print) throws InterruptedException {
        lock.lock();
        try {
            while (current <= n) {
                while (current <= n && !matcher.test(current)) {
                    condition.await();
                }
                if (current > n) break;
                print.run();
                current++;
                condition.signalAll();
            }
        } finally { lock.unlock(); }
    }
}
```

**Trade-off vs the semaphore-ring version**: this uses `signalAll()` (wakes all waiters, they re-check and most go back to sleep — a "thundering herd" on every number), whereas the semaphore ring wakes exactly one thread each time. The semaphore ring is more efficient; the lock/condition version is more flexible if the turn-order isn't a fixed rotation.

---

## 7. Why this shape holds up under follow-ups

- **"Extend to FizzBuzzBazz (add divisible-by-7 rule)"** → add a `BazzCondition` + a 5th `FizzBuzzWorker` in the ring, and re-wire the `signalNext` chain to include it. `PrintCondition`/`FizzBuzzWorker` are untouched — this is exactly why Strategy + Template Method were chosen here over hardcoding four `if/else` branches in one method.
- **"What if there were 100 conditions instead of 4?"** → the fixed-ring approach starts to feel awkward (100 semaphores, mostly no-op handoffs); this is the natural point to switch to the **lock/condition variable** approach with a data-driven rule list, since it doesn't require walking every rule per number via explicit handoff — worth raising this crossover point proactively in an interview.
- **"Make it testable without printing to stdout"** → already handled: `onPrint`/`print` is injected as a `Runnable`/`Command`, so tests can pass a callback appending to a `List<String>` and assert on exact output order.
- **"Avoid deadlock on shutdown when n is reached"** → the `if (number > n) { signal next; return; }` guard in `FizzBuzzWorker.run()` exists specifically so that when one worker detects completion, it still releases its semaphore once more so the *next* worker in the ring also observes `number > n` and exits — without this, three of the four threads would block forever waiting on a semaphore that never gets released.

---

Want me to extend this with the **data-driven N-rule generalization (rule list + lock/condition version scaled to arbitrary rule counts), a lock-free version using `AtomicInteger` + spin-free busy-wait avoidance via `LockSupport.park`/`unpark`, or a comparison of semaphore-ring vs condition-variable throughput under contention**, or move to a different LLD problem?