# Java 8 → Java 27: The Complete Evolution Guide

> **Scope:** Every feature release from Java 8 (March 2014) to Java 27 (September 2026), covering language features, APIs, JVM internals, internal architectural styles, garbage collection, memory utilization and performance metrics, followed by a large interview question bank.
>
> **Current state (Nov 2026):** Latest release is **JDK 27** (GA 15 Sep 2026, non-LTS). Latest **LTS** is **JDK 25** (Sep 2025). Next release: JDK 28 (Mar 2027), expected to bring the first Project Valhalla features.

---

## Table of Contents

1. [Release Timeline & Support Model](#1-release-timeline--support-model)
2. [Quick Comparison Matrix](#2-quick-comparison-matrix)
3. [Version-by-Version Deep Dive](#3-version-by-version-deep-dive)
4. [JVM Internal Architecture](#4-jvm-internal-architecture)
5. [Architectural Styles & Patterns Used Inside the JDK](#5-architectural-styles--patterns-used-inside-the-jdk)
6. [Garbage Collection: Complete Guide](#6-garbage-collection-complete-guide)
7. [Memory Utilization & Layout](#7-memory-utilization--layout)
8. [Performance Metrics, Startup & Footprint](#8-performance-metrics-startup--footprint)
9. [Concurrency Evolution](#9-concurrency-evolution)
10. [Observability & Tooling](#10-observability--tooling)
11. [Removed / Deprecated Features](#11-removed--deprecated-features)
12. [Migration Guide (8 → 11 → 17 → 21 → 25)](#12-migration-guide)
13. [The OpenJDK Projects (Amber, Loom, Panama, Valhalla, Leyden)](#13-the-openjdk-projects)
14. [Interview Questions](#14-interview-questions)

---

## 1. Release Timeline & Support Model

Since Java 10, the JDK ships a **feature release every six months** (March and September). Every **two years** (since JDK 17; previously every three) one release is designated **LTS (Long-Term Support)**.

| Version | GA Date | LTS? | Headline |
|---|---|---|---|
| **8** | Mar 2014 | ✅ LTS | Lambdas, Streams, java.time, Metaspace |
| 9 | Sep 2017 | | Modules (JPMS), JShell, G1 default |
| 10 | Mar 2018 | | `var`, AppCDS, container awareness |
| **11** | Sep 2018 | ✅ LTS | HttpClient, ZGC (exp), Epsilon, JFR open-sourced |
| 12 | Mar 2019 | | Switch expressions (preview), Shenandoah (exp) |
| 13 | Sep 2019 | | Text blocks (preview), dynamic CDS |
| 14 | Mar 2020 | | Switch expressions final, helpful NPEs, CMS removed |
| 15 | Sep 2020 | | Text blocks final, ZGC & Shenandoah production, hidden classes |
| 16 | Mar 2021 | | Records, `instanceof` patterns, elastic metaspace |
| **17** | Sep 2021 | ✅ LTS | Sealed classes, strong encapsulation |
| 18 | Mar 2022 | | UTF-8 default, `jwebserver` |
| 19 | Sep 2022 | | Virtual threads (preview), record patterns (preview) |
| 20 | Mar 2023 | | Scoped values (incubator) |
| **21** | Sep 2023 | ✅ LTS | Virtual threads, pattern-matching switch, sequenced collections, generational ZGC |
| 22 | Mar 2024 | | FFM API final, unnamed variables `_` |
| 23 | Sep 2024 | | Markdown Javadoc, generational ZGC default |
| 24 | Mar 2025 | | Stream gatherers, no-pinning virtual threads, AOT cache, post-quantum crypto |
| **25** | Sep 2025 | ✅ LTS | Compact source files, scoped values, compact object headers (product) |
| 26 | Mar 2026 | | HTTP/3, Applet API removed, AOT caching with any GC |
| 27 | Sep 2026 | | G1 default everywhere, compact headers by default, PQ hybrid TLS |

**Versioning scheme (JEP 322, JDK 10+):** `$FEATURE.$INTERIM.$UPDATE.$PATCH` (e.g., `21.0.5`). Before that: `1.8.0_402`.

**Preview / Incubator lifecycle:**
- **Incubator module** (`jdk.incubator.*`): non-final API, needs `--add-modules`.
- **Preview feature**: fully specified but impermanent; needs `--enable-preview --release N`.
- **Experimental VM feature**: needs `-XX:+UnlockExperimentalVMOptions`.
- **Product**: permanent, supported.

---

## 2. Quick Comparison Matrix

| Area | Java 8 | Java 11 | Java 17 | Java 21 | Java 25 | Java 27 |
|---|---|---|---|---|---|---|
| Default GC (server) | Parallel | G1 | G1 | G1 | G1 | G1 (everywhere) |
| Low-latency GCs | CMS | ZGC (exp), CMS (deprecated) | ZGC, Shenandoah | Generational ZGC | Gen ZGC only, Gen Shenandoah | same |
| Class metadata | Metaspace | Metaspace | Elastic Metaspace | Elastic Metaspace | + compact headers (opt-in) | compact headers default |
| Object header (64-bit, compressed) | 12 B | 12 B | 12 B | 12 B | 8 B opt-in | 8 B default |
| String storage | `char[]` (UTF-16) | `byte[]` + coder (compact strings) | same | same | same | same |
| Concurrency | ForkJoin, CompletableFuture | + Flow API | same | **Virtual threads** | + Scoped values, no pinning | + Structured concurrency (preview) |
| Modules | Classpath only | JPMS | Strong encapsulation | same | Module imports | same |
| Data modeling | POJOs | POJOs, `var` | Records, sealed | Record patterns, switch patterns | Flexible constructors | Primitive patterns (preview) |
| Native interop | JNI | JNI | JNI + FFM incubator | FFM preview | FFM final (22) | same |
| HTTP client | `HttpURLConnection` | HttpClient (HTTP/1.1, 2) | same | same | same | + HTTP/3 (26) |
| Startup tech | CDS (boot) | AppCDS | Default CDS + dynamic | same | AOT cache (Leyden) | AOT cache with any GC |
| Default charset | Platform-dependent | Platform | Platform | UTF-8 (18+) | UTF-8 | UTF-8 |

---

## 3. Version-by-Version Deep Dive

### 3.1 Java 8 (LTS, March 2014) — The Functional Revolution

#### Language
- **Lambda expressions** — `(a, b) -> a + b`. Compiled to `invokedynamic` calls bootstrapped by `LambdaMetafactory`, **not** anonymous inner classes. The lambda body becomes a private synthetic method (`lambda$main$0`); the JVM spins a hidden class at first invocation.
- **Functional interfaces** — `@FunctionalInterface`; `java.util.function` (`Function`, `Supplier`, `Consumer`, `Predicate`, `BiFunction`, `UnaryOperator`, primitive specializations like `IntFunction`).
- **Method references** — `String::length`, `System.out::println`, `ArrayList::new`.
- **Default and static methods in interfaces** — enabled library evolution (e.g., `Collection.stream()`, `Map.forEach`).
- **Repeating annotations** and **type annotations** (`@NonNull String`).
- Improved type inference (target typing).

#### APIs
- **Stream API** — lazy, pipeline-based, optionally parallel (`parallelStream()` uses common `ForkJoinPool`).
- **`Optional<T>`**.
- **`java.time` (JSR-310)** — immutable `LocalDate`, `LocalDateTime`, `ZonedDateTime`, `Instant`, `Duration`, `Period`; replaces `Date`/`Calendar`.
- **`CompletableFuture`** — composable async pipelines.
- **Concurrency**: `StampedLock`, `LongAdder`/`DoubleAdder` (striped counters), `ConcurrentHashMap` rewritten (no segments; CAS + `synchronized` on bin head; bins treeify to red-black trees beyond 8 entries).
- `HashMap` treeification of buckets (8 → tree, 6 → back to list) to defeat hash-collision DoS.
- `Base64`, `String.join`, `Arrays.parallelSort`, `Files.lines`.
- **Nashorn** JavaScript engine (replaced Rhino).

#### JVM
- **PermGen removed → Metaspace** (JEP 122). Class metadata in native memory; grows dynamically; `-XX:MaxMetaspaceSize`.
- Default GC: **Parallel GC** on server-class machines.
- Tiered compilation on by default (since 7u/8).
- Container awareness backported in **8u191** (`-XX:+UseContainerSupport`).

```java
List<String> names = people.stream()
    .filter(p -> p.age() > 18)
    .map(Person::name)
    .sorted()
    .collect(Collectors.toList());
```

---

### 3.2 Java 9 (Sept 2017) — Modularity

#### Language
- **Private interface methods**.
- **Try-with-resources on effectively-final variables**: `try (in) { ... }`.
- Diamond `<>` with anonymous inner classes.
- `@SafeVarargs` on private methods.
- `_` became a keyword (reserved).

#### APIs
- **JPMS — Java Platform Module System (Project Jigsaw)**: `module-info.java`, `requires`, `exports`, `opens`, `provides … with`, `uses`. JDK itself split into ~70 modules.
- **Collection factory methods**: `List.of`, `Set.of`, `Map.of`, `Map.ofEntries` (immutable, null-hostile, randomized iteration order for Set/Map).
- **Stream**: `takeWhile`, `dropWhile`, `ofNullable`, 3-arg `iterate`.
- **Optional**: `ifPresentOrElse`, `or`, `stream`.
- **Process API**: `ProcessHandle` (pid, children, onExit).
- **Flow API** (`java.util.concurrent.Flow`) — Reactive Streams interfaces.
- **VarHandle** — safe replacement for many `sun.misc.Unsafe` uses (atomic/ordered access modes).
- **Stack-Walking API** (`StackWalker`) — lazy, filterable.
- Multi-release JARs.
- HTTP/2 client (incubator).

#### JVM & Tools
- **G1 becomes the default GC** (JEP 248).
- **Compact Strings** (JEP 254): `String` backed by `byte[]` + `coder` byte; Latin-1 strings use 1 byte/char → ~50% less memory for most strings.
- **Indify String Concatenation** (JEP 280): `+` compiled to `invokedynamic` → `StringConcatFactory`.
- **Segmented Code Cache** (JEP 197): non-method, profiled, non-profiled segments.
- **Unified JVM Logging** (JEP 158): `-Xlog:gc*:file=gc.log`.
- **JShell** REPL, **jlink** (custom runtime images), `jdeps` improvements.
- CMS deprecated (JEP 291).

---

### 3.3 Java 10 (March 2018)

- **Local-variable type inference — `var`** (JEP 286). Compile-time only; not dynamic typing.
- **Application Class-Data Sharing (AppCDS)** (JEP 310) — share app classes across JVMs, faster startup.
- **Parallel Full GC for G1** (JEP 307) — G1's fallback full GC was single-threaded before.
- **Thread-Local Handshakes** (JEP 312) — stop individual threads without a global safepoint.
- **Container awareness** — JVM reads cgroup CPU/memory limits (`-XX:MaxRAMPercentage`, `-XX:InitialRAMPercentage`).
- Experimental **Graal JIT** (JEP 317).
- APIs: `List.copyOf`, `Collectors.toUnmodifiableList`, `Optional.orElseThrow()`.
- Time-based release versioning (JEP 322).

---

### 3.4 Java 11 (LTS, Sept 2018)

#### Language
- `var` in lambda parameters (JEP 323) — allows annotations: `(@NonNull var x) -> ...`.
- **Nest-based access control** (JEP 181) — inner classes access private members without synthetic bridge methods.

#### APIs
- **HTTP Client** standardized (`java.net.http`, JEP 321): HTTP/1.1 & HTTP/2, sync & async, WebSocket.
- **String**: `isBlank`, `lines`, `strip`, `stripLeading`, `stripTrailing`, `repeat`.
- **Files**: `readString`, `writeString`.
- `Predicate.not`, `Collection.toArray(IntFunction)`, `Optional.isEmpty`.
- TLS 1.3 (JEP 332), ChaCha20-Poly1305, Curve25519/448.

#### JVM
- **Epsilon GC** (JEP 318) — no-op allocator; for testing & ultra-short-lived jobs.
- **ZGC** experimental (JEP 333) — Linux x64.
- **Java Flight Recorder** open-sourced (JEP 328) + JDK Mission Control.
- Low-overhead heap profiling (JEP 331).
- Dynamic class-file constants `CONSTANT_Dynamic` (JEP 309).
- **Launch single-file source programs**: `java Hello.java` (JEP 330).

#### Removed
- **Java EE & CORBA modules** (JAXB, JAX-WS, JAF, javax.annotation, JTA) — JEP 320.
- JavaFX unbundled; Java Web Start / applets in Oracle JDK removed.
- Nashorn deprecated.

> **Licensing note:** From 11, Oracle JDK requires a commercial license for production (later relaxed under the NFTC license for 17+). OpenJDK builds (Temurin, Corretto, Zulu, Microsoft, Liberica) are functionally identical.

---

### 3.5 Java 12 (March 2019)
- Switch expressions (preview).
- **Shenandoah GC** experimental (JEP 189) — Red Hat.
- G1 **abortable mixed collections** (JEP 344) & **promptly return unused committed memory** (JEP 346).
- **Default CDS archive** for JDK classes (JEP 341).
- JMH microbenchmark suite in JDK source (JEP 230).
- APIs: `String.indent`, `String.transform`, `Collectors.teeing`, `CompactNumberFormat`, `Files.mismatch`.

### 3.6 Java 13 (Sept 2019)
- Text blocks (preview), `yield` in switch expressions (2nd preview).
- **Dynamic CDS archives** (JEP 350): `-XX:ArchiveClassesAtExit`.
- ZGC **uncommit unused memory** (JEP 351).
- Reimplemented legacy `Socket` API (JEP 353) — prerequisite for virtual threads.

### 3.7 Java 14 (March 2020)
- **Switch expressions final** (JEP 361).
- Records (preview), `instanceof` pattern matching (preview).
- **Helpful NullPointerExceptions** (JEP 358): `Cannot invoke "String.length()" because "user.name" is null`.
- **CMS GC removed** (JEP 363).
- ZGC on macOS & Windows; **NUMA-aware G1** (JEP 345).
- **JFR Event Streaming** (JEP 349) — consume JFR data in-process continuously.
- Foreign-Memory Access API (incubator), `jpackage` (incubator).

```java
int days = switch (month) {
    case FEB -> 28;
    case APR, JUN, SEP, NOV -> 30;
    default -> { yield 31; }
};
```

### 3.8 Java 15 (Sept 2020)
- **Text blocks final** (JEP 378).
- Sealed classes (preview).
- **Hidden classes** (JEP 371) — non-discoverable classes for frameworks/lambdas; replaces `Unsafe.defineAnonymousClass`.
- **ZGC production-ready** (JEP 377); **Shenandoah production-ready** (JEP 379).
- **Nashorn removed** (JEP 372).
- **Biased locking deprecated/disabled** (JEP 374) — complexity outweighed benefit on modern CPUs.
- EdDSA signatures (JEP 339).

```java
String json = """
    {
      "name": "Java",
      "version": 15
    }
    """;
```

### 3.9 Java 16 (March 2021)
- **Records final** (JEP 395) — transparent, immutable data carriers; auto `equals`, `hashCode`, `toString`, accessors, canonical constructor.
- **Pattern matching for `instanceof` final** (JEP 394).
- `Stream.toList()` (unmodifiable).
- **Elastic Metaspace** (JEP 387) — returns unused metaspace to OS faster, less fragmentation (buddy allocator, 4 MB chunks → granular commit).
- ZGC **concurrent thread-stack processing** (JEP 376) — sub-millisecond pauses independent of thread count.
- **Strong encapsulation of JDK internals by default** (JEP 396): `--illegal-access=deny`.
- `jpackage` final, Unix-domain socket channels (JEP 380), Vector API (1st incubator), Alpine/musl and Windows/AArch64 ports.
- Warnings for synchronizing on value-based classes (JEP 390).
- OpenJDK migrated to Git/GitHub.

```java
record Point(int x, int y) {
    Point { if (x < 0) throw new IllegalArgumentException(); } // compact ctor
}
if (obj instanceof Point p && p.x() > 0) { ... }
```

### 3.10 Java 17 (LTS, Sept 2021)
- **Sealed classes final** (JEP 409): `sealed interface Shape permits Circle, Square {}`.
- **Strongly encapsulate JDK internals** (JEP 403) — `--illegal-access` removed; only `--add-opens` works.
- Restore always-strict floating-point (JEP 306) — `strictfp` redundant.
- **Enhanced pseudo-random generators** (JEP 356): `RandomGenerator`, `RandomGeneratorFactory`, LXM family.
- Pattern matching for switch (preview).
- **Context-specific deserialization filters** (JEP 415).
- macOS/AArch64 (Apple Silicon) port, Metal rendering pipeline.
- **Deprecated**: Security Manager (JEP 411), Applet API for removal.
- **Removed**: RMI Activation, experimental AOT (jaotc) & Graal JIT.

### 3.11 Java 18 (March 2022)
- **UTF-8 by default** (JEP 400) — `Charset.defaultCharset()` is UTF-8 everywhere.
- **Simple Web Server** `jwebserver` (JEP 408).
- **`@snippet` in Javadoc** (JEP 413).
- Core reflection reimplemented on **method handles** (JEP 416).
- Internet-address resolution SPI (JEP 418).
- **Finalization deprecated for removal** (JEP 421).

### 3.12 Java 19 (Sept 2022)
- **Virtual threads** (preview, JEP 425).
- Record patterns (preview), FFM API (preview), Structured Concurrency (incubator).
- Linux/RISC-V port.

### 3.13 Java 20 (March 2023)
- Scoped values (incubator); further previews of virtual threads, record patterns, switch patterns, FFM, structured concurrency.

### 3.14 Java 21 (LTS, Sept 2023) — The Loom Release
- **Virtual threads final** (JEP 444).
- **Record patterns final** (JEP 440).
- **Pattern matching for switch final** (JEP 441) — type patterns, guards (`when`), null handling, exhaustiveness with sealed types.
- **Sequenced Collections** (JEP 431): `SequencedCollection`, `SequencedSet`, `SequencedMap` → `getFirst`, `getLast`, `addFirst`, `reversed()`.
- **Generational ZGC** (JEP 439) — `-XX:+UseZGC -XX:+ZGenerational`.
- **Key Encapsulation Mechanism API** (JEP 452).
- Prepare to disallow dynamic agent loading (JEP 451) — warnings.
- Previews: String Templates (later withdrawn), unnamed patterns `_`, unnamed classes & instance main, structured concurrency, scoped values.
- Deprecated 32-bit Windows x86 port.

```java
sealed interface Shape permits Circle, Rect {}
record Circle(double r) implements Shape {}
record Rect(double w, double h) implements Shape {}

double area(Shape s) {
    return switch (s) {
        case Circle(double r)          -> Math.PI * r * r;
        case Rect(double w, double h) when w == h -> w * w;
        case Rect(double w, double h)  -> w * h;
    }; // exhaustive: no default needed
}

try (var exec = Executors.newVirtualThreadPerTaskExecutor()) {
    IntStream.range(0, 100_000).forEach(i ->
        exec.submit(() -> { Thread.sleep(1000); return i; }));
}
```

### 3.15 Java 22 (March 2024)
- **Foreign Function & Memory API final** (JEP 454) — replaces JNI for most uses: `Linker`, `SymbolLookup`, `MemorySegment`, `Arena`.
- **Unnamed variables & patterns final** (JEP 456): `catch (Exception _)`, `case Point(var x, _)`.
- **Launch multi-file source-code programs** (JEP 458).
- **Region pinning for G1** (JEP 423) — JNI critical regions no longer stall GC.
- Previews: statements before `super(...)`, Class-File API, Stream Gatherers, implicitly declared classes.

### 3.16 Java 23 (Sept 2024)
- **Markdown documentation comments** (JEP 467): `///`.
- **ZGC: generational mode by default** (JEP 474).
- Deprecate `sun.misc.Unsafe` memory-access methods (JEP 471).
- Previews: primitive types in patterns (JEP 455), module import declarations, flexible constructor bodies, implicitly declared classes.
- String Templates **removed** from the preview pipeline.

### 3.17 Java 24 (March 2025) — 24 JEPs
- **Stream Gatherers final** (JEP 485): custom intermediate ops — `gather(Gatherers.windowFixed(3))`, `windowSliding`, `fold`, `scan`, `mapConcurrent`.
- **Class-File API final** (JEP 484) — replaces internal ASM copy.
- **Synchronize virtual threads without pinning** (JEP 491) — `synchronized` no longer pins carrier threads (biggest Loom pain point fixed).
- **Ahead-of-Time Class Loading & Linking** (JEP 483) — first Project Leyden deliverable; `-XX:AOTMode=record/create`, `-XX:AOTCache=app.aot`.
- **Compact Object Headers** experimental (JEP 450) — `-XX:+UnlockExperimentalVMOptions -XX:+UseCompactObjectHeaders`.
- G1 **late barrier expansion** (JEP 475) — faster C2 compilation.
- **ZGC non-generational mode removed** (JEP 490).
- Generational Shenandoah (experimental, JEP 404).
- **Security Manager permanently disabled** (JEP 486).
- **Post-quantum crypto**: ML-KEM (JEP 496), ML-DSA (JEP 497); KDF API (preview).
- Warn upon use of `Unsafe` memory access (JEP 498).
- Linking run-time images without JMODs (JEP 493) — ~25% smaller JDK.
- Removed Windows 32-bit x86 port; deprecated 32-bit x86.

### 3.18 Java 25 (LTS, Sept 2025) — 18 JEPs
- **Compact source files & instance main methods final** (JEP 512):
  ```java
  void main() { IO.println("Hello, Java 25!"); }
  ```
- **Module import declarations final** (JEP 511): `import module java.base;`.
- **Flexible constructor bodies final** (JEP 513) — statements before `super(...)`/`this(...)` (validate args, compute values).
- **Scoped values final** (JEP 506) — immutable, inheritable, bounded-lifetime alternative to `ThreadLocal`, ideal for virtual threads.
- **Compact Object Headers → product** (JEP 519) — `-XX:+UseCompactObjectHeaders` (no unlock needed).
- **Generational Shenandoah → product** (JEP 521).
- **AOT command-line ergonomics** (JEP 514): one-step `-XX:AOTCacheOutput=app.aot`.
- **AOT method profiling** (JEP 515) — profiles stored in AOT cache so JIT warms up faster.
- JFR: CPU-time profiling (experimental, JEP 509), cooperative sampling (JEP 518), method timing & tracing (JEP 520).
- **Key Derivation Function API final** (JEP 510).
- Previews: PEM encodings (JEP 470), Stable Values (JEP 502), primitive patterns (3rd), structured concurrency (5th); Vector API (10th incubator).
- **32-bit x86 port removed** (JEP 503).

### 3.19 Java 26 (March 2026) — 10 JEPs
- **HTTP/3 for HttpClient** (JEP 517) — QUIC-based, opt-in via `HttpClient.Version.HTTP_3`.
- **Prepare to Make Final Mean Final** (JEP 500) — warnings when deep reflection mutates `final` fields; future releases will deny by default.
- **Applet API removed** (JEP 504).
- **AOT Object Caching with any GC** (JEP 516) — AOT cache works with ZGC too (GC-agnostic object streaming).
- **G1: improved throughput by reducing synchronization** (JEP 522) — leaner write barriers / card-table handling.
- Previews/incubators: Lazy Constants 2nd preview (renamed from Stable Values, JEP 526), Structured Concurrency 6th (JEP 525), Primitive patterns 4th (JEP 530), PEM 2nd (JEP 524), Vector API 11th incubator (JEP 529).
- Non-JEP: JDBC 4.5 MR support; `HttpExchange` attributes no longer shared with context by default.

### 3.20 Java 27 (Sept 2026) — 9 JEPs (latest)

| JEP | Title | Status |
|---|---|---|
| 523 | **G1 as the Default GC Everywhere** — even on 1-CPU / low-memory machines that previously got Serial GC | Final |
| 527 | **Post-Quantum Hybrid Key Exchange for TLS 1.3** — ECDHE + ML-KEM combined automatically when peer supports it | Final |
| 534 | **Compact Object Headers by Default** — 8-byte headers on by default | Final |
| 536 | **JFR In-Process Data Redaction** — masks values matching `*password*`, `*token*` etc. before writing recordings | Final |
| 531 | Lazy Constants | 3rd Preview |
| 532 | Primitive Types in Patterns, `instanceof`, `switch` | 5th Preview |
| 533 | Structured Concurrency (joiners throw `ExecutionException`, `onTimeout()` → `timeout()`) | 7th Preview |
| 538 | PEM Encodings of Cryptographic Objects | Preview |
| 537 | Vector API | 12th Incubator |

**Practical impact of 27:** No source changes needed, but **re-measure heap sizing and container limits** — compact headers shrink heaps, and tiny containers now run G1 instead of Serial (slightly more native memory for G1 remembered sets, but better pause behavior).

---

## 4. JVM Internal Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                       .java  ──javac──►  .class (bytecode)       │
└───────────────────────────────┬─────────────────────────────────┘
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│ CLASS LOADER SUBSYSTEM                                           │
│  Bootstrap (C++) → Platform (9+, was Extension) → Application    │
│  Loading → Linking (Verify, Prepare, Resolve) → Initialization   │
│  + CDS / AOT Cache (pre-parsed, pre-linked classes mmapped)      │
└───────────────────────────────┬─────────────────────────────────┘
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│ RUNTIME DATA AREAS                                               │
│  Shared:   Heap (GC-managed) │ Metaspace (native) │ Code Cache   │
│  Per-thread: JVM Stack (frames) │ PC Register │ Native Stack     │
│  Virtual threads: stack chunks stored as heap objects            │
└───────────────────────────────┬─────────────────────────────────┘
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│ EXECUTION ENGINE                                                 │
│  Template Interpreter → C1 (client JIT) → C2 (server JIT)        │
│  Tiered compilation levels 0–4, deoptimization, OSR              │
│  Garbage Collector (Serial/Parallel/G1/ZGC/Shenandoah/Epsilon)   │
└───────────────────────────────┬─────────────────────────────────┘
                                ▼
              JNI / FFM API (22+) ──► Native libraries
```

### 4.1 Class Loading
- **Delegation model**: child asks parent first (prevents spoofing `java.lang.String`).
- Java 9 changed hierarchy: `Bootstrap → Platform → App` (Extension loader and `jre/lib/ext` removed; `rt.jar` replaced by `lib/modules` jimage).
- **Linking**: verification (bytecode verifier, type-checking with StackMapTable), preparation (static fields default values), resolution (symbolic → direct refs, lazily).
- **Initialization**: `<clinit>` runs once, thread-safe (basis of holder-idiom singletons).

### 4.2 Runtime Data Areas

| Area | Java 8+ | Notes |
|---|---|---|
| Heap | Young (Eden + 2 Survivors) + Old, or G1/ZGC regions | `-Xms`, `-Xmx`, `-XX:MaxRAMPercentage` |
| Metaspace | Native memory, class metadata | Replaced PermGen in 8; elastic in 16 |
| Compressed Class Space | Part of metaspace, 1 GB default | `-XX:CompressedClassSpaceSize` |
| Code Cache | JIT-compiled code; segmented since 9 | `-XX:ReservedCodeCacheSize` (240 MB w/ tiered) |
| Thread stacks | ~1 MB reserved each (platform threads, 64-bit Linux) | `-Xss`; virtual threads use growable heap chunks |
| String pool | In heap since Java 7 | `-XX:StringTableSize` |

### 4.3 Execution Engine & JIT

**Tiered compilation levels:**
| Level | Executor | Profiling |
|---|---|---|
| 0 | Interpreter | Full |
| 1 | C1 | None (trivial methods) |
| 2 | C1 | Invocation & backedge counters |
| 3 | C1 | Full profiling |
| 4 | C2 | Uses profile; aggressive optimizations |

Typical path: `0 → 3 → 4`. C2 optimizations: inlining (the "mother of all optimizations"), escape analysis + scalar replacement, loop unrolling, vectorization (SuperWord), lock elision/coarsening, null-check elimination, intrinsics (`System.arraycopy`, `Math`, CRC32, AES). Speculative optimizations are protected by **uncommon traps** → **deoptimization**.

**Graal**: Experimental JIT in 10–16 (removed from OpenJDK in 17); lives on in GraalVM and Native Image (closed-world AOT to native binary).

### 4.4 Object Model
- **Mark word** (64 bits): hash, age, lock state, GC bits.
- **Klass pointer**: 32 bits compressed (or 64).
- Compact Object Headers (JEP 450/519/534): mark word and klass pointer merged into **one 64-bit header** (22-bit class pointer), saving 4 bytes per object.
- **Compressed OOPs**: 32-bit references for heaps < ~32 GB (shift by 3 with 8-byte alignment). Crossing 32 GB loses this — a 33 GB heap may hold *less* than a 31 GB heap.

### 4.5 Safepoints & Handshakes
- **Safepoint**: global stop where all threads are at known states (GC roots, deopt). Polled at method returns & loop backedges.
- **Thread-local handshakes** (10): per-thread operations without global stop — used by ZGC, biased-lock revocation, stack sampling.
- **Time-to-safepoint (TTSP)** is a hidden source of latency (long counted loops).

---

## 5. Architectural Styles & Patterns Used Inside the JDK

| Component | Internal Style / Pattern | Explanation |
|---|---|---|
| **JVM overall** | Layered architecture + stack-based virtual machine | Bytecode is stack-based (operand stack), portable; HotSpot layers: runtime, interpreter, compilers, GC, OS abstraction |
| **Execution** | Adaptive optimization / mixed-mode | Interpret → profile → JIT hot spots (hence "HotSpot"); speculate & deoptimize |
| **Interpreter** | Template-based interpreter | Machine-code templates generated at startup per bytecode, not a C `switch` loop |
| **GC subsystem** | Strategy / plugin pattern (`CollectedHeap`, `BarrierSet` interfaces) | Each GC plugs in its own heap, barriers, and C1/C2 barrier code |
| **G1 / ZGC / Shenandoah heaps** | Region-based architecture | Heap divided into regions; collect garbage-first / incremental compaction |
| **ZGC** | Colored pointers + load barriers | Metadata bits in references; barriers "heal" pointers on load → concurrent relocation |
| **Shenandoah** | Load-reference barriers (formerly Brooks forwarding pointers) | Concurrent evacuation |
| **G1 marking** | SATB (Snapshot-At-The-Beginning) + card tables + remembered sets | Pre-write barrier logs old values; post-write barrier tracks cross-region refs |
| **Class loading** | Chain of Responsibility / parent delegation | Loader hierarchy |
| **Module system** | Microkernel + Service Locator (`ServiceLoader`, `provides/uses`) | Small `java.base` core, optional modules; SPI plugin architecture (JDBC drivers, charsets, `HttpClient` providers) |
| **Streams** | Pipes-and-filters, lazy evaluation, internal iteration | `Spliterator` source → chain of `Sink`s; fused, single pass; terminal op triggers |
| **Parallel streams / ForkJoinPool** | Divide-and-conquer + work-stealing | Per-worker deques; LIFO own, FIFO steal |
| **Virtual threads** | M:N scheduling, continuations | Many virtual threads mounted on few carrier threads (ForkJoinPool, FIFO mode); stack frames copied to/from heap on unmount/mount |
| **NIO** | Reactor pattern (Selector) | Event demultiplexing over epoll/kqueue/IOCP |
| **HttpClient** | Async, event-driven, `CompletableFuture`-based; reactive `Flow` for bodies | `BodyPublisher` / `BodySubscriber` |
| **j.u.c. locks** | Template Method (`AbstractQueuedSynchronizer`) | `ReentrantLock`, `Semaphore`, `CountDownLatch` built on AQS CLH queue |
| **Atomics / CHM** | Lock-free / CAS, striping (`LongAdder`) | Non-blocking algorithms |
| **CopyOnWriteArrayList** | Copy-on-write / immutable snapshot | Read-heavy workloads |
| **Lambdas, string concat, records `toString`** | `invokedynamic` + bootstrap methods | Linkage deferred to runtime; JDK can improve strategies without recompiling user code |
| **Records / sealed / patterns** | Algebraic Data Types + Data-Oriented Programming | Sum types (sealed) × product types (records) + exhaustive pattern matching |
| **JFR** | Event-driven, thread-local buffers, lock-free ring buffers | <1–2% overhead in default profile |
| **Project Leyden (AOT cache)** | Training run → snapshot ("condensers") | Shift work from runtime to an earlier phase |
| **Structured concurrency** | Hierarchical task scope (structured programming for threads) | Child tasks can't outlive scope; error/cancellation propagation |

---

## 6. Garbage Collection: Complete Guide

### 6.1 Fundamentals
- **Generational hypothesis**: most objects die young → separate young/old generations, collect young often.
- **Phases**: Mark (trace from GC roots: stacks, statics, JNI handles), Sweep, Compact/Copy.
- **Barriers**: code injected around reference reads/writes to keep GC invariants during concurrent work.
- **Key metrics**: Throughput (% time not in GC), Latency (pause times), Footprint (memory overhead). You can typically optimize two of three.

### 6.2 Collector Catalogue

| GC | Flag | Introduced / Status | Generational | Pauses | Best For |
|---|---|---|---|---|---|
| **Serial** | `-XX:+UseSerialGC` | Always | Yes | STW, single thread | Tiny heaps, single CPU, CLI tools |
| **Parallel** | `-XX:+UseParallelGC` | Default in ≤8 | Yes | STW, multi-thread | Batch / max throughput |
| **CMS** | `-XX:+UseConcMarkSweepGC` | Deprecated 9, **removed 14** | Yes | Mostly concurrent, no compaction → fragmentation | (legacy) |
| **G1** | `-XX:+UseG1GC` | Default 9–26 (server-class), **everywhere in 27** | Yes (regions) | Pause-target (default 200 ms) | General purpose, 4 GB–64 GB+ |
| **ZGC** | `-XX:+UseZGC` | Exp 11, prod 15, generational 21, gen-default 23, non-gen removed 24 | Yes (21+) | **< 1 ms**, independent of heap size | Low latency, huge heaps (up to 16 TB) |
| **Shenandoah** | `-XX:+UseShenandoahGC` | Exp 12, prod 15, generational product 25 | Opt (25+) | < 10 ms, concurrent compaction | Low latency (not in Oracle JDK builds) |
| **Epsilon** | `-XX:+UseEpsilonGC` | 11 (experimental) | — | None (never collects) | Perf testing, short-lived jobs |

### 6.3 Default GC by Version

| Version | Default GC |
|---|---|
| 8 | Parallel (server-class); Serial otherwise |
| 9 – 26 | G1 on server-class machines (≥ 2 CPUs and ≥ 1792 MB); Serial otherwise |
| 27+ | **G1 on all machines** |

> Containers with 1 CPU / < 1.8 GB RAM silently got **Serial GC** before 27 — a classic production surprise.

### 6.4 G1 (Garbage-First) Internals
- Heap split into **1–32 MB regions** (up to 512 MB in newer JDKs; ~2048 regions target); each region is Eden, Survivor, Old, or Humongous (objects ≥ ½ region).
- **Young GC** (STW, parallel evacuation), **Concurrent marking** (triggered at IHOP, default 45%, adaptive), **Mixed GC** (young + selected old regions with most garbage — hence "garbage first").
- **Remembered sets** per region track incoming references; **card table** marks dirty 512-byte cards.
- **SATB** pre-write barrier + post-write barrier.
- Full GC: fallback; parallel since 10.
- Evolution: parallel full GC (10), abortable mixed (12), uncommit memory (12), NUMA (14), region pinning (22), late barrier expansion (24), reduced synchronization (26), default everywhere (27).

```bash
-XX:+UseG1GC -XX:MaxGCPauseMillis=100 -XX:G1HeapRegionSize=16m
-XX:InitiatingHeapOccupancyPercent=40 -XX:G1ReservePercent=15
```

### 6.5 ZGC Internals
- **Colored pointers**: metadata bits in 64-bit references (marked/remapped states).
- **Load barriers**: on every reference load, check color; if stale, fix (relocate/remap) and self-heal the field.
- **Concurrent**: marking, relocation, reference processing, thread-stack scanning (16). STW phases are tiny and constant.
- **Generational ZGC (21)**: young & old generations, collected independently → much lower allocation-stall risk and CPU cost; uses store barriers too.
- Multi-mapped memory removed in generational mode → RSS reporting more accurate.
- Tuning is minimal: set `-Xmx`, optionally `-XX:SoftMaxHeapSize`, `-XX:ConcGCThreads`.

### 6.6 Shenandoah Internals
- Concurrent evacuation using **load-reference barriers** (since JDK 13; originally Brooks forwarding pointer word per object).
- Pause times independent of heap size; heuristics: `adaptive`, `static`, `compact`, `aggressive`.
- Generational mode: `-XX:ShenandoahGCMode=generational` (product in 25).

### 6.7 GC Choice Guide

| Workload | Recommendation |
|---|---|
| Microservice, 1–4 GB heap, typical latency | G1 (default) |
| Batch / ETL / Spark executors | Parallel GC |
| Latency-critical (trading, gaming, p99.9 SLAs) | Generational ZGC |
| Very large heaps (100 GB – TB) | ZGC |
| CLI tool / Lambda / tiny container | Serial (or G1 in 27 default), plus AOT cache |
| Benchmarking allocation cost | Epsilon |

### 6.8 GC Logging & Analysis
```bash
# Java 9+ unified logging
-Xlog:gc*,safepoint:file=gc.log:time,uptime,level,tags:filecount=5,filesize=20m

# Java 8 legacy
-XX:+PrintGCDetails -XX:+PrintGCDateStamps -Xloggc:gc.log
```
Tools: GCeasy, GCViewer, JDK Mission Control, `jstat -gcutil <pid> 1s`.

---

## 7. Memory Utilization & Layout

### 7.1 Memory Footprint Improvements Over Time

| Feature | Version | Saving |
|---|---|---|
| PermGen → Metaspace | 8 | Eliminates `OutOfMemoryError: PermGen space`; auto-sizing |
| Compact Strings | 9 | Latin-1 strings use 1 byte/char (≈ 50% less for string data; strings often 20–40% of heap) |
| String deduplication (G1) | 8u20 (G1), extended to all GCs in 18 | `-XX:+UseStringDeduplication` |
| Segmented code cache | 9 | Less fragmentation, better i-cache locality |
| Container awareness | 10 / 8u191 | Correct heap sizing inside Docker/K8s |
| G1 uncommit memory | 12 | Returns idle memory to OS |
| ZGC uncommit | 13 | Same for ZGC |
| Elastic Metaspace | 16 | Lower metaspace footprint, faster return to OS |
| Virtual threads | 21 | KB-scale per thread vs ~1 MB stack reservation for platform threads |
| Generational ZGC | 21 | Lower heap headroom needed vs non-gen ZGC |
| JDK image without JMODs | 24 | ~25% smaller JDK on disk |
| Compact Object Headers | 24 exp / 25 product / 27 default | 12 → 8 bytes per object; typically **10–20% less heap** for small-object-heavy apps, fewer GCs, better cache density |

### 7.2 Object Size Examples (64-bit, compressed oops)

| Object | Legacy header (12 B) | Compact header (8 B) |
|---|---|---|
| `new Object()` | 16 B (12 + 4 padding) | 8 B |
| `Integer` | 16 B | 16 B (header 8 + int 4 + 4 padding) |
| `Long` | 24 B | 16 B |
| `String` "hello" (shell only) | 24 B | 16–24 B (+ `byte[]` 16 + 5 → 24 B) |
| Empty `ArrayList` | 24 B | 16–24 B |

Use **JOL (Java Object Layout)** to measure: `ClassLayout.parseClass(Foo.class).toPrintable()`.

### 7.3 Native Memory Areas (what "RSS > Xmx" means)
Total process memory ≈ Heap + Metaspace + Code Cache + Thread stacks + GC structures (remembered sets, mark bitmaps) + Direct ByteBuffers + mmapped files + JIT compiler arenas + symbol tables + libc malloc.

```bash
-XX:NativeMemoryTracking=summary
jcmd <pid> VM.native_memory summary
```

### 7.4 Container Sizing Rules of Thumb
- Use `-XX:MaxRAMPercentage=70–80` instead of hard `-Xmx` in containers.
- Leave 20–30% for non-heap (more for many threads or heavy direct buffers / Netty).
- Re-measure after upgrading to 25 (compact headers opt-in) / 27 (compact headers + G1 everywhere).

### 7.5 Common OOM Types
| Error | Cause |
|---|---|
| `Java heap space` | Heap exhausted / leak |
| `GC overhead limit exceeded` | >98% time GC, <2% reclaimed (Parallel) |
| `Metaspace` | Class-loader leaks (redeploys, dynamic proxies) |
| `Direct buffer memory` | `-XX:MaxDirectMemorySize` hit |
| `unable to create native thread` | OS thread/ulimit/memory limit |
| `Requested array size exceeds VM limit` | Array > ~2³¹ elements |
| `Compressed class space` | Too many classes in compressed class space |

---

## 8. Performance Metrics, Startup & Footprint

> Exact numbers depend heavily on workload and hardware; figures below are typical directions reported by OpenJDK engineers and public benchmarks, not guarantees. Always benchmark your own app (JMH for micro, load tests for macro).

### 8.1 Throughput / Latency Trends
| Upgrade | Typical effect |
|---|---|
| 8 → 11 | G1 default (better pauses than Parallel for interactive apps, slightly lower raw throughput); compact strings reduce heap; overall often 5–15% faster |
| 11 → 17 | Large accumulation of GC and JIT improvements; G1 and Parallel both faster; commonly 5–20% |
| 17 → 21 | Generational ZGC, virtual threads for I/O-bound concurrency (orders-of-magnitude more concurrent tasks) |
| 21 → 25 | AOT cache (startup/warm-up), compact headers (heap), no pinning for `synchronized` |
| 25 → 27 | G1 throughput synchronization improvements (26), defaults on by default (27) |

### 8.2 Pause-Time Profiles
| GC | Typical p99 pause |
|---|---|
| Serial / Parallel | 100 ms – seconds (scales with heap/live set) |
| G1 | 10–200 ms (target-driven) |
| Shenandoah | 1–10 ms |
| ZGC (generational) | < 1 ms |

### 8.3 Startup & Warm-Up Technologies
| Tech | Version | What it caches |
|---|---|---|
| CDS (JDK classes) | 5 / default archive 12 | Parsed class metadata |
| AppCDS | 10 | Application classes |
| Dynamic CDS | 13 | Archive at exit |
| AOT class loading & linking | 24 | Loaded + linked classes (Leyden) |
| AOT command-line ergonomics | 25 | One-step training & cache creation |
| AOT method profiling | 25 | JIT profiles → faster peak performance |
| AOT object caching with any GC | 26 | Heap objects, ZGC compatible |
| GraalVM Native Image | external | Full native binary (ms startup, closed-world) |

```bash
# JDK 25+: train and create cache in one step
java -XX:AOTCacheOutput=app.aot -jar app.jar   # training run
java -XX:AOTCache=app.aot -jar app.jar          # production run
```

### 8.4 Key Metrics to Monitor in Production
- **GC**: pause time (p50/p99/max), GC frequency, % time in GC, allocation rate (MB/s), promotion rate, live set after full GC.
- **Memory**: heap used/committed, metaspace, code cache usage, RSS vs container limit, direct memory.
- **Threads**: count, states (BLOCKED / WAITING), virtual thread pinning events (`jdk.VirtualThreadPinned`).
- **JIT**: compilation time, code cache full warnings, deoptimization storms.
- **Safepoints**: time-to-safepoint, total safepoint time.
- **CPU**: user vs system, GC thread CPU.

Exposed via JMX (`java.lang:type=GarbageCollector`, `MemoryPoolMXBean`), JFR, Micrometer/Prometheus.

---

## 9. Concurrency Evolution

| Version | Feature |
|---|---|
| 5 | `java.util.concurrent`, `ExecutorService` |
| 7 | ForkJoinPool |
| 8 | `CompletableFuture`, parallel streams, `StampedLock`, `LongAdder`, CHM rewrite |
| 9 | Flow API, `VarHandle`, `CompletableFuture` timeouts (`orTimeout`, `completeOnTimeout`) |
| 15 | Biased locking disabled |
| 19–21 | **Virtual threads** (preview → final) |
| 21–27 | **Structured Concurrency** (preview series) |
| 20–25 | **Scoped Values** (incubator → final in 25) |
| 24 | `synchronized` no longer pins virtual threads |
| 24 | `Gatherers.mapConcurrent` |

### Virtual Threads: How They Work
- `Thread.ofVirtual().start(r)` / `Executors.newVirtualThreadPerTaskExecutor()`.
- Scheduled by a dedicated **ForkJoinPool** of carrier threads (parallelism = #cores).
- On blocking I/O / `LockSupport.park`, the continuation **unmounts**: stack frames copied to heap; carrier freed.
- **Pinning** (carrier blocked): pre-24 inside `synchronized`; still during native frames / `Object.wait` in some cases. Detect with `-Djdk.tracePinnedThreads` (removed later) or JFR `jdk.VirtualThreadPinned`.
- **Don't pool virtual threads**; use semaphores to limit concurrency to downstream resources.
- **Avoid heavy `ThreadLocal`** usage with millions of threads → use `ScopedValue`.
- Not faster for CPU-bound work; they increase **throughput for I/O-bound** workloads (Little's Law: concurrency = throughput × latency).

```java
// Structured concurrency (preview, JDK 27 shape)
try (var scope = StructuredTaskScope.open()) {
    Subtask<User>  user  = scope.fork(() -> findUser(id));
    Subtask<Order> order = scope.fork(() -> fetchOrder(id));
    scope.join();                       // throws if any fails; cancels the rest
    return new Response(user.get(), order.get());
}

// Scoped values (final in 25)
static final ScopedValue<User> CURRENT = ScopedValue.newInstance();
ScopedValue.where(CURRENT, user).run(() -> handle());
```

---

## 10. Observability & Tooling

| Tool | Since | Use |
|---|---|---|
| `jcmd` | 7 | Swiss-army knife: `GC.heap_info`, `Thread.print`, `VM.flags`, `JFR.start` |
| `jstat`, `jmap`, `jstack` | early | Legacy diagnostics |
| JFR + JMC | open-sourced 11 | Low-overhead profiling/events |
| JFR streaming | 14 | Continuous in-process consumption |
| JFR CPU-time profiling, method tracing | 25 | Accurate CPU profiles, method timing |
| JFR data redaction | 27 | Mask secrets in recordings |
| Unified logging `-Xlog` | 9 | GC, safepoint, class loading logs |
| Helpful NPEs | 14 (default on 15) | Precise null cause |
| `jshell` | 9 | REPL |
| `jlink`, `jpackage` | 9, 16 | Custom runtimes, native installers |
| `jwebserver` | 18 | Static file server |
| `jdeps`, `jdeprscan` | 8/9 | Dependency & deprecation analysis |
| Thread dumps with virtual threads | 21 | `jcmd <pid> Thread.dump_to_file -format=json` |

---

## 11. Removed / Deprecated Features

| Feature | Deprecated | Removed |
|---|---|---|
| PermGen | — | 8 |
| `jre/lib/ext`, `rt.jar`, endorsed dirs | — | 9 |
| Java EE modules (JAXB, JAX-WS), CORBA | 9 | 11 |
| JavaFX bundled | — | 11 |
| Nashorn | 11 | 15 |
| CMS GC | 9 | 14 |
| Pack200 | 11 | 14 |
| Biased locking | 15 | 18 (code removed) |
| RMI Activation | 15 | 17 |
| Experimental AOT (jaotc) & Graal JIT | — | 17 |
| `--illegal-access` | 16 | 17 |
| Security Manager | 17 | Permanently disabled 24 |
| Finalization | 18 (for removal) | pending |
| Non-generational ZGC | 23 | 24 |
| Windows 32-bit x86 | 21 | 24 |
| 32-bit x86 (all) | 24 | 25 |
| `sun.misc.Unsafe` memory access | 23 | warnings 24, removal pending |
| Applet API | 9 / 17 for removal | 26 |
| Thread.stop / suspend / resume | long ago | `stop` throws UOE since 20 |

---

## 12. Migration Guide

### 8 → 11 (the hardest jump)
- Add back Java EE deps: `jakarta.xml.bind-api`, `jaxb-runtime`, `jakarta.annotation-api`, `jakarta.activation`.
- Upgrade build tools & plugins (Maven compiler 3.8+, Surefire, Gradle 5+), Lombok, Mockito, ByteBuddy, ASM, Spring Boot 2.1+.
- Replace `sun.*` internal APIs (`sun.misc.BASE64Encoder` → `java.util.Base64`).
- GC logging flags changed → `-Xlog`.
- Default GC changes Parallel → G1: re-tune.
- ClassLoader casting to `URLClassLoader` breaks.

### 11 → 17
- Strong encapsulation: add `--add-opens` for reflective frameworks or upgrade them.
- Remove `--illegal-access`, CMS flags (JVM refuses to start with unknown removed options in some cases).
- Spring Boot 3 / Jakarta EE 9+ requires 17 and `javax.*` → `jakarta.*` rename.

### 17 → 21
- Mostly smooth. Adopt virtual threads carefully (pinning with `synchronized` pre-24, ThreadLocal usage, connection pool sizing).
- Dynamic agent loading warnings → add `-XX:+EnableDynamicAgentLoading` or load agents at startup.

### 21 → 25
- Security Manager gone (24) — remove `System.setSecurityManager`.
- `Unsafe` warnings — upgrade Netty, etc.
- 32-bit x86 builds gone.
- Try AOT cache and compact headers for startup and memory wins.

### 25 → 27 (non-LTS)
- Watch for reflection mutating `final` fields (JEP 500 warnings).
- Applet API removal.
- Re-validate memory sizing because compact headers and G1 are now defaults.

**Tools:** `jdeps --jdk-internals`, `jdeprscan`, OpenRewrite recipes, IDE inspections.

---

## 13. The OpenJDK Projects

| Project | Goal | Delivered |
|---|---|---|
| **Amber** | Productivity language features | `var`, switch expressions, text blocks, records, sealed, patterns, unnamed vars, compact source files, flexible constructors, module imports |
| **Loom** | Lightweight concurrency | Virtual threads (21), scoped values (25), structured concurrency (preview) |
| **Panama** | Native interop & SIMD | FFM API (22), Vector API (incubating, waiting on Valhalla), `jextract` |
| **Valhalla** | Value classes, "codes like a class, works like an int" | Expected starting JDK 28 (value classes/objects preview), later null-restricted types, specialized generics |
| **Leyden** | Startup, warm-up, footprint | AOT class loading (24), AOT profiles (25), AOT objects any GC (26) |
| **Lilliput** | Smaller object headers | Compact Object Headers (24 → 25 → 27 default); goal: 4-byte headers |
| **Babylon** | Code reflection for GPU/ML | In progress |
| **ZGC / Shenandoah** | Low latency GC | See §6 |

---

## 14. Interview Questions

### 14.1 Java 8 Fundamentals

1. What is a functional interface? Can it have default and static methods? *(Yes; exactly one abstract method, excluding `Object` methods.)*
2. How are lambdas implemented internally? Why not anonymous classes? *(`invokedynamic` + `LambdaMetafactory`; avoids one class file per lambda, lazy linking, allows JVM to optimize strategy later.)*
3. Difference between `map` and `flatMap` in streams and Optional.
4. Intermediate vs terminal operations; what is lazy evaluation and short-circuiting?
5. Why can a stream be consumed only once?
6. When should you **not** use parallel streams? *(Small data, blocking I/O, shared mutable state, `LinkedList` sources, common pool contention in servers.)*
7. `Collectors.groupingBy` vs `partitioningBy` vs `toMap` (merge function and duplicate keys).
8. `Optional` best practices — why not use it for fields or parameters?
9. Difference between `findFirst` and `findAny`.
10. What does "effectively final" mean and why is it required for lambda captures?
11. Diamond problem with default methods — how is it resolved? *(Class wins; more specific interface wins; else must override and call `X.super.m()`.)*
12. `CompletableFuture`: `thenApply` vs `thenCompose` vs `thenCombine`; `*Async` variants and which executor they use.
13. Why was PermGen replaced by Metaspace?
14. How does `ConcurrentHashMap` work in Java 8 vs Java 7?
15. How does `HashMap` handle collisions in Java 8? Treeify thresholds?
16. `java.time`: `Instant` vs `LocalDateTime` vs `ZonedDateTime` vs `OffsetDateTime`.
17. `StampedLock` optimistic reads — when useful?
18. `LongAdder` vs `AtomicLong`.

### 14.2 Java 9–17 Features

19. What problems does JPMS solve? `exports` vs `opens`; `requires transitive`; `requires static`.
20. What is an automatic module vs unnamed module?
21. Why does `List.of()` throw on `null` and on `add()`? Difference from `Collections.unmodifiableList` and `Arrays.asList`.
22. What are Compact Strings and how do they affect `String` memory?
23. What is `var`? Where can't it be used? Is Java dynamically typed now?
24. What is JShell useful for? What is jlink?
25. Explain the new HttpClient (sync/async, HTTP/2, HTTP/3 in 26).
26. Switch expressions vs switch statements; what does `yield` do?
27. Text blocks: how is incidental indentation stripped?
28. Records: what is generated? Can records have instance fields? Extend classes? Implement interfaces? Be mutable? *(Implicitly final, extend `Record`, no extra instance fields, can have static fields/methods, compact constructor.)*
29. Are records suitable as JPA entities? Why not?
30. Sealed classes: `permits`, `final`, `sealed`, `non-sealed` subclasses; why with records = algebraic data types?
31. What are Helpful NullPointerExceptions?
32. What are hidden classes? Who uses them?
33. What changed with strong encapsulation in 16/17? How do you fix `InaccessibleObjectException`?
34. Why was biased locking removed?
35. Why was the Security Manager deprecated and removed?
36. What changed with UTF-8 by default (18)? What could break?

### 14.3 Java 21–27 Features

37. What are virtual threads? How do they differ from platform threads and from reactive programming?
38. What is a carrier thread? What is pinning? What changed in Java 24?
39. Should you pool virtual threads? How do you limit concurrency to a database?
40. Why are `ThreadLocal`s problematic with virtual threads? What are Scoped Values?
41. Explain Structured Concurrency and its benefits over `ExecutorService` + `Future`.
42. Are virtual threads faster for CPU-bound tasks? Explain with Little's Law.
43. Pattern matching for switch: guards (`when`), `case null`, dominance, exhaustiveness.
44. Record patterns & nested deconstruction — give an example.
45. Sequenced Collections — what problem did they fix?
46. Unnamed variables `_` — where are they useful?
47. FFM API vs JNI: advantages, `Arena` lifetimes, `MemorySegment` safety.
48. What are Stream Gatherers? Implement a sliding window.
49. Compact source files & instance main methods — who benefits?
50. Flexible constructor bodies — what can and can't you do before `super()`?
51. Module import declarations — `import module java.base;` pros/cons.
52. What is Project Leyden's AOT cache? Training run vs production run.
53. What are Compact Object Headers? How much do they save and what are the trade-offs? *(4 bytes/object; limits on number of classes addressable; identity hash bits relocated.)*
54. What changed in Java 27 with default GC and why might your container memory change?
55. What is post-quantum hybrid key exchange in TLS 1.3 (JDK 27)? ML-KEM vs ML-DSA?
56. What does "Prepare to Make Final Mean Final" (JEP 500) affect? Which frameworks?
57. What are Lazy Constants (formerly Stable Values)? How do they compare to `final` fields and double-checked locking?
58. What is Project Valhalla trying to achieve? What is a value class?

### 14.4 JVM Internals & Architecture

59. Describe the JVM architecture: class loader subsystem, runtime data areas, execution engine.
60. Explain class loading phases and parent delegation. How can you break delegation and why would you (Tomcat, OSGi)?
61. What is the difference between `ClassNotFoundException` and `NoClassDefFoundError`?
62. What is stored on stack vs heap vs metaspace?
63. Explain tiered compilation and the C1/C2 compilers.
64. What is deoptimization? What is OSR?
65. What is escape analysis and scalar replacement? Does Java do stack allocation?
66. What is inlining and why is method size (`-XX:MaxInlineSize`, `FreqInlineSize`) important?
67. What is a safepoint? What is time-to-safepoint?
68. Explain the object header (mark word + klass pointer).
69. What are compressed oops and why does a 32 GB heap boundary matter?
70. What is `invokedynamic`? Name four JDK features built on it.
71. What is CDS / AppCDS and how does it reduce startup?
72. What's the difference between JIT and AOT (GraalVM Native Image vs Leyden)?
73. What is the Java Memory Model? Happens-before rules, `volatile`, `final` field semantics.
74. What internal architectural patterns does ForkJoinPool use? (work-stealing)
75. How is a Stream pipeline executed internally? (Spliterator, Sink chain, op flags)
76. What is AQS and which classes are built on it?

### 14.5 Garbage Collection

77. Explain the generational hypothesis. Why separate young and old generations?
78. Describe Eden, Survivor spaces, tenuring threshold, promotion.
79. Minor GC vs Major GC vs Full GC vs Mixed GC.
80. Compare Serial, Parallel, CMS, G1, ZGC, Shenandoah, Epsilon.
81. What was wrong with CMS? *(Fragmentation, concurrent mode failure, complexity.)*
82. How does G1 work? Regions, humongous objects, remembered sets, card tables, IHOP, mixed collections.
83. What is SATB marking?
84. How does ZGC achieve sub-millisecond pauses? Colored pointers and load barriers.
85. What is the benefit of generational ZGC over single-generation ZGC?
86. How does Shenandoah differ from ZGC?
87. What are read/write/load barriers?
88. What is an allocation stall? Concurrent mode failure? Evacuation failure / to-space exhausted?
89. What are TLABs (Thread-Local Allocation Buffers) and why is allocation in Java so cheap? *(Bump-the-pointer.)*
90. What is the default GC in Java 8, 11, 17, 21, 25, 27? When does the JVM pick Serial (pre-27)?
91. Which GC would you pick for: a batch job, a 200 GB in-memory cache, a 512 MB microservice, a trading engine?
92. How do you enable and read GC logs in Java 8 vs Java 11+?
93. How do you tune G1 for lower pauses? Why not set Young Gen size explicitly with G1?
94. What are soft, weak, phantom references? `Cleaner` vs `finalize()`.
95. Why was finalization deprecated?
96. What is String deduplication? How is it different from `String.intern()`?

### 14.6 Memory & Performance

97. Why is the process RSS larger than `-Xmx`? List non-heap memory consumers.
98. How do you diagnose a memory leak? (heap dump, MAT, dominator tree, GC roots, allocation profiling with JFR)
99. How do you diagnose high CPU? (thread dumps, `top -H`, async-profiler, JFR)
100. What causes `OutOfMemoryError: Metaspace`? How to fix classloader leaks?
101. How does the JVM behave inside Docker/Kubernetes? `MaxRAMPercentage`, `ActiveProcessorCount`.
102. What is Native Memory Tracking?
103. How do you measure object size? (JOL)
104. What is false sharing? `@Contended`?
105. What are JMH pitfalls (dead-code elimination, constant folding, warm-up)?
106. Which JVM metrics would you put on a production dashboard?
107. How would you reduce startup time for a serverless Java function? (CDS/AOT cache, CRaC, Native Image, smaller classpath, Serial GC, tiered stop at 1)
108. Explain throughput vs latency vs footprint trade-off in GC tuning.
109. What is the code cache and what happens when it is full?
110. How does `-XX:+AlwaysPreTouch` affect startup and latency?

### 14.7 Migration & Ecosystem

111. What are the main challenges migrating from Java 8 to 11/17?
112. Why does Spring Boot 3 require Java 17?
113. What is the difference between Oracle JDK and OpenJDK builds (Temurin, Corretto, Zulu)?
114. What is the LTS model and how often are LTS releases now?
115. What are preview features and incubator modules? How do you enable them?
116. Which tool finds usage of JDK internal APIs? (`jdeps --jdk-internals`)
117. Why might upgrading to 27 change your application's memory and GC behavior even with no code changes?

### 14.8 Coding / Scenario Questions

118. Rewrite a Java 8 visitor-pattern hierarchy using sealed interfaces + records + pattern-matching switch.
119. Use virtual threads to fetch 10,000 URLs concurrently with a concurrency limit of 200.
120. Implement a custom `Gatherer` that emits running averages.
121. Convert a `ThreadLocal`-based request context to `ScopedValue`.
122. Write a stream that groups employees by department and finds the top-paid in each.
123. Call C's `strlen` using the FFM API.
124. Given a GC log showing frequent Full GCs with G1, walk through your diagnosis.
125. A service in a 1 GB container gets OOM-killed by Kubernetes but never throws `OutOfMemoryError`. Why, and how do you fix it?
126. Your p99 latency spikes every few minutes on Java 17 with G1 and a 64 GB heap. What would you investigate? When would you switch to ZGC?
127. After moving to virtual threads, throughput didn't improve. List possible causes. *(CPU-bound, pinning, pool-limited downstream resources, `synchronized` on hot paths pre-24, blocking native calls.)*

### 14.9 Rapid-Fire (One-liners)

| Q | A |
|---|---|
| Version that introduced `var` | 10 |
| Version that removed CMS | 14 |
| First LTS with records | 17 (final in 16) |
| First LTS with virtual threads | 21 |
| When did G1 become default | 9 |
| When did G1 become default on *all* machines | 27 |
| When did ZGC become production-ready | 15 |
| Generational ZGC default | 23 |
| Security Manager permanently disabled | 24 |
| FFM API final | 22 |
| Stream Gatherers final | 24 |
| Scoped Values final | 25 |
| Compact object headers on by default | 27 |
| HTTP/3 in HttpClient | 26 |
| Default charset UTF-8 | 18 |
| `Stream.toList()` | 16 |
| `String.isBlank()`, `repeat()` | 11 |
| `List.of()` | 9 |
| Helpful NPE messages | 14 |
| Text blocks final | 15 |
| Sequenced collections | 21 |
| Markdown Javadoc | 23 |
| Instance `void main()` final | 25 |

---

*Prepared November 2026. Latest GA: JDK 27; latest LTS: JDK 25. Sources: OpenJDK JEP index (openjdk.org/jeps), Oracle Java release announcements, InfoQ and Baeldung release coverage.*
