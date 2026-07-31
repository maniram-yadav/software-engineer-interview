# Concurrent Bloom Filter — LLD

## 1. Requirements

**Functional**
- `add(element)` — insert an element into the filter.
- `mightContain(element)` — return `false` (definitely not present) or `true` (possibly present, with some false-positive probability).
- Configurable expected insertions + target false-positive probability → auto-compute optimal bit-array size (`m`) and hash count (`k`).
- Pluggable hash strategy (so hash function choice doesn't leak into core logic).
- Support **counting** variant (allows deletion) as an optional extension.
- Support **scalable** variant (grows automatically as more elements are added than originally sized for, without the FPP degrading unboundedly).

**Non-functional (the actual point of this problem)**
- **High-throughput concurrent `add()` and `mightContain()`** from many threads simultaneously, with correctness (no false negatives ever — a Bloom filter must never say "definitely not" for something that was actually added).
- Reads (`mightContain`) should **never block** on writes (`add`) — this is a read-heavy structure in most real use cases (e.g., cache/CDN "have I seen this key before" checks).
- Avoid a single global lock around the bit array — that would serialize every thread and defeat the purpose of a concurrent structure.
- Growth (in the scalable variant) is rare relative to add/query — its synchronization cost shouldn't matter, but it must not corrupt or block hot-path operations for long.

---

## 2. The core concurrency insight (this drives the whole design)

A standard Bloom filter's bit array is **monotonic** — bits only ever transition `0 → 1`, never back to `0` (until an explicit full clear). That single fact means:

- **Writes don't need mutual exclusion, only atomicity per word.** Setting a bit is "OR this bit into a 64-bit word" — implementable as a **lock-free CAS retry loop** on an `AtomicLongArray`, with no locking at all.
- **Reads need no synchronization beyond visibility.** Since `AtomicLongArray` gives volatile-style read semantics, `mightContain()` just reads the current word — it may occasionally miss a bit that's mid-flight from a concurrent `add()` (briefly racy), but it can **never observe a bit flip back to 0**, so false negatives are still structurally impossible; at worst a concurrent add-in-progress causes a temporary false negative for that *specific racing pair*, which standard Bloom filter semantics already tolerate as "checked before the add completed."

This is exactly how Guava's `BloomFilter` implements its `LOCK_FREE` strategy — no locks on the hot path at all.

---

## 3. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **Lock-free CAS (concurrency primitive, not GoF)** | `BitArray.set()` using `AtomicLongArray.compareAndSet` | The core throughput mechanism — replaces a global lock with per-word optimistic atomic updates, since bits are monotonic (see above). |
| **Strategy** | `HashStrategy`: `Murmur3HashStrategy`, `DoubleHashingStrategy` | How an element maps to `k` bit indices is an independent, swappable concern — different hash functions trade speed vs. distribution quality; isolating this keeps `BloomFilter` core untouched when hash choice changes. |
| **Builder** | `BloomFilter.Builder` | Sizing math (`m`, `k` from expected insertions + target FPP) has several derived/optional parameters — Builder keeps this computation out of the constructor and avoids telescoping params. |
| **Decorator** | `CountingBloomFilter` wraps the bit-array concept with counters instead of bits, adding `remove()` | Deletion support is an *additional* capability layered on top of the base add/query contract — Decorator lets it exist as an alternate/extended implementation without forcing every Bloom filter to pay the counter-array memory cost. |
| **Composite** | `ScalableBloomFilter` holds a list of `BloomFilter` "generations"; `mightContain` checks across all, `add` always goes to the latest | A scalable filter *is* a collection of filters treated as one — Composite is the natural shape: same interface (`add`/`mightContain`), internally fans out across children. |
| **Copy-on-Write + narrow lock** | `ScalableBloomFilter`'s generation list is a `CopyOnWriteArrayList`; growth uses a short-held lock only around the append | Reads (`mightContain` iterating generations) are lock-free and see a consistent snapshot; the rare "add a new generation" mutation is the only place synchronization is needed, and it's cheap since growth is infrequent. |
| **Template Method (light)** | `AbstractBloomFilter` fixes the skeleton: compute indices via `HashStrategy` → delegate bit ops to a storage backend | Both `BloomFilter` and `CountingBloomFilter` share the same "hash then touch k positions" shape; only what happens per-position (set-bit vs increment-counter) differs. |

**SOLID**
- **S**: `BitArray` only manages atomic bit storage; `HashStrategy` only computes indices; `ScalableBloomFilter` only manages generation lifecycle.
- **O**: New hash function → new `HashStrategy`. New storage backend (e.g., off-heap) → new class implementing the same bit-array contract. Nothing existing changes.
- **L**: Any `HashStrategy` substitutable wherever used; `CountingBloomFilter` and `BloomFilter` both satisfy the same `mightContain`/`add` contract.
- **I**: `HashStrategy` exposes only `getIndices`; no bloated interface forcing unrelated capabilities.
- **D**: `AbstractBloomFilter` depends on `HashStrategy` and a bit-storage abstraction injected at construction, never concrete hash/storage classes.

---

## 4. Class Diagram (textual)

```
┌──────────────────────┐
│  HashStrategy             │  (Strategy interface)
│  + getIndices(element, k, m): int[]│
└──────────▲───────────┘
   ┌───────┼────────────┐
Murmur3HashStrategy  DoubleHashingStrategy

┌──────────────────────┐
│  BitArray                  │  (lock-free bit storage)
│  - bits: AtomicLongArray      │
│  + set(index): boolean          │  (CAS loop; returns true if bit was newly set)
│  + get(index): boolean            │
│  + approxCardinality(): long        │
└──────────────────────┘

┌──────────────────────┐
│  AbstractBloomFilter       │  (Template Method)
│  # hashStrategy               │
│  # numHashFunctions (k)         │
│  # numBits (m)                    │
│  + add(element)                     │  [template: computes indices, delegates]
│  + mightContain(element): boolean     │
│  # doAdd(indices)  [abstract]           │
│  # doCheck(indices): boolean [abstract]  │
└──────────▲───────────┘
   ┌───────┼──────────────────┐
BloomFilter (uses BitArray)   CountingBloomFilter (Decorator, uses AtomicIntegerArray counters)
                                    + remove(element)

┌──────────────────────┐
│  BloomFilter.Builder       │
│  + expectedInsertions(n)      │
│  + falsePositiveProbability(p)  │
│  + hashStrategy(strategy)         │
│  + build(): BloomFilter              │
└──────────────────────┘

┌──────────────────────┐
│  ScalableBloomFilter        │  (Composite)
│  - generations: CopyOnWriteArrayList<BloomFilter>│
│  - growthLock: ReentrantLock    │
│  + add(element)                    │
│  + mightContain(element): boolean    │
│  - maybeGrow()                         │
└──────────────────────┘
```

---

## 5. Code (Java)

### 5.1 BitArray — lock-free bit storage (the concurrency core)

```java
import java.util.concurrent.atomic.AtomicLongArray;

public class BitArray {
    private final AtomicLongArray words;
    private final int bitSize;

    public BitArray(int bitSize) {
        this.bitSize = bitSize;
        this.words = new AtomicLongArray((bitSize + 63) / 64);
    }

    /**
     * Atomically sets the bit at `index`. Lock-free via CAS retry loop —
     * safe because bits only ever go 0 -> 1, so retry-on-conflict is always
     * correct (no ABA-type problem: the target state is monotonic).
     * @return true if this call was the one that actually flipped the bit (useful for cardinality estimation)
     */
    public boolean set(int index) {
        int wordIndex = index >>> 6;      // index / 64
        long mask = 1L << (index & 0x3F); // index % 64

        long oldWord, newWord;
        do {
            oldWord = words.get(wordIndex);
            if ((oldWord & mask) != 0) return false; // already set — no-op, avoid pointless CAS
            newWord = oldWord | mask;
        } while (!words.compareAndSet(wordIndex, oldWord, newWord));
        return true;
    }

    /** Lock-free read — no synchronization needed beyond AtomicLongArray's built-in visibility guarantees. */
    public boolean get(int index) {
        int wordIndex = index >>> 6;
        long mask = 1L << (index & 0x3F);
        return (words.get(wordIndex) & mask) != 0;
    }

    public int size() { return bitSize; }

    /** Approximate number of bits set — used to decide when a ScalableBloomFilter generation should grow. */
    public long approxBitsSet() {
        long count = 0;
        for (int i = 0; i < words.length(); i++) {
            count += Long.bitCount(words.get(i));
        }
        return count;
    }
}
```

### 5.2 HashStrategy — Strategy for index computation

Uses the standard **double-hashing** trick (Kirsch–Mitzenmacher): computing only 2 real hashes and deriving all `k` indices as `h1 + i*h2`, avoiding `k` expensive hash computations per element.

```java
public interface HashStrategy {
    /** @return array of k bit indices in [0, numBits) for this element */
    int[] getIndices(Object element, int numHashFunctions, int numBits);
}

public class Murmur3HashStrategy implements HashStrategy {
    @Override
    public int[] getIndices(Object element, int k, int numBits) {
        byte[] bytes = toBytes(element);
        long hash64 = murmur3Hash64(bytes);
        int h1 = (int) (hash64 >>> 32);
        int h2 = (int) hash64;

        int[] indices = new int[k];
        for (int i = 0; i < k; i++) {
            int combined = h1 + i * h2;
            if (combined < 0) combined = ~combined; // ensure non-negative
            indices[i] = combined % numBits;
        }
        return indices;
    }

    private byte[] toBytes(Object element) {
        return element.toString().getBytes(java.nio.charset.StandardCharsets.UTF_8);
    }

    // simplified MurmurHash3 (production: use Guava's Hashing.murmur3_128() or a vetted library)
    private long murmur3Hash64(byte[] data) {
        long h1 = 0x9368e53c2f6af274L, h2 = 0x586dcd208f7cd3fdL;
        long c1 = 0x87c37b91114253d5L, c2 = 0x4cf5ad432745937fL;
        int len = data.length;
        int i = 0;
        while (i + 8 <= len) {
            long k1 = 0;
            for (int j = 0; j < 8; j++) k1 |= ((long) (data[i + j] & 0xff)) << (8 * j);
            i += 8;
            k1 *= c1; k1 = Long.rotateLeft(k1, 31); k1 *= c2; h1 ^= k1;
            h1 = Long.rotateLeft(h1, 27); h1 += h2; h1 = h1 * 5 + 0x52dce729;
        }
        h1 ^= len; h2 ^= len;
        h1 += h2; h2 += h1;
        h1 ^= (h1 >>> 33); h1 *= 0xff51afd7ed558ccdL; h1 ^= (h1 >>> 33);
        return h1;
    }
}
```

### 5.3 Template Method — AbstractBloomFilter

```java
public abstract class AbstractBloomFilter<T> {
    protected final HashStrategy hashStrategy;
    protected final int numHashFunctions; // k
    protected final int numBits;          // m

    protected AbstractBloomFilter(HashStrategy hashStrategy, int numHashFunctions, int numBits) {
        this.hashStrategy = hashStrategy;
        this.numHashFunctions = numHashFunctions;
        this.numBits = numBits;
    }

    /** Template Method: fixed skeleton, subclasses only define what happens per index. */
    public final void add(T element) {
        int[] indices = hashStrategy.getIndices(element, numHashFunctions, numBits);
        doAdd(indices);
    }

    public final boolean mightContain(T element) {
        int[] indices = hashStrategy.getIndices(element, numHashFunctions, numBits);
        return doCheck(indices);
    }

    protected abstract void doAdd(int[] indices);
    protected abstract boolean doCheck(int[] indices);

    public abstract double estimatedFalsePositiveRate();
}
```

### 5.4 BloomFilter — the base concurrent implementation, with sizing Builder

```java
public class BloomFilter<T> extends AbstractBloomFilter<T> {
    private final BitArray bitArray;

    private BloomFilter(HashStrategy hashStrategy, int numHashFunctions, int numBits) {
        super(hashStrategy, numHashFunctions, numBits);
        this.bitArray = new BitArray(numBits);
    }

    @Override
    protected void doAdd(int[] indices) {
        for (int idx : indices) bitArray.set(idx); // each set() call is independently lock-free
    }

    @Override
    protected boolean doCheck(int[] indices) {
        for (int idx : indices) {
            if (!bitArray.get(idx)) return false; // definitely not present — short-circuit
        }
        return true; // possibly present
    }

    @Override
    public double estimatedFalsePositiveRate() {
        double bitsSet = bitArray.approxBitsSet();
        double ratio = bitsSet / numBits;
        return Math.pow(ratio, numHashFunctions);
    }

    public int getNumBits() { return numBits; }
    public int getNumHashFunctions() { return numHashFunctions; }

    public static class Builder<T> {
        private long expectedInsertions = 1_000_000;
        private double falsePositiveProbability = 0.01;
        private HashStrategy hashStrategy = new Murmur3HashStrategy();

        public Builder<T> expectedInsertions(long n) { this.expectedInsertions = n; return this; }
        public Builder<T> falsePositiveProbability(double p) { this.falsePositiveProbability = p; return this; }
        public Builder<T> hashStrategy(HashStrategy s) { this.hashStrategy = s; return this; }

        public BloomFilter<T> build() {
            int m = optimalNumBits(expectedInsertions, falsePositiveProbability);
            int k = optimalNumHashFunctions(expectedInsertions, m);
            return new BloomFilter<>(hashStrategy, k, m);
        }

        // m = -(n * ln(p)) / (ln(2)^2)
        private int optimalNumBits(long n, double p) {
            return (int) Math.ceil(-n * Math.log(p) / (Math.log(2) * Math.log(2)));
        }

        // k = (m/n) * ln(2)
        private int optimalNumHashFunctions(long n, int m) {
            return Math.max(1, (int) Math.round((m / (double) n) * Math.log(2)));
        }
    }
}
```

### 5.5 CountingBloomFilter — Decorator adding delete support

Uses `AtomicIntegerArray` counters instead of single bits — increment/decrement are also naturally CAS-able, same lock-free approach.

```java
import java.util.concurrent.atomic.AtomicIntegerArray;

public class CountingBloomFilter<T> extends AbstractBloomFilter<T> {
    private final AtomicIntegerArray counters;

    public CountingBloomFilter(HashStrategy hashStrategy, int numHashFunctions, int numBits) {
        super(hashStrategy, numHashFunctions, numBits);
        this.counters = new AtomicIntegerArray(numBits);
    }

    @Override
    protected void doAdd(int[] indices) {
        for (int idx : indices) incrementAtomically(idx);
    }

    @Override
    protected boolean doCheck(int[] indices) {
        for (int idx : indices) {
            if (counters.get(idx) == 0) return false;
        }
        return true;
    }

    /** The actual deletion — decrements each counter. Safe to call concurrently with add()/mightContain(). */
    public void remove(T element) {
        int[] indices = hashStrategy.getIndices(element, numHashFunctions, numBits);
        for (int idx : indices) decrementAtomically(idx);
    }

    private void incrementAtomically(int index) {
        int old, updated;
        do {
            old = counters.get(index);
            updated = old + 1;
        } while (!counters.compareAndSet(index, old, updated));
    }

    private void decrementAtomically(int index) {
        int old, updated;
        do {
            old = counters.get(index);
            if (old == 0) return; // guard against underflow (e.g., removing something never added)
            updated = old - 1;
        } while (!counters.compareAndSet(index, old, updated));
    }

    @Override
    public double estimatedFalsePositiveRate() {
        long nonZero = 0;
        for (int i = 0; i < counters.length(); i++) if (counters.get(i) > 0) nonZero++;
        return Math.pow((double) nonZero / numBits, numHashFunctions);
    }
}
```

> **Correctness caveat worth stating explicitly in an interview**: counting filters can still produce false negatives if `remove()` is called on an element that was never added (decrementing shared counters that other elements' hashes also touch) — this is a known limitation of counting Bloom filters generally, not specific to this implementation, and should be called out rather than silently glossed over.

### 5.6 ScalableBloomFilter — Composite that grows under load

```java
import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.locks.ReentrantLock;

public class ScalableBloomFilter<T> {
    private static final double GROWTH_TRIGGER_FILL_RATIO = 0.5; // grow when a generation is ~50% saturated
    private static final double TIGHTENING_RATIO = 0.9; // each new generation gets a tighter FPP target

    private final List<BloomFilter<T>> generations = new CopyOnWriteArrayList<>();
    private final ReentrantLock growthLock = new ReentrantLock(); // only held during the rare "add generation" event
    private final long baseExpectedInsertions;
    private double currentFpp;
    private final HashStrategy hashStrategy;

    public ScalableBloomFilter(long baseExpectedInsertions, double initialFpp, HashStrategy hashStrategy) {
        this.baseExpectedInsertions = baseExpectedInsertions;
        this.currentFpp = initialFpp;
        this.hashStrategy = hashStrategy;
        generations.add(newGeneration(baseExpectedInsertions, initialFpp));
    }

    private BloomFilter<T> newGeneration(long expectedInsertions, double fpp) {
        return new BloomFilter.Builder<T>()
                .expectedInsertions(expectedInsertions)
                .falsePositiveProbability(fpp)
                .hashStrategy(hashStrategy)
                .build();
    }

    public void add(T element) {
        // hot path: no lock. Always add to the current last generation.
        BloomFilter<T> current = generations.get(generations.size() - 1);
        current.add(element);
        maybeGrow(current);
    }

    public boolean mightContain(T element) {
        // hot path: no lock. CopyOnWriteArrayList gives a safe, consistent snapshot to iterate.
        for (BloomFilter<T> gen : generations) {
            if (gen.mightContain(element)) return true; // found in any generation -> possibly present
        }
        return false; // absent from every generation -> definitely not present
    }

    private void maybeGrow(BloomFilter<T> current) {
        if (current.estimatedFalsePositiveRate() < GROWTH_TRIGGER_FILL_RATIO) return;

        growthLock.lock(); // rare path — contention here doesn't matter for overall throughput
        try {
            // re-check after acquiring lock: another thread may have already grown it
            if (generations.get(generations.size() - 1) != current) return;

            currentFpp *= TIGHTENING_RATIO; // each generation is stricter, keeping cumulative FPP bounded
            BloomFilter<T> nextGen = newGeneration(baseExpectedInsertions, currentFpp);
            generations.add(nextGen); // CopyOnWriteArrayList: safe to append while others iterate concurrently
        } finally {
            growthLock.unlock();
        }
    }

    public int getGenerationCount() { return generations.size(); }
}
```

### 5.7 Putting it together

```java
public class ConcurrentBloomFilterDemo {
    public static void main(String[] args) throws InterruptedException {
        BloomFilter<String> filter = new BloomFilter.Builder<String>()
                .expectedInsertions(1_000_000)
                .falsePositiveProbability(0.01)
                .hashStrategy(new Murmur3HashStrategy())
                .build();

        int threadCount = 8;
        ExecutorService pool = Executors.newFixedThreadPool(threadCount);
        CountDownLatch latch = new CountDownLatch(threadCount);

        // concurrent writers
        for (int t = 0; t < threadCount; t++) {
            final int threadId = t;
            pool.submit(() -> {
                for (int i = 0; i < 100_000; i++) {
                    filter.add("thread" + threadId + "-item" + i);
                }
                latch.countDown();
            });
        }
        latch.await();
        pool.shutdown();

        System.out.println("mightContain(thread0-item5): " + filter.mightContain("thread0-item5")); // true
        System.out.println("mightContain(never-added): " + filter.mightContain("never-added"));       // false (usually)
        System.out.println("Estimated FPP: " + filter.estimatedFalsePositiveRate());

        // scalable variant — keeps accepting inserts well beyond original sizing
        ScalableBloomFilter<String> scalable = new ScalableBloomFilter<>(10_000, 0.01, new Murmur3HashStrategy());
        for (int i = 0; i < 100_000; i++) scalable.add("key-" + i);
        System.out.println("Generations after growth: " + scalable.getGenerationCount());
        System.out.println("mightContain(key-50000): " + scalable.mightContain("key-50000"));
    }
}
```

---

## 6. Why this shape holds up under follow-ups

- **"Why not just synchronize the whole `add()`/`mightContain()` methods?"** — this is the question the whole design answers: a global lock would serialize every thread regardless of which bits they touch, destroying throughput. The CAS-per-word approach lets threads updating *different* words proceed in true parallel, and even threads racing on the *same* word only retry briefly rather than blocking.
- **"What if two threads try to set the same bit at the same time?"** — walk through the CAS loop: both read the same `oldWord`, one wins the `compareAndSet`, the other retries, reads the now-updated word, sees the bit is already set, and returns `false` without re-writing. No corruption, no lost updates.
- **"Support merging two Bloom filters (union)"** → add a `union(BloomFilter other)` that ORs the two `BitArray`s word-by-word — only valid if `m`/`k`/hash function match; this is a natural extension of `BitArray`, no core logic disruption.
- **"Estimate cardinality (how many elements were actually added)"** → `BitArray.approxBitsSet()` already exists for exactly this — feed it into the standard Swamidass–Baldi estimator formula.
- **"Distribute the Bloom filter across multiple nodes (too big for one machine)"** → partition the bit array by index ranges across nodes (a distributed `BitArray` behind the same interface), or replicate a smaller filter per node and merge via `union` — the `BitArray` abstraction is exactly the seam for this.
- **"Make it persistent / durable across restarts"** → back `BitArray` with a memory-mapped file (`MappedByteBuffer`) instead of `AtomicLongArray` — same CAS-on-word approach works if you use `VarHandle`/`Unsafe` CAS on the mapped memory; the public `set`/`get` contract is unchanged.

---

Want me to extend this with **cardinality estimation math, a distributed/partitioned Bloom filter across nodes, benchmarking lock-free vs. striped-lock vs. global-lock throughput, or a Cuckoo filter comparison (which supports deletion natively without the false-negative caveat)**?