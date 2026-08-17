# The Complete Rust Collections Guide
### Interview Questions with Detailed Answers + Full Theory + Inner Architecture + Complete Tutorial

---

## Table of Contents

**Part A — Interview Questions**
1. [The Collections Landscape](#1-the-collections-landscape)
2. [`Vec<T>` — The Growable Array](#2-vect--the-growable-array)
3. [`VecDeque<T>` — The Double-Ended Queue](#3-vecdequet--the-double-ended-queue)
4. [`LinkedList<T>` — The Doubly Linked List](#4-linkedlistt--the-doubly-linked-list)
5. [`HashMap<K, V>` — The Hash Table](#5-hashmapk-v--the-hash-table)
6. [`BTreeMap<K, V>` — The Ordered Map](#6-btreemapk-v--the-ordered-map)
7. [`HashSet<T>` & `BTreeSet<T>` — Sets](#7-hashsett--btreesett--sets)
8. [`BinaryHeap<T>` — The Priority Queue](#8-binaryheapt--the-priority-queue)
9. [`String` & `&str` as Collections](#9-string--str-as-collections)
10. [Slices `&[T]` and Arrays `[T; N]`](#10-slices-t-and-arrays-t-n)
11. [Choosing the Right Collection](#11-choosing-the-right-collection)
12. [Iterators, `collect()` & the Entry API](#12-iterators-collect--the-entry-api)
13. [Ownership, Borrowing & Collection Gotchas](#13-ownership-borrowing--collection-gotchas)
14. [Common Pitfalls & Anti-Patterns](#14-common-pitfalls--anti-patterns)

**Part B — Complete Theory & Inner Architecture**
15. [Theoretical Deep Dive: Memory Layout, Growth Strategies, Hashing & Tree Internals](#15-theoretical-deep-dive-memory-layout-growth-strategies-hashing--tree-internals)

**Part C — Full Tutorial**
16. [Complete Tutorial: Building an LRU Cache and a Task Scheduler](#16-complete-tutorial-building-an-lru-cache-and-a-task-scheduler)

---

# Part A — Interview Questions

## 1. The Collections Landscape

### Q1. What collections does `std::collections` provide, and how are they grouped?
```
Sequences:   Vec<T>, VecDeque<T>, LinkedList<T>
Maps:        HashMap<K,V>, BTreeMap<K,V>
Sets:        HashSet<T>, BTreeSet<T>
Misc:        BinaryHeap<T>
```
The standard library groups collections by the access pattern they optimize for, not by implementation detail. **Sequences** preserve insertion/positional order and are indexed by position. **Maps** and **Sets** are keyed lookup structures — maps store key→value pairs, sets store keys only (in fact `HashSet<T>` is a thin wrapper around `HashMap<T, ()>`, and `BTreeSet<T>` around `BTreeMap<T, ()>`). **`BinaryHeap`** is neither — it's a priority queue that only guarantees efficient access to the *maximum* element, not full ordering. Every one of these is built on top of `Vec<T>`'s allocation machinery except `LinkedList`, which does its own node-by-node heap allocation, and the tree-based collections, which allocate B-Tree nodes.

Two more sequence-shaped types matter even though they aren't in `std::collections`: **arrays** `[T; N]` (fixed-size, stack-allocated, size known at compile time) and **slices** `&[T]` / `&mut [T]` (a borrowed *view* into any contiguous sequence — a `Vec`, an array, or another slice). Almost every collection method that takes "some sequence" actually takes `&[T]`, which is why understanding slices is prerequisite to understanding `Vec`.

### Q2. Why does Rust have *both* `HashMap` and `BTreeMap` instead of just one "the" map type?
```rust
use std::collections::{HashMap, BTreeMap};

let mut hm: HashMap<String, i32> = HashMap::new();
let mut bt: BTreeMap<String, i32> = BTreeMap::new();
// hm: O(1) avg get/insert, iteration order is UNSPECIFIED (looks random)
// bt: O(log n) get/insert, iteration order is ALWAYS sorted by key
```
This is one of the most common interview questions, and the answer is a direct trade-off between **speed** and **order**. `HashMap` gives average O(1) lookups by hashing the key into a bucket index, but that hash scrambles any relationship between keys, so iteration order is unspecified and can even change between runs (see Q13 on `HashMap`'s DoS-resistant random seed). `BTreeMap` gives O(log n) lookups but keeps keys in **sorted order** at all times via a B-Tree, so iterating a `BTreeMap` always yields keys in ascending order, and it supports **range queries** (`map.range(5..10)`) that `HashMap` cannot do at all. Rule of thumb: default to `HashMap` for raw speed; reach for `BTreeMap` when you need sorted iteration, range queries, or deterministic ordering (e.g., reproducible test output, snapshot testing, or serializing to a canonical form).

### Q3. What do all growable Rust collections have in common at the type level?
```rust
// Every std collection owns its elements and is generic over them.
struct Vec<T> { /* ptr, len, capacity */ }
struct HashMap<K, V, S = RandomState> { /* ... */ }
```
Every `std::collections` type is generic, owns its contents (no garbage collector — when the collection is dropped, every element is dropped too, recursively), and grows/shrinks its own heap allocation as needed. None of them are `Copy` (copying a `Vec` would silently deep-clone or double-free, so Rust requires an explicit `.clone()`). Most implement `IntoIterator` in three flavors — `iter()` (borrows `&T`), `iter_mut()` (borrows `&mut T`), and `into_iter()` (consumes the collection, yields owned `T`) — which is why `for x in &v`, `for x in &mut v`, and `for x in v` all compile but mean different things.

---

## 2. `Vec<T>` — The Growable Array

### Q4. What is a `Vec<T>` at the memory level, and how does it differ from an array?
```rust
let arr: [i32; 3] = [1, 2, 3];       // fixed size, lives on the stack (or inline wherever it's stored)
let vec: Vec<i32> = vec![1, 2, 3];   // heap-allocated buffer, grows at runtime

// Vec<T> is exactly this (conceptually):
struct Vec<T> {
    ptr: *mut T,      // pointer to heap buffer
    len: usize,       // number of initialized elements
    cap: usize,       // number of elements the buffer can hold before reallocating
}
```
A `Vec<T>` is a triple of (pointer, length, capacity) — 24 bytes on a 64-bit machine — where `ptr` points at a contiguous heap allocation holding `cap` slots, of which the first `len` are initialized `T` values. `len <= cap` always. This is essentially what `ArrayList` is in Java, `std::vector` in C++, or a Python `list` (modulo Python's dynamic typing). An array `[T; N]` has no heap indirection at all: `N` is part of the *type*, baked in at compile time, so `[i32; 3]` and `[i32; 4]` are different, incompatible types, and the data lives wherever the array itself lives (stack, inside a struct, etc.) with zero allocation overhead.

### Q5. How does `Vec::push` achieve amortized O(1) time if reallocation is O(n)?
```rust
let mut v = Vec::new();
for i in 0..5 {
    println!("len={} cap={}", v.len(), v.capacity());
    v.push(i);
}
// len=0 cap=0
// len=1 cap=1
// len=2 cap=2
// len=3 cap=4   <- reallocated, capacity doubled
// len=4 cap=4
// (next push at len=4 would double cap to 8)
```
When `len == cap` and a new element is pushed, `Vec` allocates a **new**, larger buffer (`std`'s growth factor is 2x), copies every existing element over via `memcpy`, frees the old buffer, then appends the new element. That single push is O(n). But because capacity doubles, reallocation only happens O(log n) times over n pushes, and the total work across all n pushes sums to O(n) (a geometric series: n + n/2 + n/4 + ... ≈ 2n). Dividing total work by n operations gives O(1) **amortized** — meaning it's O(1) on average across a long sequence of operations, even though any *individual* push can occasionally be O(n). This is why you should call `Vec::with_capacity(n)` up front when you know the final size — it skips all the intermediate reallocations entirely.

### Q6. What's the difference between indexing `v[i]` and `v.get(i)`?
```rust
let v = vec![10, 20, 30];

let a = v[5];          // PANICS: index out of bounds
let b = v.get(5);      // returns None, no panic
let c = v.get(1);      // returns Some(&20)

if let Some(val) = v.get(1) {
    println!("{val}");
}
```
`Index`/`v[i]` panics on out-of-bounds access — appropriate when an out-of-range index represents a genuine logic bug you want to fail loudly on. `get(i)` returns `Option<&T>`, letting you handle "index might not exist" as a normal, recoverable case (e.g., indexing based on user input or a computed offset). Both perform the exact same bounds check internally; `v[i]` just unwraps-or-panics for you. Prefer `get` at any boundary where the index isn't provably in range.

### Q7. What happens when you remove an element from the middle of a `Vec`? Contrast `remove` and `swap_remove`.
```rust
let mut v = vec!['a', 'b', 'c', 'd'];
v.remove(1);        // v = ['a', 'c', 'd']  -- shifts everything after index 1 left by one: O(n)

let mut v2 = vec!['a', 'b', 'c', 'd'];
v2.swap_remove(1);  // v2 = ['a', 'd', 'c'] -- swaps index 1 with the LAST element, then pops: O(1)
```
`remove(i)` preserves order but must shift every subsequent element left one slot, costing O(n). `swap_remove(i)` is O(1) because it swaps the target with the last element and pops, but it **does not preserve order**. Use `swap_remove` whenever element order doesn't matter (e.g., an unordered pool of worker handles) — it's a very common micro-optimization in hot loops.

### Q8. Why can't you push to a `Vec` while holding a reference into it?
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);              // ERROR: cannot borrow `v` as mutable because it is also borrowed as immutable
println!("{first}");
```
`push` might trigger a reallocation, which frees the old buffer — if `first` still pointed at the old buffer, it would be a **dangling pointer** (a classic C++ "iterator/reference invalidation" bug). Rust's borrow checker rejects this at *compile time*: `first` holds an immutable borrow of `v` that's still alive at the `push` call (it's used afterward in `println!`), and `push` needs a mutable borrow, so the two conflict. This is precisely the class of memory-safety bug (use-after-free via invalidated reference) that the borrow checker exists to eliminate, and it's a favorite interview probe — the compiler error itself demonstrates why Rust doesn't need a garbage collector to be memory-safe.

### Q9. What does `Vec::drain` do, and when is it preferable to `clear()` or a manual loop?
```rust
let mut v = vec![1, 2, 3, 4, 5];
let removed: Vec<i32> = v.drain(1..3).collect(); // removes indices 1,2, returns them
// removed = [2, 3], v = [1, 4, 5]

let mut v2 = vec![1, 2, 3];
v2.drain(..);   // removes everything but keeps the allocation (unlike v2 = Vec::new())
```
`drain(range)` removes the given range and returns an iterator over the removed elements, letting you both *consume* and *keep* elements in one pass without an intermediate allocation — useful for "partition out the matching elements" patterns. `drain(..)` empties the whole `Vec` like `clear()`, but unlike `clear()`, you get ownership of the drained values rather than having them dropped in place. Note `drain`'s iterator must be fully consumed (or dropped) to actually perform the removal — leaking it via `mem::forget` is a documented (safe, but logically surprising) way to leave the `Vec` in a temporarily inconsistent length state.

---

## 3. `VecDeque<T>` — The Double-Ended Queue

### Q10. What problem does `VecDeque` solve that `Vec` doesn't?
```rust
use std::collections::VecDeque;

let mut v: Vec<i32> = vec![1, 2, 3];
v.insert(0, 0);          // O(n) - shifts all 3 elements right

let mut dq: VecDeque<i32> = VecDeque::from([1, 2, 3]);
dq.push_front(0);        // O(1) amortized
dq.push_back(4);         // O(1) amortized
// dq = [0, 1, 2, 3, 4]
```
`Vec` is only efficient at growing/shrinking from the **back**; anything at the front requires shifting all remaining elements, which is O(n). `VecDeque` ("double-ended queue") is implemented as a **ring buffer** (circular buffer) over a single heap allocation, with a `head` index that can wrap around, so push/pop are O(1) amortized at **both** ends. Use `VecDeque` for queues, sliding-window algorithms, BFS frontiers, or any "add/remove from both ends" workload; use `Vec` when you only ever touch the back (it has slightly less indexing overhead since there's no wraparound arithmetic).

### Q11. How is a ring buffer laid out in memory, and what's the catch with contiguous slice access?
```rust
// Buffer of capacity 6, after some front/back pushes, logically holding [a, b, c]:
// physical: [c, _, _, _, a, b]
//                        ^head=4, len=3  (wraps around: index 4,5,0 = a,b,c)

let mut dq: VecDeque<i32> = VecDeque::from([1,2,3]);
dq.push_front(0);
let (front_slice, back_slice) = dq.as_slices(); // may be split into TWO slices if wrapped
```
A ring buffer tracks a `head` index and length; logical index `i` maps to physical index `(head + i) % capacity`. This makes both-end operations O(1) without shifting data, but it means the elements are not necessarily contiguous in memory — if the buffer has wrapped, the data is split across two runs (end-of-buffer and start-of-buffer). That's why `VecDeque` doesn't implement `Deref<Target = [T]>` the way `Vec` does; instead `as_slices()`/`as_mut_slices()` return a *pair* of slices, and `make_contiguous()` will physically rotate the buffer (an O(n) operation) if you need one single `&mut [T]` (e.g., to call `.sort()`, which needs a contiguous slice).

---

## 4. `LinkedList<T>` — The Doubly Linked List

### Q12. Why does the Rust docs themselves say "you should almost never use `LinkedList`"?
```rust
use std::collections::LinkedList;
let mut list: LinkedList<i32> = LinkedList::new();
list.push_back(1);
list.push_back(2);
```
`LinkedList<T>` is a doubly-linked list where every element is a **separate heap allocation** connected by pointers. This means: no cache locality (each `.next()` is a pointer chase to a random heap address — brutal on modern CPUs with deep cache hierarchies), higher per-element memory overhead (two pointers per node beyond the data itself), and O(n) indexing since there's no random access. In almost every case `VecDeque` is strictly better: same O(1) push/pop at both ends, but backed by one contiguous, cache-friendly allocation. The only real justifications for `LinkedList` are: O(1) *splicing* of one entire list into the middle of another without moving elements (`splice` methods), or when you need stable pointers into individual nodes that survive insertions/removals elsewhere in the list — both rare in application code. Interviewers ask this specifically to check whether you understand that "linked list" being a CS-101 staple doesn't mean it's a good default in a systems language with cache-aware performance characteristics.

---

## 5. `HashMap<K, V>` — The Hash Table

### Q13. Why does `std::HashMap` use SipHash by default, and what does that trade off?
```rust
use std::collections::HashMap;
let mut m: HashMap<&str, i32> = HashMap::new(); // uses RandomState -> SipHash-1-3, seeded randomly per-process
m.insert("a", 1);
```
Rust's default `HashMap` hasher is **SipHash-1-3**, a cryptographically-hardened hash function, seeded with a random value generated **per `HashMap` instance** (technically per `RandomState`, drawn from OS randomness) at program startup. This is a deliberate security choice: naive hash functions (like FNV or a simple multiplicative hash) are vulnerable to **"HashDoS"** — an attacker who knows your hash function can craft inputs (e.g., form field names, JSON keys) that all collide into the same bucket, degrading every lookup from O(1) to O(n) and turning a web request into an algorithmic-complexity denial-of-service attack. SipHash is resistant to this because an attacker can't predict bucket placement without knowing the random per-process seed. The cost: SipHash is measurably slower than non-cryptographic hashes. When you control the input space (internal, trusted keys — e.g., small integer IDs) and need maximum speed, it's idiomatic to swap in a faster hasher such as `rustc-hash`'s `FxHashMap` or `ahash`'s `AHashMap` via `HashMap<K, V, S>`'s third type parameter.

### Q14. Walk through what `HashMap::insert` actually does internally (Rust's `hashbrown`/SwissTable design).
```
1. hash = SipHash(key)
2. Split hash into: 7 high bits ("H2", stored in a separate metadata byte array)
                     remaining bits -> initial bucket group index
3. Scan metadata bytes in SIMD-sized groups (16 at a time) for a matching H2 byte
4. On match, compare the actual key for equality (handles hash collisions)
5. On no match, or an empty/deleted slot found -> that's the insertion point
```
Since Rust 1.36, `std::HashMap` is implemented via **`hashbrown`**, a Rust port of Google's **SwissTable** design (also used in Abseil's C++ `flat_hash_map`). Instead of the classic "array of `Option<(K,V)>` buckets with chaining" you might picture from a textbook, SwissTable keeps keys/values in a flat array and a **parallel metadata array** of one byte per slot, encoding either "empty," "deleted (tombstone)," or the top 7 bits of that slot's hash ("H2"). Probing scans metadata bytes 16-at-a-time using SIMD instructions, which is extremely cache- and branch-predictor-friendly compared to chasing pointers through a linked chain — this is precisely why it outperforms the "array of linked lists" hash table design taught in most algorithms courses. Full equality comparison against the real key only happens after a metadata byte matches, since H2 collisions (1-in-128 by construction) are cheap to filter out first.

### Q15. What is the Entry API, and why is it the idiomatic way to do "insert or update"?
```rust
use std::collections::HashMap;

// Naive (does TWO lookups):
let mut counts: HashMap<&str, i32> = HashMap::new();
if counts.contains_key("a") {
    *counts.get_mut("a").unwrap() += 1;
} else {
    counts.insert("a", 1);
}

// Idiomatic (ONE lookup):
*counts.entry("a").or_insert(0) += 1;

// Common variants:
counts.entry("b").or_insert_with(|| expensive_default());
counts.entry("c").and_modify(|v| *v += 1).or_insert(1);
```
`entry(key)` returns an `Entry` enum (`Occupied` or `Vacant`) representing that key's slot in a **single** hash lookup, and the combinator methods (`or_insert`, `or_insert_with`, `and_modify`, `or_default`) let you handle both branches without a second lookup. The naive version above does two full hash+probe cycles (`contains_key`, then `get_mut`/`insert`); the entry version does exactly one. This is the textbook example of Rust's "make the efficient way also the ergonomic way" design philosophy, and interviewers frequently ask you to write a word-frequency counter specifically to see if you reach for `entry` instead of a `contains_key`/`insert` pair.

### Q16. What are `Hash` and `Eq`, and why must they agree with each other?
```rust
#[derive(PartialEq, Eq, Hash)]
struct UserId(u64);
// derive requires: if a == b, then hash(a) must == hash(b)

// BAD: manual impls that disagree
struct BadKey(f64);
impl PartialEq for BadKey { fn eq(&self, o: &Self) -> bool { (self.0 - o.0).abs() < 0.001 } }
// If you also impl Hash naively on the raw bits, two "equal" BadKeys can hash differently -> broken HashMap
```
Any type used as a `HashMap` key must implement `Eq` (a stricter, total version of `PartialEq` — this is *why* `f64` alone can't be a `HashMap` key: `NaN != NaN` violates reflexivity, so `f64` only implements `PartialEq`, not `Eq`) and `Hash`. The contract every implementer must uphold: **if `a == b`, then `hash(a) == hash(b)`**. This is not enforced by the compiler — it's a logical invariant you must maintain yourself when hand-writing `Hash`/`Eq` (as opposed to `#[derive(Hash, Eq)]`, which derives both from the same field list and is guaranteed consistent). Violate it and you get a `HashMap` that silently fails to find keys it should find, because the key hashes into the wrong bucket relative to where equal keys live — a nasty, hard-to-debug class of bug, and a favorite "spot the bug" interview question.

### Q17. Why doesn't `HashMap` implement `Hash` itself, and why is iteration order unstable across runs?
Because `HashMap`'s own iteration order depends on the random per-instance seed (Q13) and the internal table layout (which changes with resizes), two `HashMap`s with identical key-value pairs can iterate in different orders — so `HashMap` cannot implement `Hash` (which would need a canonical, order-independent way to combine element hashes) or `Ord`, though it does implement `PartialEq` (comparing as sets of pairs, ignoring order). This randomness is also why you'll see interview/test code do `let mut keys: Vec<_> = map.keys().collect(); keys.sort();` before printing or asserting — comparing against a fixed expected string would be flaky otherwise. If you need deterministic iteration, that's a strong signal to reach for `BTreeMap` instead (Q2).

---

## 6. `BTreeMap<K, V>` — The Ordered Map

### Q18. What is a B-Tree, and why does Rust use one instead of the red-black tree that C++'s `std::map` uses?
```
A B-Tree node (order ~11, tuned to fit a cache line) holds MULTIPLE sorted keys per node:
        [ 10 | 25 | 40 ]
       /    |    |    \
   <10   10-25  25-40  >40
```
A classic balanced BST (red-black tree, AVL tree — what C++'s `std::map` uses) stores exactly one key per node and does one pointer-chasing comparison per level, with tree height ≈ log₂(n). A **B-Tree** node instead holds many keys (Rust's `BTreeMap` targets roughly 11 per internal node, chosen so a node fits in a small number of cache lines) and does a **linear or binary scan within the node** before descending, so the tree height is roughly log_B(n) — far shallower for the same n. Fewer levels means fewer cache-missing pointer chases, which is why B-Trees dominate red-black trees in practice on modern hardware despite doing "more work" per node in big-O terms: the real bottleneck is memory latency, not comparison count. This mirrors exactly why databases and filesystems use B-Trees/B+Trees for on-disk indexes — same cache/page locality argument, just at a different level of the memory hierarchy.

### Q19. What can `BTreeMap` do that `HashMap` fundamentally cannot?
```rust
use std::collections::BTreeMap;

let mut scores: BTreeMap<i32, &str> = BTreeMap::new();
scores.insert(90, "Alice");
scores.insert(75, "Bob");
scores.insert(88, "Carol");

for (score, name) in &scores {
    println!("{score}: {name}");   // ALWAYS prints in ascending key order: 75, 88, 90
}

// Range queries - impossible on HashMap:
for (score, name) in scores.range(80..100) {
    println!("{score}: {name}");   // 88, 90
}

println!("{:?}", scores.first_key_value()); // Some((&75, "Bob"))
println!("{:?}", scores.last_key_value());  // Some((&90, "Alice"))
```
Because `BTreeMap` maintains keys in sorted order at all times, it supports **ordered iteration** (deterministic, ascending), **range queries** (`.range(lo..hi)`, `.range(..=x)`, all in O(log n + k) for k results), and **order-statistics-style queries** (`first_key_value`, `last_key_value`, `range` combined with `.next()`/`.next_back()` for "closest key ≥/≤ x"). `HashMap` has none of these — hashing intentionally destroys any ordering relationship between keys, so there's no way to ask a `HashMap` "give me everything between 80 and 100" without a full O(n) scan. Any time an interview problem mentions "find the nearest," "range between," or "sorted output," that's the signal to reach for `BTreeMap`/`BTreeSet` over the hash-based equivalents.

---

## 7. `HashSet<T>` & `BTreeSet<T>` — Sets

### Q20. What is a `HashSet` under the hood, and when do you reach for it over a `Vec`?
```rust
use std::collections::HashSet;

let mut seen: HashSet<i32> = HashSet::new();
let nums = [1, 2, 3, 2, 1, 4];
let mut unique = Vec::new();
for n in nums {
    if seen.insert(n) {           // insert() returns true if the value was NOT already present
        unique.push(n);
    }
}
// unique = [1, 2, 3, 4]  -- dedup while preserving first-seen order, O(n) instead of O(n^2)
```
`HashSet<T>` is literally `HashMap<T, ()>` — same SwissTable machinery, just with a zero-sized value type so no space is spent storing values. Reach for it whenever you need **membership testing** ("have I seen this before?", "is x in this collection?") — a `Vec::contains` is O(n) per check (O(n²) total across n checks), while `HashSet::contains`/`insert` is O(1) average, making it the standard tool for deduplication, visited-node tracking in graph traversal, and detecting duplicates.

### Q21. What set-algebra operations does `HashSet`/`BTreeSet` give you for free?
```rust
use std::collections::HashSet;

let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
let b: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();

let union: HashSet<_>        = a.union(&b).collect();          // {1,2,3,4,5,6}
let intersection: HashSet<_> = a.intersection(&b).collect();   // {3,4}
let difference: HashSet<_>   = a.difference(&b).collect();     // {1,2}       (in a, not in b)
let sym_diff: HashSet<_>     = a.symmetric_difference(&b).collect(); // {1,2,5,6}

assert!(a.is_subset(&union));
```
Both set types implement the standard set-algebra operations as lazy iterator-returning methods (`union`, `intersection`, `difference`, `symmetric_difference`), plus predicate methods (`is_subset`, `is_superset`, `is_disjoint`). `BTreeSet`'s versions can exploit the sorted ordering to run these as a single linear merge pass (like merging two sorted lists), while `HashSet`'s versions iterate one set and probe the other. Knowing these exist (rather than hand-rolling them with nested loops or nested nested `contains` calls) is a quick tell of Rust fluency.

---

## 8. `BinaryHeap<T>` — The Priority Queue

### Q22. What does `BinaryHeap` guarantee, and how does that differ from a fully sorted structure?
```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(4);
heap.push(1);
heap.push(5);

println!("{:?}", heap.peek()); // Some(&5) -- the max, in O(1)
while let Some(x) = heap.pop() {
    print!("{x} ");             // 5 4 3 1 1  -- pops in descending order, each pop O(log n)
}
```
`BinaryHeap<T>` is a **max-heap**: it only guarantees that `peek()`/`pop()` give you the largest element in O(1)/O(log n) respectively. It does *not* maintain full sorted order internally (unlike `BTreeSet`) — internally it's a binary tree encoded implicitly in a `Vec` (child of index `i` at `2i+1`/`2i+2`), satisfying the **heap property** (every parent ≥ its children) but not a total order between siblings. This weaker guarantee is exactly why it's faster to build and maintain than a sorted structure: `push` is O(log n) ("sift up"), `pop` is O(log n) ("sift down"), and building a heap from n elements via `BinaryHeap::from(vec)` is O(n) (not O(n log n)) using Floyd's build-heap algorithm. Reach for it for: "top-k" problems, Dijkstra's/Prim's/A* algorithms (need "next-closest" repeatedly), and any streaming scenario needing quick access to the current extreme without maintaining full order.

### Q23. `BinaryHeap` is a max-heap — how do you use it as a min-heap?
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
min_heap.push(Reverse(4));

while let Some(Reverse(x)) = min_heap.pop() {
    print!("{x} ");   // 1 3 4  -- ascending, i.e. min-first
}
```
`std::cmp::Reverse<T>` is a zero-cost newtype wrapper that flips the `Ord` implementation of whatever it wraps. Wrapping every pushed value in `Reverse` means the heap's "largest by `Reverse`'s ordering" is actually the *smallest* underlying value, turning the max-heap into a min-heap for free — no custom `Ord` impl needed, no separate min-heap type in `std`. This trick, plus implementing `Ord`/`PartialOrd` manually on a custom struct (e.g., a `(priority, task)` pair where you compare only on `priority`), is the standard pattern for Dijkstra's algorithm implementations in Rust, and interviewers frequently ask you to implement Dijkstra specifically to see if you know `Reverse`.

---

## 9. `String` & `&str` as Collections

### Q24. Why is Rust's `String` UTF-8, and why can't you index it with `s[0]`?
```rust
let s = String::from("héllo");
// let c = s[0];              // ERROR: `String` cannot be indexed by `{integer}`

println!("{}", s.len());              // 6  (bytes: 'h','é' is 2 bytes,'l','l','o')
println!("{}", s.chars().count());    // 5  (Unicode scalar values)

for c in s.chars() {
    print!("{c} ");    // h é l l o
}
for b in s.bytes() {
    print!("{b} ");    // 104 195 169 108 108 111
}
```
`String` (owned, growable) and `&str` (borrowed string slice) are both **guaranteed-valid UTF-8** byte sequences — this guarantee is load-bearing throughout the ecosystem (e.g., you can always safely treat a `&str` as bytes, but not vice versa without validation). Because UTF-8 is a variable-width encoding (1–4 bytes per character), `s[0]` can't mean "give me the first character" — there's no O(1) way to know where character boundaries fall, and *any* indexing API would either lie about what it returns or secretly be O(n). Rust simply refuses to compile `s[0]` rather than give you a footgun. Instead you choose your unit explicitly: `.chars()` for Unicode scalar values, `.bytes()` for raw bytes, or `.chars().nth(i)` (still O(n), but explicitly so) for the nth character. You *can* slice by byte range (`&s[0..1]`), but it panics at runtime if the range doesn't fall on a UTF-8 character boundary — this is the classic "sliced a string with a multi-byte character in it and panicked in production" bug.

### Q25. `String` vs `&str` — what's the ownership/mutability story, and when do you use which in a function signature?
```rust
fn greet(name: &str) -> String {         // borrow in, own out
    format!("Hello, {name}!")
}

let owned = String::from("world");
let borrowed_literal = "world";           // &'static str, lives in the binary's read-only data
println!("{}", greet(&owned));            // &String auto-derefs to &str
println!("{}", greet(borrowed_literal));
```
`String` is a heap-allocated, growable, owned buffer (structurally: a `Vec<u8>` with the UTF-8 invariant enforced) — use it when you need to *own* or *mutate* text (build it up with `push_str`, return it from a function, store it in a struct). `&str` is a borrowed view (pointer + length) into UTF-8 bytes owned by *someone else* — a `String`, a `&'static` string literal compiled into the binary, or a slice of either. The idiomatic rule: accept `&str` in function parameters (it accepts both `&String` via deref coercion and string literals, maximizing caller flexibility and avoiding forced allocation), and return `String` when you're constructing new owned data. This is the exact same ownership pattern as `Vec<T>` vs `&[T]` (Q26) — `String` is to `&str` what `Vec<T>` is to `&[T]`.

---

## 10. Slices `&[T]` and Arrays `[T; N]`

### Q26. What is a slice, and why do idiomatic function signatures prefer `&[T]` over `&Vec<T>`?
```rust
fn sum(nums: &[i32]) -> i32 {   // accepts a Vec, an array, or another slice — all via one signature
    nums.iter().sum()
}

let v = vec![1, 2, 3];
let a = [4, 5, 6];
sum(&v);          // &Vec<i32> auto-derefs (coerces) to &[i32]
sum(&a);          // &[i32; 3] coerces to &[i32]
sum(&v[1..]);      // a sub-slice works too
```
A slice `&[T]` is a **fat pointer**: a pointer to the first element plus a length (16 bytes on 64-bit), borrowing a contiguous run of `T`s it doesn't own. `&Vec<T>` only ever refers to a whole, specifically-`Vec`-backed sequence; `&[T]` is the more general, most-flexible type that a `Vec`, an array, or any sub-range of either can all coerce into. Taking `&[T]` (or `&mut [T]`) as a parameter is strictly more reusable than `&Vec<T>` for the same reason `&str` beats `&String` — it's Rust's version of "accept the most general type your function actually needs," and Clippy's `ptr_arg` lint will flag `&Vec<T>`/`&String` parameters specifically for this.

### Q27. When would you reach for a fixed-size array `[T; N]` over a `Vec<T>`?
```rust
struct Matrix3x3 {
    data: [[f64; 3]; 3],   // no heap allocation at all, size known at compile time
}

fn checksum(bytes: [u8; 4]) -> u32 { /* ... */ }  // arrays of known size are Copy if T: Copy
```
Use `[T; N]` when N is fixed at compile time and small-to-moderate (stack-allocated, zero heap allocation, and `Copy` if `T: Copy` — a `Vec` is never `Copy`). This matters for performance-sensitive code (no allocator call), embedded/`no_std` targets (no allocator at all, possibly), and cases where the size is a meaningful part of the type's contract (a 3x3 matrix, an RGB triple `[u8; 3]`, a fixed-width hash `[u8; 32]`). Reach for `Vec<T>` the moment the size is only known at runtime or needs to grow/shrink.

---

## 11. Choosing the Right Collection

### Q28. Give a decision framework for picking a collection under interview pressure.
```
Need order preserved, index-based access, mostly append at the end?        -> Vec<T>
Need push/pop at BOTH front and back (queue, sliding window, BFS)?         -> VecDeque<T>
Need fast key->value lookup, don't care about iteration order?             -> HashMap<K,V>
Need key->value lookup AND sorted iteration / range queries?               -> BTreeMap<K,V>
Need fast "have I seen this?" / dedup, order doesn't matter?               -> HashSet<T>
Need "have I seen this?" AND sorted iteration / range queries?             -> BTreeSet<T>
Need repeated access to the current min/max (Dijkstra, top-k, scheduling)? -> BinaryHeap<T>
Need O(1) splicing of whole sublists, or stable per-node pointers?         -> LinkedList<T> (rare!)
Need a borrowed, read-only view into any of the above?                     -> &[T] / &str
```
This table alone answers the majority of "which collection would you use for X" interview questions. The meta-skill being tested isn't memorizing APIs — it's recognizing which *access pattern* (sequential vs. keyed, ordered vs. unordered, both-ends vs. one-end) a problem actually needs, then picking the cheapest structure that provides it. A strong signal in interviews: explicitly say *why* you rejected the alternatives (e.g., "I could use a `Vec` and scan it, but since I'm checking membership in a loop, that's O(n²) overall — a `HashSet` gets me O(n) instead").

### Q29. Big-O cheat sheet across the core collections (all `std`, average case unless noted).

| Operation                | `Vec` | `VecDeque` | `HashMap` | `BTreeMap` | `BinaryHeap` |
|---------------------------|-------|------------|-----------|------------|--------------|
| Push/pop back              | O(1)* | O(1)*      | —         | —          | —            |
| Push/pop front              | O(n)  | O(1)*      | —         | —          | —            |
| Insert/remove middle       | O(n)  | O(n)       | —         | —          | —            |
| Get by index                | O(1)  | O(1)       | —         | —          | —            |
| Insert/get/remove by key   | —     | —          | O(1)*     | O(log n)   | —            |
| Push (insert)               | —     | —          | —         | —          | O(log n)     |
| Peek min/max                 | —     | —          | —         | O(log n)†  | O(1)         |
| Pop min/max                  | —     | —          | —         | O(log n)   | O(log n)     |
| Contains / membership       | O(n)  | O(n)       | O(1)*     | O(log n)   | O(n)         |
| Ordered iteration            | n/a (already ordered) | n/a | ✗ | ✓ | ✗ |

\* amortized. † via `first_key_value`/`last_key_value`, which is O(log n) to descend to the tree edge.

---

## 12. Iterators, `collect()` & the Entry API

### Q30. How does `.collect()` know which collection to build?
```rust
let v: Vec<i32> = (1..5).collect();
let s: HashSet<i32> = (1..5).collect();
let m: HashMap<i32, i32> = (1..5).map(|x| (x, x * x)).collect();
let string: String = vec!['h', 'i'].into_iter().collect();

// turbofish syntax when type inference has nothing to go on:
let v2 = (1..5).collect::<Vec<_>>();
```
`collect()` is generic over its return type via the `FromIterator` trait — every collection that can be meaningfully built from an iterator implements `FromIterator<Item>`, and `collect::<T>()` simply calls `T::from_iter`. The compiler picks the right impl via **type inference** from the binding's declared type (`let v: Vec<i32> = ...`), the function's declared return type, or an explicit turbofish (`::<Vec<_>>()`) when neither is available. This is a good moment to mention `FromIterator` by name in an interview — it signals you understand collect() isn't magic, it's one trait method dispatched on the target type.

### Q31. What's the difference between `iter()`, `iter_mut()`, and `into_iter()`, and what does `for x in collection` actually desugar to?
```rust
let v = vec![1, 2, 3];

for x in &v { }        // x: &i32       -- calls v.iter(),     v still usable after
for x in &mut v.clone() { }  // x: &mut i32   -- calls v.iter_mut(), lets you mutate in place
for x in v { }          // x: i32        -- calls v.into_iter(), v is CONSUMED, unusable after
```
`for item in collection` desugars to `for item in IntoIterator::into_iter(collection)`. Which of the three iterator flavors you get depends on what you hand it: `for x in &v` calls `(&v).into_iter()`, which for `Vec<T>` is implemented to be equivalent to `v.iter()` (yields `&T`, `v` still owned by the caller afterward); `for x in &mut v` similarly yields `&mut T` via `iter_mut()`; and `for x in v` (handing over the collection itself, not a reference) calls `v.into_iter()`, which yields owned `T` and **moves** `v`, so `v` cannot be used again after the loop. This trips up almost every intermediate Rust developer at least once — "why can't I use my `Vec` after this `for` loop" nearly always traces back to accidentally iterating by value instead of by reference.

---

## 13. Ownership, Borrowing & Collection Gotchas

### Q32. Why does this fail to compile, and what are the two idiomatic fixes?
```rust
let mut v = vec![1, 2, 3, 4, 5];
for x in &v {
    if *x % 2 == 0 {
        v.remove(0);   // ERROR: cannot borrow `v` as mutable while borrowed as immutable
    }
}

// Fix 1: retain (best when the rule is "keep/drop based on a predicate")
v.retain(|x| x % 2 != 0);

// Fix 2: collect indices first, then mutate (best for more complex logic)
let to_remove: Vec<usize> = v.iter().enumerate()
    .filter(|(_, x)| **x % 2 == 0)
    .map(|(i, _)| i)
    .collect();
for i in to_remove.into_iter().rev() {   // reverse order so earlier removals don't shift later indices
    v.remove(i);
}
```
Iterating `&v` holds an immutable borrow for the loop's whole duration, so any mutating call on `v` inside the loop body is rejected — this is the borrow checker preventing the exact "modify a collection while iterating it" bug that's a segfault or corrupted-iterator crash in C++ and undefined/surprising behavior in many other languages. `retain` is the idiomatic one-liner for filter-in-place. When the logic is too complex for a single predicate, collect what needs to change first (as indices, or as a separate `Vec` of new elements), *then* mutate after the borrow from iteration has ended — note the `.rev()` in fix 2, needed because removing index 1 shifts everything after it, invalidating any later indices you collected earlier in ascending order.

### Q33. Why does mutating a value obtained from `HashMap::get_mut` sometimes require care with the borrow's lifetime?
```rust
use std::collections::HashMap;
let mut m: HashMap<&str, Vec<i32>> = HashMap::new();
m.insert("a", vec![1, 2, 3]);

if let Some(v) = m.get_mut("a") {
    v.push(4);
}   // <- mutable borrow of `m` ends here

m.insert("b", vec![5]);   // fine now, no outstanding borrow
```
`get_mut` returns `Option<&mut V>`, a mutable borrow of the map that must not outlive its last use — the compiler enforces this via **non-lexical lifetimes** (the borrow's actual scope is inferred from last-use, not the enclosing block), which is why the `if let` pattern above compiles even though the borrow is technically "inside" a block that appears to extend further. The gotcha shows up when you try to hold onto that `&mut V` across a *second* map operation (e.g., calling `m.insert(...)` while `v` from `get_mut` is still alive) — that's rejected for the same reason `Vec::push` rejects held references (Q8): a concurrent structural mutation to the map (e.g., a resize) could invalidate the reference.

---

## 14. Common Pitfalls & Anti-Patterns

### Q34. What's wrong with this "check then act" pattern, and why does Rust push you toward the Entry API / `if let` instead?
```rust
// ANTI-PATTERN: two lookups, and a race if this were ever made concurrent
use std::collections::HashMap;
let mut m: HashMap<String, i32> = HashMap::new();
if !m.contains_key("x") {
    m.insert("x".to_string(), 0);
}
let val = m.get_mut("x").unwrap();  // second lookup
*val += 1;

// IDIOMATIC:
*m.entry("x".to_string()).or_insert(0) += 1;
```
Beyond the doubled lookup cost (Q15), "check then act" is a textbook **TOCTOU** (time-of-check-to-time-of-use) pattern — harmless here in single-threaded code, but the exact shape of bug that becomes a real race condition the moment similar logic is applied to shared state under concurrency (e.g., a `Mutex<HashMap<..>>` checked and then acted on in two separate lock acquisitions). Training yourself to reach for `entry()` (or `if let Some(x) = ...` for reads) instead of `contains_key` + separate access is good practice that also happens to generalize correctly if the code later needs to become concurrent.

### Q35. Why is `.clone()`-ing a whole `Vec`/`HashMap` just to satisfy the borrow checker usually a code smell?
```rust
// SMELL: cloning to dodge a borrow error
fn process(data: &mut Vec<i32>) {
    let snapshot = data.clone();          // O(n) allocation + copy, just to sidestep borrowing
    for x in &snapshot {
        if *x > 10 { data.push(*x * 2); }
    }
}

// BETTER: compute what to add first, then mutate once
fn process_better(data: &mut Vec<i32>) {
    let to_add: Vec<i32> = data.iter().filter(|&&x| x > 10).map(|x| x * 2).collect();
    data.extend(to_add);
}
```
Reaching for `.clone()` the instant the borrow checker complains is the single most common "learning Rust" anti-pattern — it *compiles*, but it silently pays an O(n) allocation-and-copy cost every time, and it papers over a design that usually has a cheaper, more idiomatic shape (compute-then-mutate, splitting a struct into independently-borrowable fields, or using indices instead of references). It's fine as a *temporary* unblock while learning, and occasionally the right permanent answer when data really is small or truly needs to be duplicated (e.g., handing an independent copy to another thread) — but in an interview, defaulting straight to `.clone()` without considering `retain`/`entry`/collect-then-mutate/restructuring reads as not yet fluent in ownership, so it's worth narrating the trade-off out loud even if you do end up cloning.

### Q36. What's the risk in `HashMap<K, V>` when `K` is a mutable reference or contains interior mutability, and why does Rust's type system only partially protect you?
```rust
use std::cell::Cell;
use std::collections::HashSet;

let a = Cell::new(1);
let mut set = HashSet::new();
// set.insert(a); // if Cell<i32> implemented Hash (it doesn't, precisely to prevent this),
                   // you could mutate `a` after inserting it, changing its hash,
                   // and the set could never find it again by its new value.
```
The `Hash`/`Eq` contract (Q16) assumes a key's hash **never changes** while it's stored in the map/set — if it could, the key would end up "stuck" in the wrong bucket relative to its current value, silently breaking lookups (a real, if rare, footgun even in garbage-collected languages like Java, where mutating a key already in a `HashMap` after insertion is an equally well-known bug). Rust's type system mitigates this by simply not implementing `Hash` for interior-mutability wrappers like `Cell<T>`/`RefCell<T>` — but it can't fully prevent it, because a key type can still contain a `Cell` deep inside a field that participates in a hand-written `Hash` impl, or you can mutate a key through a raw pointer/`unsafe`. The practical rule: never store a key whose `Hash`/`Eq`-relevant fields can be mutated while it lives in the map; if you need to "update" a key, remove the old entry and re-insert under the new key instead.

---

# Part B — Complete Theory & Inner Architecture

## 15. Theoretical Deep Dive: Memory Layout, Growth Strategies, Hashing & Tree Internals

### 15.1 `Vec<T>`'s allocator contract, precisely
`Vec<T>` never allocates until the first element is pushed (`Vec::new()` starts with `cap == 0` and a **dangling, non-null** pointer — dangling-but-non-null is important: it keeps `null`-based niche optimizations available elsewhere, e.g. `Option<Vec<T>>` stays the same size as `Vec<T>`). Growth uses `RawVec` internally, which calls the global allocator's `realloc` when possible (letting the allocator extend the block in place if there's free space after it, avoiding a copy) and falls back to alloc-copy-free otherwise. The **growth factor** for `std::Vec` is 2x (grow-by-doubling) — chosen because it guarantees amortized O(1) push (Q5) while keeping the "wasted" over-allocated memory bounded to at most roughly the size of the vec itself at any moment. Some other languages/collections use 1.5x (e.g., some `.NET` collections) as a tighter memory/time trade-off; Rust favors 2x for push-heavy workloads and expects you to call `with_capacity`/`reserve`/`shrink_to_fit` explicitly when you know more about the access pattern than the default heuristic does.

### 15.2 Why SwissTable-style flat hashing beats classic separate chaining
The hash table taught in most textbooks uses **separate chaining**: an array of buckets, each a linked list of entries that hashed into it. This has poor cache behavior — following a chain means pointer-chasing to scattered heap locations, and each node carries per-allocation overhead. **Open addressing** (what `hashbrown`/SwissTable, and most modern high-performance hash tables, use instead) stores entries directly in one flat array and resolves collisions by probing to another slot in the *same* array according to a fixed scheme. `hashbrown` specifically uses a **1-byte metadata array** in parallel with the data array (not intermixed with it — the split is itself a deliberate cache-layout choice), where each metadata byte is either a sentinel for empty/deleted or the top 7 bits of that slot's full hash. Because a slot's fate can usually be decided just by scanning metadata bytes (cheap, SIMD-parallelizable, no full-key equality check needed for a mismatch), probing is dramatically cheaper than the naive "compare full keys one by one" approach open addressing is often (wrongly) assumed to require. Probing itself uses **quadratic probing within 16-slot groups**, chosen to avoid the clustering pathologies of naive linear probing while still being cache-group-friendly.

### 15.3 Load factor, resizing, and why `with_capacity` matters more for `HashMap` than for `Vec`
`hashbrown` targets roughly an **87.5% (7/8) maximum load factor** before triggering a resize (grow, and rebuild the whole table — every element gets rehashed into new bucket positions since bucket index depends on both the hash *and* the table's current size). This resize is O(n) and — unlike `Vec`'s resize, which is "just" a memcpy — involves recomputing probe positions for every single element, since a change in table size changes which bucket each key's hash maps to. This is precisely why pre-sizing with `HashMap::with_capacity(n)` matters even more for hash tables than `Vec::with_capacity` matters for vectors in a tight loop: it avoids not just repeated allocation, but repeated full-table rehashing.

### 15.4 B-Trees in depth: node structure, splits, and why height stays so low
Each internal `BTreeMap` node holds up to `2*B - 1` keys (Rust tunes `B` so a node is a small, fixed number of machine words — historically `B = 6`, i.e. up to 11 keys per node, chosen empirically to fit cache-line-friendly node sizes) sorted internally, plus up to `2*B` child pointers interleaved between them, satisfying the invariant "every key in child `i` is between key `i-1` and key `i` of the parent." **Insertion** finds the correct leaf via a scan/binary-search down the tree, inserts the key there, and if the node overflows past `2*B - 1` keys, **splits** it in half, pushing its median key up into the parent (recursively, possibly all the way to the root, which is the only way a B-Tree's height ever grows — by one level, uniformly, from the top, which is exactly why B-Trees stay perfectly balanced with no separate rebalancing pass the way red-black trees need rotations). **Deletion** is the mirror image: if a node underflows below `B - 1` keys, it either borrows a key from an adjacent sibling (rotation) or merges with a sibling, again possibly propagating up. Because branching factor `B` is large (~6–11), even a `BTreeMap` with a billion entries has a height of only around 7–8 — versus ~30 for a binary tree with the same n — which is the entire performance story: fewer levels means fewer cache-missing memory accesses per lookup, even though each level does "more work" (an in-node scan instead of one comparison).

### 15.5 Amortized analysis, formally
The "amortized O(1)" claim for `Vec::push` (Q5) and `HashMap::insert` (Q14/15.3) rests on the **aggregate method** of amortized analysis: sum the total cost of n operations, then divide by n. For `Vec` doubling: the total cost of n pushes is n (one unit per push) plus the cost of all reallocations, which is `1 + 2 + 4 + ... + n ≈ 2n` (each element gets copied at most once per doubling it survives, and the copies form a geometric series dominated by the last, largest term). Total: `O(n) + O(2n) = O(3n) = O(n)` for n operations, i.e. O(1) per operation on average — even though the *specific* push that triggers a reallocation is individually O(n). This distinction (worst-case-per-call vs. amortized-over-a-sequence) is worth stating explicitly in an interview, since "O(1)" and "amortized O(1)" are different, precise claims, and conflating them is a common (and easily caught) imprecision.

### 15.6 Zero-sized types (ZSTs) and how `HashSet`/`BTreeSet` get "free" as wrappers
`()` (unit) is a **zero-sized type** — it occupies no memory at all, and the compiler specializes allocation/storage logic so that a collection of `()` values (or, structurally, the "value" half of `HashMap<T, ()>`/`BTreeMap<T, ()>`) does no allocation work for those values whatsoever; only the keys cost anything. This is why `HashSet<T>` and `BTreeSet<T>` genuinely are just `HashMap<T, ()>`/`BTreeMap<T, ()>` under the hood in the standard library source, rather than separate hand-optimized implementations — the ZST optimization makes the "wasted" value storage compile away to nothing, so there's no performance reason to duplicate the logic.

### 15.7 Drop order and recursive ownership
When any `std` collection is dropped, its `Drop` impl iterates its contents and drops each element in turn (which recursively drops *their* owned contents, and so on) before freeing its own backing allocation(s). This is why a `Vec<String>` going out of scope frees not just the `Vec`'s own buffer but every individual `String`'s heap buffer too, fully automatically, with no garbage collector — ownership forms a tree (or DAG-shaped-but-acyclic-in-practice structure, since `Rc`/`Arc` cycles are the one documented leak hazard) rooted at whatever's on the stack, and dropping the root cascades deterministically through the whole tree at a statically-known point (end of scope), which is the core mechanical reason Rust needs neither a GC nor manual `free()` calls.

---

# Part C — Full Tutorial

## 16. Complete Tutorial: Building an LRU Cache and a Task Scheduler

This tutorial builds two small, realistic systems that between them exercise every collection covered above: an **LRU (Least Recently Used) cache** (`HashMap` + `VecDeque`), and a **priority task scheduler** (`BinaryHeap` + `BTreeMap` + `HashSet`). Both are extremely common "prove you know collections" interview exercises.

### 16.1 LRU Cache: the design

An LRU cache needs O(1) `get`/`put`, and on every access must be able to (a) find the value by key instantly, and (b) know which key was used *least* recently, in case it needs to evict. A `HashMap` alone gives you (a) but not (b) — hashing destroys order. The standard solution pairs a `HashMap<K, V>` for O(1) lookup with an ordering structure that tracks recency. We'll build a simplified version using `HashMap` + `VecDeque` (an "age list" you push the most-recently-used key onto the back of, and re-order on access) — production-grade implementations typically use an intrusive doubly linked list for true O(1) reordering, but the `VecDeque` version below stays within `std::collections` and keeps the code approachable while being close enough to O(1) in practice for interview purposes; we'll call out where a real linked list would help.

```rust
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,   // front = least recently used, back = most recently used
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        LruCache { capacity, map: HashMap::new(), order: VecDeque::new() }
    }

    fn touch(&mut self, key: &K) {
        // Move `key` to the back (most-recently-used end).
        // NOTE: this is O(n) because VecDeque has no O(1) "remove by value" —
        // a real production LRU uses an intrusive linked list + HashMap<K, NodePtr>
        // to make this O(1). We accept O(n) here to stay within std::collections.
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        self.order.push_back(key.clone());
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.touch(key);
            self.map.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            self.touch(&key);
            return;
        }

        if self.map.len() >= self.capacity {
            // Evict the least-recently-used key: the front of `order`.
            if let Some(oldest) = self.order.pop_front() {
                self.map.remove(&oldest);
            }
        }

        self.map.insert(key.clone(), value);
        self.order.push_back(key);
    }
}

fn main() {
    let mut cache: LruCache<&str, i32> = LruCache::new(2);
    cache.put("a", 1);
    cache.put("b", 2);
    assert_eq!(cache.get(&"a"), Some(&1));   // "a" is now most-recently-used
    cache.put("c", 3);                        // capacity 2 -> evicts "b" (least recently used)
    assert_eq!(cache.get(&"b"), None);        // evicted
    assert_eq!(cache.get(&"a"), Some(&1));    // survived
    assert_eq!(cache.get(&"c"), Some(&3));    // survived
    println!("LRU cache behaves correctly");
}
```
Notice the pattern: `HashMap` answers "does this key exist, and what's its value" in O(1); `VecDeque` answers "what's the access-order relationship between keys." Every collections concept from Part A shows up here: the Entry-API-adjacent `contains_key`+`insert` pattern (Q34 — arguably this *should* be tightened with `entry()` in the update branch), `pop_front`/`push_back` on the deque for O(1) both-ended access (Q10), and the explicit trade-off comment about what a real O(1) implementation would need (a linked list with a `HashMap<K, NodePtr>`, i.e. exactly the "rare, legitimate `LinkedList` use case" from Q12 — stable per-node pointers for O(1) removal from the middle).

### 16.2 Task Scheduler: the design

Now a priority task scheduler: tasks have a priority and must run in priority order (`BinaryHeap`), but we also want to be able to look up a task's current status by ID (`HashMap`), cancel a task before it runs (`HashSet` of cancelled IDs — cheaper than removing arbitrary elements from a heap, which isn't O(log n) for an arbitrary element, only for the root), and produce a sorted audit log of completion times (`BTreeMap`).

```rust
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Task {
    id: u32,
    priority: u8,   // higher = more urgent
    name: String,
}

// Custom Ord: BinaryHeap is a max-heap, so "greatest" = highest priority.
// Ties broken by lower id first (earlier-submitted wins), for determinism.
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.id.cmp(&self.id)) // reversed: lower id = "greater" on tie
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Scheduler {
    queue: BinaryHeap<Task>,
    cancelled: HashSet<u32>,          // O(1) cancel, checked lazily on pop
    status: HashMap<u32, &'static str>,
    completed_at: BTreeMap<u32, u32>, // tick -> task id, for a chronological audit log
    tick: u32,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            queue: BinaryHeap::new(),
            cancelled: HashSet::new(),
            status: HashMap::new(),
            completed_at: BTreeMap::new(),
            tick: 0,
        }
    }

    fn submit(&mut self, task: Task) {
        self.status.insert(task.id, "queued");
        self.queue.push(task);
    }

    fn cancel(&mut self, id: u32) {
        // Don't scan/rebuild the heap (O(n)) — just mark cancelled and
        // skip it lazily when it's popped. Classic "lazy deletion" pattern.
        self.cancelled.insert(id);
        self.status.insert(id, "cancelled");
    }

    /// Runs the single next non-cancelled task, if any.
    fn run_next(&mut self) -> Option<Task> {
        while let Some(task) = self.queue.pop() {
            if self.cancelled.remove(&task.id) {
                continue; // was cancelled — drop it, keep popping
            }
            self.tick += 1;
            self.status.insert(task.id, "completed");
            self.completed_at.insert(self.tick, task.id);
            return Some(task);
        }
        None
    }

    /// Chronological audit log, oldest first — free thanks to BTreeMap's sorted iteration.
    fn audit_log(&self) -> impl Iterator<Item = (&u32, &u32)> {
        self.completed_at.iter()
    }
}

fn main() {
    let mut sched = Scheduler::new();
    sched.submit(Task { id: 1, priority: 5, name: "low".into() });
    sched.submit(Task { id: 2, priority: 9, name: "urgent".into() });
    sched.submit(Task { id: 3, priority: 9, name: "also urgent".into() });
    sched.cancel(3); // cancel before it runs

    while let Some(t) = sched.run_next() {
        println!("ran: {} (priority {})", t.name, t.priority);
    }
    // Expected order: "urgent" (priority 9, id 2, submitted before the tie was cancelled),
    // then "low" (priority 5). Task 3 is skipped via lazy deletion.

    println!("audit log (tick -> task id): {:?}", sched.audit_log().collect::<Vec<_>>());
}
```
This exercises `BinaryHeap` with a hand-written `Ord` (Q23's sibling technique — instead of `Reverse`, we directly define "greater priority is greater," with a tie-break that itself reverses `id` ordering to prefer earlier submissions), the **lazy deletion** pattern for O(1) cancellation against a structure that doesn't support efficient arbitrary removal (a very common systems-design trick worth naming explicitly in interviews — it trades a bit of wasted heap space for avoiding an O(n) heap rebuild), and `BTreeMap` purely for the "give me this in sorted/chronological order for free" property (Q19) that a `HashMap` could never provide without an explicit sort step.

### 16.3 What to say out loud in an interview while building something like this
Interviewers weight *justification* over final code. As you reach for each collection, say the trade-off: "I'm using a `HashSet` here instead of checking membership against a `Vec` because I'll be cancelling potentially many tasks and I want O(1) instead of O(n) per cancellation." "I'm not removing cancelled tasks from the heap directly because `BinaryHeap` doesn't support efficient removal of an arbitrary element — only the max — so I'm using lazy deletion instead and paying a small, bounded amount of wasted space." That's the signal that separates "knows the APIs" from "understands why the APIs are shaped the way they are," which is what this whole guide has been building toward.
