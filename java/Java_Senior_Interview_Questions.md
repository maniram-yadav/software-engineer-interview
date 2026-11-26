# Senior Java Interview Question Bank (10+ Years Experience)

**Topics:** Core Java · Spring / Spring Boot · Annotations · Hibernate & ORM · AOP · Microservices · Distributed Service Management

At 10+ years, interviewers care less about definitions and more about **trade-offs, internals, failure modes, and production experience**. Each question below lists what a strong answer should cover. The final section is a set of scenario and system design questions.

> **How to use this file:** Each topic is split into *Core*, *Deep Dive*, and *Production / Scenario* questions. A senior candidate should handle all three levels.

---

## Table of Contents

1. [Core Java](#1-core-java)
2. [Java Concurrency & Multithreading](#2-java-concurrency--multithreading)
3. [JVM Internals, Memory & Performance](#3-jvm-internals-memory--performance)
4. [Modern Java (8 → 21+)](#4-modern-java-8--21)
5. [Spring Core](#5-spring-core)
6. [Spring Boot](#6-spring-boot)
7. [Annotations – Main Usages (Reference + Questions)](#7-annotations--main-usages)
8. [Spring Data, Transactions](#8-spring-data--transactions)
9. [Hibernate & ORM](#9-hibernate--orm)
10. [AOP (Aspect-Oriented Programming)](#10-aop-aspect-oriented-programming)
11. [Spring Security](#11-spring-security)
12. [Microservices Architecture](#12-microservices-architecture)
13. [Distributed Service Management](#13-distributed-service-management)
14. [Messaging & Event-Driven Systems](#14-messaging--event-driven-systems)
15. [Observability, Deployment & Cloud-Native](#15-observability-deployment--cloud-native)
16. [Scenario-Based & System Design Questions](#16-scenario-based--system-design-questions)
17. [Leadership & Behavioral (Senior-Level)](#17-leadership--behavioral-senior-level)

---

## 1. Core Java

### Core

**Q1. Explain the `equals()` and `hashCode()` contract. What breaks if you violate it?**
- Equal objects must have equal hash codes; unequal objects *may* collide.
- If you override `equals` but not `hashCode`, `HashMap`/`HashSet` lookups fail: the object is stored in one bucket and searched for in another.
- Mutable fields in `hashCode` are dangerous. If the key changes after insertion, the entry is "lost" in the map.
- For JPA entities, base equality on a business key or a stable ID, and handle the null ID before persistence.

**Q2. How does `HashMap` work internally? What changed in Java 8?**
- Array of buckets. Index = `(n - 1) & hash`, where the hash is spread with `h ^ (h >>> 16)`.
- Collisions form a linked list. **Java 8:** a bucket converts to a red-black tree when it holds 8+ nodes and capacity ≥ 64 (treeification), giving O(log n) worst case.
- Load factor 0.75. Resize doubles the capacity. Java 8 resize keeps node order (lo/hi split), which fixed the infinite-loop problem from concurrent resize in Java 7.
- Not thread-safe. Use `ConcurrentHashMap`.

**Q3. `ConcurrentHashMap` – how does it achieve thread safety? Java 7 vs Java 8.**
- Java 7: segment locking (16 segments by default).
- Java 8+: CAS for empty bins and `synchronized` on the first node of a bin. Lock-free reads (volatile).
- Null keys and values are not allowed (ambiguity between "absent" and "null").
- `size()` is an estimate under concurrency. Use `mappingCount()`.
- Atomic compound operations: `computeIfAbsent`, `merge`, `compute`.

**Q4. Difference between `Comparable` and `Comparator`. How do you build complex comparators in modern Java?**
- `Comparator.comparing(Employee::getDept).thenComparing(Employee::getSalary, reverseOrder()).thenComparing(nullsLast(...))`.

**Q5. Immutability: how do you design an immutable class? Why does it matter?**
- `final` class, `private final` fields, no setters, defensive copies of mutable inputs and outputs, no `this` escape in the constructor.
- Benefits: thread safety, safe caching, safe hash keys.
- Java 16+ `record`s give shallow immutability only (a `List` field is still mutable unless copied with `List.copyOf`).

**Q6. String pool, `intern()`, and why `String` is immutable.**
- Security (class loading, network paths), hashcode caching, pool sharing.
- Since Java 7 the pool lives on the heap. Java 9 compact strings (Latin-1 `byte[]`). G1 string deduplication.

**Q7. Checked vs unchecked exceptions – what is your stance in API design?**
- Senior answer: checked exceptions don't compose with lambdas and streams. Most modern frameworks (Spring) use unchecked exceptions. Translate exceptions at layer boundaries, and never swallow them.
- `try-with-resources`, suppressed exceptions, and `finally` vs `return` pitfalls.

### Deep Dive

**Q8. Explain the `ClassLoader` hierarchy and delegation model. When have you broken it?**
- Bootstrap → Platform (formerly Extension) → Application. Parent-first delegation.
- Child-first loading in servlet containers (Tomcat webapp classloader) and OSGi.
- Problems: `ClassCastException` for the same class loaded by two loaders, and metaspace leaks on redeploy (ThreadLocals and JDBC drivers holding a reference to the loader).

**Q9. Generics: type erasure, bounded wildcards, PECS.**
- `List<? extends Number>` → producer (read). `List<? super Integer>` → consumer (write).
- Erasure means you can't do `new T()` or `instanceof List<String>`. Bridge methods. Heap pollution and `@SafeVarargs`.

**Q10. Serialization pitfalls. Why is Java serialization considered dangerous?**
- Deserialization gadget-chain attacks (e.g., Apache Commons Collections).
- `serialVersionUID`, `transient`, `readResolve` for singletons.
- Prefer JSON, Protobuf, or Avro. Java 9+ has deserialization filters (`ObjectInputFilter`).

**Q11. How do you implement a thread-safe Singleton? Which approach is best?**
- Enum singleton (handles serialization and reflection), the initialization-on-demand holder idiom, or double-checked locking with `volatile`.
- Senior angle: in Spring, a singleton bean is usually what you want. Static singletons hurt testability.

**Q12. Explain `WeakReference`, `SoftReference`, and `PhantomReference` with real use cases.**
- `WeakHashMap` for metadata caches, soft references for memory-sensitive caches, phantom references plus `Cleaner` (replacement for `finalize()`).

---

## 2. Java Concurrency & Multithreading

### Core

**Q13. Explain the Java Memory Model (JMM) and *happens-before*.**
- Visibility, atomicity, and ordering. Rules: program order, monitor lock, volatile write→read, thread start/join, `final` field semantics.
- Instruction reordering by the compiler and CPU. Why double-checked locking without `volatile` is broken.

**Q14. `volatile` vs `synchronized` vs `AtomicXxx` vs `Lock`.**
- `volatile`: visibility and ordering, not atomicity of compound operations (`count++` is unsafe).
- `synchronized`: mutual exclusion plus visibility. Reentrant. Biased locking was removed in JDK 15+.
- `ReentrantLock`: tryLock, timeouts, interruptibility, fairness, multiple `Condition`s.
- `Atomic*`: CAS. `LongAdder` for high-contention counters.

**Q15. How does `ThreadPoolExecutor` work? Explain the core parameters and the rejection policy.**
- Order: core threads → queue → max threads → rejection.
- **Trap:** with an unbounded `LinkedBlockingQueue`, `maxPoolSize` is never used. Avoid `Executors.newFixedThreadPool` in production because the queue is unbounded and can cause an OOM.
- Rejection policies: Abort, CallerRuns (gives backpressure), Discard, DiscardOldest.
- Sizing: CPU-bound ≈ cores + 1. IO-bound ≈ cores × (1 + wait/compute).

**Q16. `CompletableFuture` – compose async calls, handle errors, and avoid pitfalls.**
- `thenApply` / `thenCompose` / `thenCombine` / `allOf` / `anyOf`. `exceptionally` / `handle` / `whenComplete`.
- Pitfall: the default `ForkJoinPool.commonPool()` is shared JVM-wide. Pass a dedicated executor for IO.
- Timeouts: `orTimeout`, `completeOnTimeout` (Java 9+).
- Context propagation (MDC, security context) is lost across threads.

**Q17. Deadlock, livelock, starvation – how do you detect and prevent them?**
- Detection: `jstack`, thread dumps, `ThreadMXBean.findDeadlockedThreads()`.
- Prevention: lock ordering, `tryLock` with timeout, smaller lock scope, immutable data.

### Deep Dive

**Q18. Explain `AbstractQueuedSynchronizer` (AQS). Which classes are built on it?**
- CLH queue variant, a state int, exclusive/shared modes.
- Used by `ReentrantLock`, `Semaphore`, `CountDownLatch`, `ReentrantReadWriteLock`, and `FutureTask` (historically).

**Q19. `CountDownLatch` vs `CyclicBarrier` vs `Phaser` vs `Semaphore`.**

**Q20. How does `ForkJoinPool` work-stealing operate? When do parallel streams hurt?**
- Deques per worker. Owners pop LIFO, thieves steal FIFO.
- Parallel streams hurt with blocking IO inside the pipeline, small datasets, `LinkedList` sources, ordered operations, and shared common-pool contention in web servers.

**Q21. Virtual Threads (Java 21, Project Loom) – how do they change server design?**
- Mounted on carrier threads. Blocking calls unmount the virtual thread. Millions of threads are feasible.
- **Pinning:** `synchronized` blocks and native calls pin the carrier (improved in JDK 24 for `synchronized`). Use `ReentrantLock` on older versions.
- Don't pool virtual threads. Limit concurrency with a `Semaphore` instead.
- Spring Boot 3.2+: `spring.threads.virtual.enabled=true`.
- ThreadLocal usage at scale → consider `ScopedValue`.
- Where reactive (WebFlux) still makes sense: streaming and backpressure.

**Q22. What is false sharing and how do you mitigate it?**
- Cache-line contention. `@Contended` (with `-XX:-RestrictContended`), padding, `LongAdder` cells.

### Production / Scenario

**Q23. Your service's CPU is at 100% and latency spikes. Walk through your diagnosis.**
- `top -H` → thread ID → hex → match in `jstack`. Or async-profiler / JFR flame graphs.
- Common culprits: GC thrashing, infinite loops, regex backtracking, lock contention spin, JSON serialization of huge payloads.

**Q24. A thread pool is exhausted and requests time out. Root causes?**
- Downstream slowness without timeouts, sync calls inside async pools, nested submission to the same pool (deadlock), and connection pool exhaustion masquerading as thread starvation.

---

## 3. JVM Internals, Memory & Performance

**Q25. Describe the JVM memory areas.**
- Heap (young: Eden/S0/S1, old), Metaspace (Java 8+, native memory), thread stacks, code cache, direct buffers.
- Container awareness: `-XX:MaxRAMPercentage` instead of a fixed `-Xmx` in Kubernetes.

**Q26. Compare GC algorithms: Parallel, CMS, G1, ZGC, Shenandoah.**
- G1: region-based, pause target (`MaxGCPauseMillis`), default since JDK 9.
- ZGC: sub-millisecond pauses, colored pointers, generational since JDK 21.
- CMS was removed in JDK 14.
- Selection: throughput (Parallel) vs latency (ZGC/Shenandoah) vs balanced (G1).

**Q27. How do you analyze a memory leak in production?**
- Symptoms: old gen growing after full GCs, `OutOfMemoryError: Java heap space`.
- `-XX:+HeapDumpOnOutOfMemoryError`, `jcmd <pid> GC.heap_dump`, Eclipse MAT dominator tree, retained size.
- Typical leaks: static collections, unbounded caches, ThreadLocals in pooled threads, listeners not deregistered, Hibernate first-level cache in long sessions.

**Q28. Types of `OutOfMemoryError` and what each means.**
- Java heap space, GC overhead limit exceeded, Metaspace, unable to create native thread, Direct buffer memory, Requested array size exceeds VM limit.

**Q29. JIT compilation: C1/C2, tiered compilation, escape analysis, inlining. What is GraalVM native image and its trade-offs?**
- Native image: fast startup and low memory, but closed-world assumption (reflection config needed), lower peak throughput without PGO. Spring Boot 3 AOT support.

**Q30. How do you benchmark Java code correctly?**
- JMH. Warmup, dead-code elimination, constant folding, `Blackhole`. Why `System.nanoTime()` loops lie.

---

## 4. Modern Java (8 → 21+)

**Q31. What key features did you adopt from Java 8, 11, 17, and 21 in production?**
- 8: lambdas, streams, `Optional`, `java.time`, `CompletableFuture`.
- 11: `var` (10), `HttpClient`, String methods, single-file launch.
- 17: records, sealed classes, pattern matching for `instanceof`, text blocks, switch expressions.
- 21: virtual threads, pattern matching for `switch`, record patterns, sequenced collections.

**Q32. Streams: intermediate vs terminal operations, laziness, short-circuiting. When are streams a bad idea?**
- Debuggability, checked exceptions, hot loops, side effects in `map`/`peek`.
- `Collectors.groupingBy`, `partitioningBy`, `toMap` with a merge function (duplicate key `IllegalStateException`).

**Q33. `Optional` – correct and incorrect usage.**
- Return type only. Not for fields, parameters, or collections. Avoid `get()`. Prefer `orElseThrow`, `map`, `orElseGet` (lazy) over `orElse` (eager).

**Q34. Sealed classes + records + pattern matching: how do they enable algebraic data types? Give an example.**
```java
sealed interface PaymentResult permits Success, Declined, Error {}
record Success(String txId) implements PaymentResult {}
record Declined(String reason) implements PaymentResult {}
record Error(Exception e) implements PaymentResult {}

String msg = switch (result) {
    case Success s   -> "OK " + s.txId();
    case Declined d  -> "Declined: " + d.reason();
    case Error e     -> "Error";
}; // exhaustive, no default needed
```

**Q35. What is the Java Platform Module System (JPMS)? Did you adopt it? Why or why not?**
- Strong encapsulation, `module-info.java`, `exports`/`opens` (reflection for frameworks). Most Spring apps stay on the classpath, and that is a perfectly valid answer if justified.

---

## 5. Spring Core

### Core

**Q36. Explain IoC and Dependency Injection. Constructor vs setter vs field injection – which do you prefer and why?**
- Constructor injection: immutability, mandatory dependencies, testability without reflection, cycle detection at startup.
- Field injection hides dependencies and needs reflection to test.
- Single constructor → `@Autowired` is optional (Spring 4.3+).

**Q37. Describe the Spring bean lifecycle in detail.**
1. Bean definition loading (`BeanDefinitionRegistryPostProcessor`, `BeanFactoryPostProcessor`)
2. Instantiation
3. Populate properties (DI)
4. `*Aware` callbacks (`BeanNameAware`, `ApplicationContextAware`…)
5. `BeanPostProcessor.postProcessBeforeInitialization`
6. `@PostConstruct` → `InitializingBean.afterPropertiesSet` → custom `init-method`
7. `BeanPostProcessor.postProcessAfterInitialization` (**AOP proxies are created here**)
8. Bean ready
9. Destruction: `@PreDestroy` → `DisposableBean.destroy` → `destroy-method`

**Q38. Bean scopes. What happens when you inject a prototype bean into a singleton?**
- singleton, prototype, request, session, application, websocket.
- The prototype is injected once, so it effectively becomes a singleton. Fixes: `ObjectProvider<T>`, `@Lookup` method injection, scoped proxy (`proxyMode = ScopedProxyMode.TARGET_CLASS`).

**Q39. `BeanFactory` vs `ApplicationContext`.**
- ApplicationContext adds eager singleton init, events, i18n, environment/profiles, AOP auto-proxying, resource loading.

**Q40. How does Spring resolve circular dependencies? Why did Spring Boot 2.6 disable them by default?**
- Three-level cache (singletonObjects, earlySingletonObjects, singletonFactories) for setter/field injection of singletons. Constructor injection cycles cannot be resolved.
- Boot 2.6+ fails fast (`spring.main.allow-circular-references=false`) because cycles indicate a design smell.
- Fixes: redesign, `@Lazy`, events, extract a third component.

### Deep Dive

**Q41. `BeanPostProcessor` vs `BeanFactoryPostProcessor`. Name real framework uses.**
- BFPP modifies *definitions* (e.g., `PropertySourcesPlaceholderConfigurer`, `ConfigurationClassPostProcessor`).
- BPP modifies *instances* (e.g., `AutowiredAnnotationBeanPostProcessor`, `AnnotationAwareAspectJAutoProxyCreator`).

**Q42. `@Configuration` vs `@Component` with `@Bean` methods (full vs lite mode).**
- `@Configuration` classes are CGLIB-proxied so inter-`@Bean` method calls return the singleton. In lite mode (`@Component` or `proxyBeanMethods=false`) every call creates a new instance.
- `proxyBeanMethods=false` is used in Boot auto-configs for startup speed and native images.

**Q43. How do Spring events work? Sync vs async, transactional events.**
- `ApplicationEventPublisher`, `@EventListener`, `@Async` listeners.
- `@TransactionalEventListener(phase = AFTER_COMMIT)` publishes only if the transaction commits. Basis for a simple outbox-style pattern (but not durable, which is why the real Outbox pattern exists).

**Q44. How do profiles and the `Environment` abstraction work? Property resolution order?**

**Q45. JDK dynamic proxy vs CGLIB proxy – when does Spring use each?**
- JDK proxies need an interface. CGLIB subclasses (can't proxy `final` classes/methods).
- Spring Boot defaults to CGLIB (`spring.aop.proxy-target-class=true`).

---

## 6. Spring Boot

**Q46. How does auto-configuration work internally?**
- `@SpringBootApplication` = `@Configuration` + `@EnableAutoConfiguration` + `@ComponentScan`.
- `AutoConfigurationImportSelector` reads `META-INF/spring/org.springframework.boot.autoconfigure.AutoConfiguration.imports` (Boot 2.7+/3; formerly `spring.factories`).
- Conditionals: `@ConditionalOnClass`, `@ConditionalOnMissingBean`, `@ConditionalOnProperty`, `@ConditionalOnWebApplication`…
- Debug with `--debug` or the `/actuator/conditions` endpoint.

**Q47. How would you write a custom Spring Boot starter?**
- An `autoconfigure` module plus a `starter` module (dependencies only). `@AutoConfiguration`, conditionals, `@ConfigurationProperties` with metadata, back-off with `@ConditionalOnMissingBean`.

**Q48. Externalized configuration precedence order.**
- Command-line args > `SPRING_APPLICATION_JSON` > OS env vars > profile-specific `application-{profile}.yml` > `application.yml` > `@PropertySource` > defaults.
- `@ConfigurationProperties` (type-safe, validated with `@Validated`) vs `@Value`.

**Q49. What does Spring Boot Actuator provide? How do you secure it?**
- health (liveness/readiness groups for K8s), metrics (Micrometer), info, env, loggers (change log level at runtime), threaddump, heapdump, prometheus.
- Expose only what you need. Use a separate management port and secure it.

**Q50. Spring Boot 2 → 3 migration challenges.**
- Java 17 baseline, `javax.*` → `jakarta.*`, Hibernate 6, Spring Security 6 (`SecurityFilterChain`, no `WebSecurityConfigurerAdapter`), Micrometer Observation / Tracing replacing Sleuth, auto-config file changes, AOT/native support.

**Q51. How do you improve Spring Boot startup time and memory footprint?**
- Lazy init, trimming auto-config, `proxyBeanMethods=false`, CDS/AppCDS, CRaC, GraalVM native, virtual threads, profiling with `ApplicationStartup` / `BufferingApplicationStartup`.

**Q52. Embedded server internals – Tomcat thread model vs Netty (WebFlux). When to choose WebFlux?**
- Thread-per-request vs event loop. WebFlux is for high-concurrency IO, streaming, and backpressure. The cost is complexity and debugging difficulty, and blocking calls on the event loop are deadly. Virtual threads reduce WebFlux's advantage for typical CRUD.

---

## 7. Annotations – Main Usages

### 7.1 Quick Reference Table

| Category | Annotation | Main Usage |
|---|---|---|
| **Stereotype** | `@Component` | Generic Spring-managed bean |
| | `@Service` | Business logic layer (semantic) |
| | `@Repository` | DAO layer; enables **exception translation** to `DataAccessException` |
| | `@Controller` | MVC controller returning views |
| | `@RestController` | `@Controller` + `@ResponseBody` |
| | `@Configuration` | Java config class with `@Bean` methods (CGLIB-proxied) |
| **DI** | `@Autowired` | Inject by type |
| | `@Qualifier` | Disambiguate between multiple beans of same type |
| | `@Primary` | Default bean when multiple candidates exist |
| | `@Value` | Inject property / SpEL expression |
| | `@Lazy` | Delay bean creation; break cycles |
| | `@Scope` | Bean scope (prototype, request…) |
| | `@Bean` | Declare bean from a factory method |
| | `@Import` | Import other configuration classes |
| | `@Profile` | Activate bean only for given profile |
| | `@Conditional*` | Conditional registration (Boot auto-config) |
| | `@DependsOn` | Force init order |
| **Lifecycle** | `@PostConstruct` / `@PreDestroy` | Init / cleanup callbacks (jakarta.annotation) |
| **Boot** | `@SpringBootApplication` | `@Configuration` + `@EnableAutoConfiguration` + `@ComponentScan` |
| | `@ConfigurationProperties` | Bind type-safe config hierarchy |
| | `@EnableConfigurationProperties` / `@ConfigurationPropertiesScan` | Register properties classes |
| | `@AutoConfiguration` | Mark auto-config class (Boot 2.7+) |
| **Web** | `@RequestMapping`, `@GetMapping`, `@PostMapping`, `@PutMapping`, `@DeleteMapping`, `@PatchMapping` | Route mapping |
| | `@PathVariable`, `@RequestParam`, `@RequestBody`, `@RequestHeader`, `@CookieValue` | Bind request data |
| | `@ResponseStatus` | Set HTTP status |
| | `@ExceptionHandler` | Handle exceptions in a controller |
| | `@ControllerAdvice` / `@RestControllerAdvice` | Global exception handling, binders |
| | `@CrossOrigin` | CORS config |
| | `@Valid` / `@Validated` | Bean validation (`@Validated` supports groups & method validation) |
| **Transactions** | `@Transactional` | Declarative transactions (propagation, isolation, rollbackFor, readOnly, timeout) |
| | `@EnableTransactionManagement` | Enable (auto in Boot) |
| **Async / Scheduling** | `@Async` + `@EnableAsync` | Run method in executor |
| | `@Scheduled` + `@EnableScheduling` | Cron / fixedRate / fixedDelay jobs |
| **Caching** | `@Cacheable`, `@CachePut`, `@CacheEvict`, `@Caching`, `@EnableCaching` | Declarative caching |
| **AOP** | `@Aspect`, `@Before`, `@After`, `@AfterReturning`, `@AfterThrowing`, `@Around`, `@Pointcut`, `@EnableAspectJAutoProxy`, `@Order` | Cross-cutting concerns |
| **JPA** | `@Entity`, `@Table`, `@Id`, `@GeneratedValue`, `@Column`, `@Version` | Mapping & optimistic locking |
| | `@OneToOne`, `@OneToMany`, `@ManyToOne`, `@ManyToMany`, `@JoinColumn`, `@JoinTable` | Relationships |
| | `@Embedded` / `@Embeddable`, `@MappedSuperclass`, `@Inheritance` | Composition & inheritance |
| | `@NamedQuery`, `@NamedEntityGraph`, `@EntityGraph` | Queries & fetch plans |
| | `@Query`, `@Modifying`, `@Lock` | Spring Data repository methods |
| | `@CreatedDate`, `@LastModifiedDate`, `@CreatedBy` + `@EnableJpaAuditing` | Auditing |
| **Testing** | `@SpringBootTest`, `@WebMvcTest`, `@DataJpaTest`, `@MockitoBean` (Boot 3.4+, replaces `@MockBean`), `@TestConfiguration`, `@DirtiesContext`, `@Sql`, `@ServiceConnection` (Testcontainers) | Test slices & context |
| **Security** | `@EnableWebSecurity`, `@EnableMethodSecurity`, `@PreAuthorize`, `@PostAuthorize`, `@Secured` | Method & web security |
| **Cloud** | `@FeignClient` / `@HttpExchange`, `@LoadBalanced`, `@RefreshScope`, `@CircuitBreaker`, `@Retry`, `@RateLimiter` (Resilience4j) | Microservice concerns |
| **Java built-in** | `@Override`, `@Deprecated(forRemoval)`, `@FunctionalInterface`, `@SuppressWarnings`, `@SafeVarargs` | Compiler hints |
| **Meta** | `@Retention`, `@Target`, `@Inherited`, `@Documented`, `@Repeatable` | Define custom annotations |

### 7.2 Annotation Interview Questions

**Q53. How are annotations processed? Compile-time vs runtime.**
- Retention: `SOURCE` (Lombok, `@Override`), `CLASS` (default, in bytecode but not reflective), `RUNTIME` (reflection; Spring uses these).
- Compile-time processing: `javax.annotation.processing.Processor` (Lombok, MapStruct, Dagger, Hibernate JPA metamodel).
- Runtime: reflection plus Spring's `AnnotationUtils` / `MergedAnnotations` (supports meta-annotations and `@AliasFor`).

**Q54. Create a custom annotation that logs execution time of a method.**
```java
@Target(ElementType.METHOD)
@Retention(RetentionPolicy.RUNTIME)
public @interface LogExecutionTime {}

@Aspect @Component
public class TimingAspect {
    @Around("@annotation(LogExecutionTime)")
    public Object time(ProceedingJoinPoint pjp) throws Throwable {
        long start = System.nanoTime();
        try { return pjp.proceed(); }
        finally {
            log.info("{} took {} ms", pjp.getSignature(),
                     (System.nanoTime() - start) / 1_000_000);
        }
    }
}
```
- Follow-up: why doesn't it work when called from the same class? (Self-invocation bypasses the proxy; see AOP.)

**Q55. What is a composed / meta-annotation? How does `@AliasFor` work?**
- `@RestController` is meta-annotated with `@Controller` and `@ResponseBody`. `@GetMapping` aliases `@RequestMapping(method=GET)`.
- Build your own, e.g. `@TransactionalService` = `@Service` + `@Transactional`.

**Q56. `@Component` vs `@Bean` – when to use which?**
- `@Component` for your own classes (scanned). `@Bean` for third-party classes, conditional creation, or complex construction.

**Q57. Why does `@Repository` exist if `@Component` works?**
- `PersistenceExceptionTranslationPostProcessor` translates vendor exceptions (e.g., `SQLException`, Hibernate exceptions) into Spring's `DataAccessException` hierarchy.

**Q58. `@Valid` vs `@Validated`.**
- `@Valid` (Jakarta): cascades validation. `@Validated` (Spring): supports validation groups and class-level method validation (on services, via an AOP proxy).

**Q59. List 5 reasons `@Transactional` may silently not work.**
1. Self-invocation (internal method call bypasses the proxy)
2. Method is `private` (or `final`/`static`). Spring 6 supports protected/package-private on CGLIB proxies, but not private.
3. Checked exception thrown: no rollback by default (use `rollbackFor`)
4. Exception caught and swallowed inside the method
5. Bean not Spring-managed (created with `new`)
6. Wrong transaction manager with multiple data sources
7. Async thread: the transaction is thread-bound and doesn't propagate

**Q60. `@Async` pitfalls.**
- Self-invocation, default executor (`SimpleAsyncTaskExecutor` creates a thread per task in older versions; Boot configures `ThreadPoolTaskExecutor`), exceptions lost on `void` methods (use `AsyncUncaughtExceptionHandler`), lost MDC/security context (use a `TaskDecorator`), no transaction propagation.

**Q61. `@Cacheable` pitfalls.**
- Self-invocation, key generation with complex params, caching `null`, cache stampede, stale data, `@CacheEvict(beforeInvocation)`, and distributed cache consistency (Redis).

**Q62. `@Scheduled` in a cluster – how do you prevent multiple instances from running the same job?**
- ShedLock, Quartz in clustered JDBC mode, a K8s CronJob, or leader election.

---

## 8. Spring Data & Transactions

**Q63. Explain all transaction propagation levels with use cases.**

| Propagation | Behavior | Use case |
|---|---|---|
| `REQUIRED` (default) | Join existing or create new | Most service methods |
| `REQUIRES_NEW` | Suspend existing, always new | Audit logs that must persist even if outer fails |
| `NESTED` | Savepoint within existing (JDBC only) | Partial rollback of a batch step |
| `SUPPORTS` | Join if exists, else non-transactional | Read-only helpers |
| `NOT_SUPPORTED` | Suspend existing, run non-tx | Long-running non-DB operations |
| `MANDATORY` | Must have existing, else exception | Enforce caller transaction |
| `NEVER` | Must NOT have existing | Guard |

- Trap: "UnexpectedRollbackException: Transaction silently rolled back because it has been marked as rollback-only." An inner `REQUIRED` method threw, the outer caught the exception, and the outer then tried to commit.

**Q64. Isolation levels and the anomalies they prevent.**
- READ_UNCOMMITTED (dirty reads), READ_COMMITTED (non-repeatable reads possible), REPEATABLE_READ (phantoms possible in theory; MySQL InnoDB prevents most via gap locks), SERIALIZABLE.
- Defaults: PostgreSQL/Oracle = READ_COMMITTED, MySQL InnoDB = REPEATABLE_READ.
- Lost updates → optimistic (`@Version`) or pessimistic locking.

**Q65. What does `readOnly = true` actually do?**
- Hibernate sets `FlushMode.MANUAL` and skips dirty checking (Hibernate 5.1+/6 can also skip snapshot creation). The JDBC connection is set read-only (some drivers/DBs optimize). Can route to read replicas with `AbstractRoutingDataSource`.

**Q66. Spring Data JPA: derived queries, `@Query`, Specifications, Projections, QueryDSL – when to use which?**
- Interface vs class (DTO) projections avoid loading entities. `Specification` for dynamic filters. Pagination with `Pageable`: `Page` runs a count query and `Slice` doesn't. Use keyset pagination for deep pages.

**Q67. How do you manage multiple data sources in Spring Boot?**
- Separate `DataSource`, `EntityManagerFactory`, and `TransactionManager` beans. `@Primary`. `@EnableJpaRepositories(basePackages, entityManagerFactoryRef, transactionManagerRef)`. Chained or distributed transactions are generally avoided in favor of sagas.

**Q68. Programmatic vs declarative transactions. When would you use `TransactionTemplate`?**
- Fine-grained control: a loop committing in chunks, avoiding self-invocation issues, or a transaction around a lambda.

---

## 9. Hibernate & ORM

### Core

**Q69. What problems does ORM solve, and what is the object-relational impedance mismatch?**
- Granularity, inheritance, identity (`==` vs PK), associations (bidirectional references vs FKs), navigation (graph vs joins).

**Q70. JPA vs Hibernate vs Spring Data JPA.**
- JPA (Jakarta Persistence) = specification. Hibernate = implementation (plus extras). Spring Data JPA = repository abstraction on top of JPA.

**Q71. Explain entity states and transitions.**
- Transient → (`persist`) → Managed → (`detach` / close context) → Detached → (`merge`) → Managed → (`remove`) → Removed.
- `persist` vs `merge` vs `save` vs `update` vs `saveOrUpdate` (Hibernate legacy, deprecated in 6).
- `merge` returns a *new* managed copy. The argument stays detached.

**Q72. First-level vs second-level vs query cache.**
- L1: per `Session`/`EntityManager`, always on, guarantees identity within the persistence context.
- L2: per `SessionFactory`, optional (Ehcache, Hazelcast, Infinispan). Concurrency strategies: READ_ONLY, NONSTRICT_READ_WRITE, READ_WRITE, TRANSACTIONAL.
- Query cache: stores IDs only and depends on L2. It is invalidated whenever any table in the query changes, so it's often counterproductive.

**Q73. Lazy vs eager loading. What are the JPA defaults?**
- `@ManyToOne` / `@OneToOne` → EAGER by default (set them to LAZY explicitly!). `@OneToMany` / `@ManyToMany` → LAZY.
- `LazyInitializationException` and why `spring.jpa.open-in-view=true` (the Boot default, which logs a warning) is an anti-pattern: it holds DB connections during view rendering and causes hidden N+1 queries.

### Deep Dive

**Q74. What is the N+1 problem? Give every solution you know.**
- One query for parents plus N queries for children.
- Solutions: `JOIN FETCH` in JPQL, `@EntityGraph`, `@BatchSize` / `hibernate.default_batch_fetch_size`, `@Fetch(FetchMode.SUBSELECT)`, DTO projections.
- Detection: Hibernate statistics, `datasource-proxy`, Hypersistence Utils `SQLStatementCountValidator` in tests.
- Pitfall: `JOIN FETCH` on multiple bags → `MultipleBagFetchException`. Fetching a collection with pagination → in-memory pagination warning (HHH000104 / HHH90003004). Fix: two-step query (IDs first, then fetch).

**Q75. How does dirty checking work? Performance implications?**
- On flush, Hibernate compares current state to the loaded snapshot for every managed entity. Large persistence contexts make flushes slow.
- Mitigation: read-only queries/transactions, `StatelessSession`, `clear()` in batches, bytecode enhancement for dirty tracking.

**Q76. Flush modes and when Hibernate flushes.**
- AUTO (before queries touching affected tables, and at commit), COMMIT, ALWAYS, MANUAL.
- Ordering of SQL during flush: inserts → updates → collection deletions → collection inserts → deletes. This explains "unique constraint violation" surprises.

**Q77. ID generation strategies and their impact on batching.**
- `IDENTITY` disables JDBC insert batching (needs the ID immediately).
- `SEQUENCE` with `allocationSize` (pooled/pooled-lo optimizers) is the preferred option.
- `TABLE` is slow and contended. UUIDs: v4 random hurts B-tree index locality, so prefer time-ordered UUIDv7.

**Q78. How do you do efficient batch inserts/updates with Hibernate?**
- `hibernate.jdbc.batch_size`, `order_inserts`, `order_updates`, SEQUENCE IDs, periodic `flush()` + `clear()`, `reWriteBatchedInserts=true` (PostgreSQL JDBC). For huge loads, use JDBC `batchUpdate` or `COPY`.

**Q79. Optimistic vs pessimistic locking.**
- `@Version` → `OptimisticLockException` on concurrent modification. Retry on conflict.
- Pessimistic: `@Lock(PESSIMISTIC_WRITE)` → `SELECT ... FOR UPDATE`. Lock timeouts, `SKIP LOCKED` for job queues.
- Choose by conflict rate: low contention → optimistic.

**Q80. Bidirectional relationships: owning side, `mappedBy`, and synchronization helpers.**
- The owning side (the side holding the FK, usually `@ManyToOne`) controls persistence. Keep both sides in sync with `addChild()`/`removeChild()` helpers.
- Unidirectional `@OneToMany` without `@JoinColumn` creates a join table and inefficient SQL.
- `@ManyToMany` with `List` → deletes all rows and reinserts. Use `Set` or model the join entity explicitly.

**Q81. Cascade types vs `orphanRemoval`.**
- `CascadeType.REMOVE` deletes children when the parent is deleted. `orphanRemoval` deletes a child when it is removed from the collection.
- Never cascade from child to parent (`@ManyToOne(cascade=ALL)` is a bug).

**Q82. Inheritance mapping strategies and trade-offs.**
- SINGLE_TABLE (fast, nullable columns, discriminator), JOINED (normalized, joins on polymorphic queries), TABLE_PER_CLASS (UNION queries, no IDENTITY), `@MappedSuperclass` (no polymorphic queries, just shared fields).

**Q83. `equals`/`hashCode` for JPA entities – what's the correct approach?**
- Use a natural/business key if one exists. Otherwise use the ID with null-safety and a constant `hashCode()` (e.g., `getClass().hashCode()`) so the entity remains in a `Set` across the persist transition. Never use Lombok `@Data` on entities (it touches lazy associations, causes recursion in toString/hashCode, and is mutable).

**Q84. What changed in Hibernate 6?**
- New SQM query engine, `jakarta.persistence`, improved type system (`@JdbcTypeCode`, JSON mapping), deprecated legacy Criteria and `save/update`, better SQL, and the default `IDENTITY`-to-`SEQUENCE` handling changes. In 6.x, `hibernate.id.new_generator_mappings` was removed.

**Q85. When would you NOT use an ORM?**
- Heavy reporting/analytics, bulk ETL, complex vendor-specific SQL, and extreme latency paths. Alternatives: jOOQ, MyBatis, JdbcTemplate/`JdbcClient` (Spring 6.1), R2DBC for reactive.

### Production / Scenario

**Q86. A page loads in 8 seconds. The SQL log shows 1,200 queries. Walk through your fix.**
- Identify the N+1 association, add fetch joins or entity graphs, or switch to DTO projections. Add a query-count assertion test to prevent regression.

**Q87. You get `OptimisticLockException` frequently under load. What do you do?**
- Analyze the hot rows. Use retry with backoff (Spring Retry), reduce transaction scope, split aggregates, use atomic SQL updates (`UPDATE ... SET qty = qty - 1 WHERE qty > 0`), or apply event sourcing for high-contention counters.

**Q88. Connection pool (HikariCP) exhaustion – causes and tuning.**
- Long transactions, open-in-view, remote calls inside transactions, leaks. `leakDetectionThreshold`, `maximumPoolSize` (small: ~cores × 2 + spindles), `connectionTimeout`. Monitor `hikaricp_connections_pending`.

---

## 10. AOP (Aspect-Oriented Programming)

### Core

**Q89. Define the AOP terminology.**
- **Aspect**: module of a cross-cutting concern. **Join point**: execution point (in Spring, always method execution). **Pointcut**: predicate selecting join points. **Advice**: action (before, after, around…). **Target**: the advised object. **Proxy**: the wrapper. **Weaving**: linking aspects with targets (compile, load, or runtime).

**Q90. Types of advice and execution order.**
- `@Before`, `@AfterReturning`, `@AfterThrowing`, `@After` (finally), `@Around` (most powerful; must call `proceed()`).
- Multiple aspects: `@Order` / `Ordered`. Lower value = higher precedence = outermost wrapper (runs first on the way in, last on the way out).

**Q91. Write pointcut expressions for common cases.**
```java
execution(public * com.app.service..*.*(..))      // all public methods in service package & subpackages
within(com.app.controller..*)                     // all join points within type(s)
@annotation(com.app.audit.Audited)                // methods annotated
@within(org.springframework.stereotype.Service)   // types annotated
bean(*Service)                                    // Spring bean name pattern
args(java.lang.String, ..)                        // argument matching
execution(* *(..)) && !within(com.app.config..*)  // combinations
```

**Q92. Spring AOP vs AspectJ.**

| | Spring AOP | AspectJ |
|---|---|---|
| Weaving | Runtime (proxies) | Compile-time / post-compile / load-time (LTW) |
| Join points | Method execution only | Methods, constructors, field access, static init… |
| Self-invocation | Not intercepted | Intercepted |
| Final/private methods | Not interceptable | Interceptable |
| Performance | Proxy overhead per call | Near-native |
| Complexity | Simple | Build/agent setup |

### Deep Dive

**Q93. Explain the self-invocation problem and its solutions.**
- `this.method()` calls the target directly and bypasses the proxy, so `@Transactional`, `@Async`, `@Cacheable`, and custom aspects are ignored.
- Solutions (best first): move the method to another bean. Inject self (`@Lazy` self-reference or `ObjectProvider`). Use `AopContext.currentProxy()` with `exposeProxy=true` (not recommended). Use AspectJ weaving. Use `TransactionTemplate`.

**Q94. How are Spring's own features built on AOP?**
- `@Transactional` → `TransactionInterceptor`. `@Async` → `AsyncExecutionInterceptor`. `@Cacheable` → `CacheInterceptor`. `@PreAuthorize` → `AuthorizationManagerBeforeMethodInterceptor`. `@Retryable` → `RetryOperationsInterceptor`.
- All are created by auto-proxy creators (BeanPostProcessors) after initialization.

**Q95. Order between `@Transactional` and a custom aspect (e.g., retry). Why does it matter?**
- A retry must wrap *outside* the transaction. Otherwise you retry inside a rollback-only transaction. Set a higher precedence on the retry aspect (`@Order(Ordered.LOWEST_PRECEDENCE - 1)` for the TX vs a lower number for retry), or configure `@EnableTransactionManagement(order = ...)`.

**Q96. Real-world aspects you've built.**
- Auditing, timing/metrics, multi-tenant filter enabling (Hibernate `@Filter`), idempotency keys, distributed locking, masking sensitive logs, feature flags, rate limiting.
- Senior angle: when NOT to use AOP. Hidden control flow, debugging difficulty, and business logic hidden in aspects. Prefer explicit code or filters/interceptors when they fit.

**Q97. AOP vs Servlet Filter vs `HandlerInterceptor` – which and when?**
- Filter: servlet level, before Spring MVC (auth, CORS, logging raw requests). Interceptor: has access to the handler (controller method). AOP: any Spring bean method, not just web.

---

## 11. Spring Security

**Q98. Explain the Spring Security filter chain architecture.**
- `DelegatingFilterProxy` → `FilterChainProxy` → one or more `SecurityFilterChain`s. Key filters: `SecurityContextHolderFilter`, `CsrfFilter`, `UsernamePasswordAuthenticationFilter` / `BearerTokenAuthenticationFilter`, `ExceptionTranslationFilter`, `AuthorizationFilter`.
- `AuthenticationManager` → `ProviderManager` → `AuthenticationProvider`s.

**Q99. How do you secure microservices with OAuth2 / OIDC and JWT?**
- Authorization server (Keycloak, Okta, Auth0, Spring Authorization Server). Services act as resource servers validating JWTs via JWKS.
- Gateway does token relay. Service-to-service: client credentials flow or mTLS.
- JWT trade-offs: stateless, but revocation is hard (short expiry plus refresh tokens, or token introspection).

**Q100. CSRF – when must it be enabled and when can it be disabled?**
- Required for cookie/session-based browser auth. Can be disabled for stateless APIs using Authorization headers.

**Q101. Method-level security and `SecurityContext` propagation in async code.**
- `@EnableMethodSecurity`, `@PreAuthorize("hasRole('ADMIN') or #id == authentication.principal.id")`.
- `DelegatingSecurityContextExecutor` or the `MODE_INHERITABLETHREADLOCAL` strategy (be careful with thread pools).

---

## 12. Microservices Architecture

### Core

**Q102. When should you NOT use microservices?**
- Small team, unclear domain boundaries, early-stage product, no DevOps maturity. The modular monolith is a valid (often better) starting point.
- Costs: distributed debugging, network failures, data consistency, operational overhead, and latency.

**Q103. How do you decide service boundaries?**
- DDD: bounded contexts, aggregates, ubiquitous language. Context mapping (anti-corruption layer, shared kernel, customer/supplier).
- Signs of wrong boundaries: chatty synchronous calls, distributed monolith (must deploy together), shared database.

**Q104. Database-per-service: how do you handle queries that span services?**
- API composition, CQRS read models built from events, data replication via CDC. Avoid shared databases, or at least shared tables.

**Q105. Synchronous vs asynchronous communication. REST vs gRPC vs messaging.**
- REST: simple, ubiquitous, human-readable. gRPC: HTTP/2, Protobuf, streaming, strong contracts, efficient for internal calls. Messaging: temporal decoupling, resilience, eventual consistency.
- Senior point: synchronous call chains multiply failure probability and latency. Availability = product of each dependency's availability.

**Q106. API Gateway – responsibilities and patterns.**
- Routing, auth, rate limiting, request aggregation, protocol translation, TLS termination, canary routing.
- Backend-for-Frontend (BFF) pattern. Spring Cloud Gateway (reactive), Kong, Envoy, AWS API Gateway.
- Pitfall: putting business logic in the gateway.

**Q107. API versioning strategies and backward compatibility.**
- URI (`/v2/`), header, media type. Tolerant reader. Additive changes only. Consumer-driven contract testing (Spring Cloud Contract, Pact).

### Deep Dive – Patterns

**Q108. Explain the Saga pattern. Choreography vs orchestration.**
- A sequence of local transactions with **compensating actions** on failure.
- Choreography: services react to events. Loosely coupled, but hard to follow and risks cyclic dependencies.
- Orchestration: a central coordinator (Temporal, Camunda, Axon, or a custom state machine). Clear flow, but the orchestrator must be HA.
- Design compensations that are idempotent and semantically correct (e.g., "refund", not "delete payment"). Handle pivot transactions.

**Q109. Why not 2PC/XA in microservices?**
- Blocking protocol, coordinator is a single point of failure, holds locks, poor support in NoSQL and brokers, and hurts availability (CAP).

**Q110. Explain the Transactional Outbox pattern. How do you implement it?**
- Problem: dual write (DB update + message publish) is not atomic.
- Solution: write the event to an `outbox` table in the same local transaction. A relay publishes it: polling publisher or **CDC (Debezium)** tailing the WAL/binlog.
- Delivery is at-least-once, so consumers must be idempotent (inbox table / dedupe by event ID).

**Q111. CQRS and Event Sourcing – benefits, costs, when to use.**
- CQRS: separate write model and read model(s). Scale and optimize independently.
- Event Sourcing: state = fold over events. Gives an audit trail, temporal queries, and replay. Costs: schema evolution of events, snapshots, eventual consistency, complexity.
- Don't apply them system-wide. Use them in a bounded context where they pay off.

**Q112. Strangler Fig pattern – how did you migrate a monolith?**
- Route via gateway/proxy, extract one capability at a time, an anti-corruption layer, data migration with dual-write/CDC, and feature flags for cutover.

**Q113. Other patterns to know:** Sidecar, Ambassador, Bulkhead, Circuit Breaker, Retry, Timeout, Rate Limiter, Cache-Aside, Database-per-Service, Shared-Nothing, Backends-for-Frontends, Anti-Corruption Layer, Service Mesh, Leader Election, Competing Consumers.

---

## 13. Distributed Service Management

### 13.1 Service Discovery & Configuration

**Q114. Client-side vs server-side service discovery.**
- Client-side: Eureka + Spring Cloud LoadBalancer. The client queries the registry and picks an instance.
- Server-side: a K8s Service (kube-proxy/DNS) or a load balancer picks the instance. In Kubernetes, Eureka is usually unnecessary.
- Health checks, heartbeat/lease renewal, self-preservation mode in Eureka.

**Q115. Centralized configuration management.**
- Spring Cloud Config Server (Git-backed), Consul KV, K8s ConfigMaps/Secrets, HashiCorp Vault, AWS Parameter Store / Secrets Manager.
- Dynamic refresh: `@RefreshScope` + `/actuator/refresh` or Spring Cloud Bus (Kafka/RabbitMQ) to broadcast.
- Secrets rotation, never committing secrets, encrypting at rest.

### 13.2 Resilience

**Q116. Circuit breaker – states and configuration (Resilience4j).**
- CLOSED → (failure rate / slow-call rate over threshold in a sliding window) → OPEN → (wait duration) → HALF_OPEN → (trial calls succeed) → CLOSED.
- Count-based vs time-based windows. `minimumNumberOfCalls`. Fallbacks must be meaningful (cached data, degraded response), not just hiding errors.
- Hystrix is in maintenance mode. Resilience4j is the standard.

**Q117. Retry best practices.**
- Retry only idempotent or transient failures. Exponential backoff + **jitter**. Cap total attempts.
- **Retry storms**: retries at every layer multiply load (3 layers × 3 retries = 27× load). Use retry budgets and retry at one layer only.
- Combine with a circuit breaker. The order matters (retry inside the CB, or the CB inside retry).

**Q118. Timeouts – why are they the most important resilience setting?**
- Every remote call needs connect and read timeouts. Deadline propagation across the call chain (gRPC deadlines). Timeout < caller's timeout.

**Q119. Bulkhead pattern – thread pool vs semaphore isolation.**
- Isolate resources per dependency so one slow downstream can't exhaust all threads.

**Q120. Rate limiting algorithms.**
- Token bucket, leaky bucket, fixed window, sliding window log/counter.
- Distributed rate limiting with Redis (Lua scripts for atomicity), gateway-level limits, `429` + `Retry-After`.

**Q121. What is graceful degradation and load shedding?**
- Serve partial results, disable non-critical features, prioritize traffic, reject early with 503 under overload, adaptive concurrency limits (Netflix concurrency-limits).

### 13.3 Distributed Systems Theory

**Q122. CAP and PACELC theorems with real database examples.**
- During a partition: choose Consistency or Availability. PACELC adds: Else, Latency vs Consistency.
- CP: ZooKeeper, etcd, HBase. AP: Cassandra, DynamoDB (tunable). PostgreSQL single-node is CA in practice (no partition tolerance).

**Q123. Consistency models.**
- Strong/linearizable, sequential, causal, read-your-writes, monotonic reads, eventual. Which one does your business need? (e.g., balance vs product catalog)

**Q124. How does consensus work? Explain Raft at a high level.**
- Leader election (terms, randomized election timeouts), log replication (majority quorum commit), safety. Used by etcd, Consul, and Kafka KRaft.

**Q125. Idempotency – how do you design idempotent APIs and consumers?**
- `Idempotency-Key` header stored with the response (Stripe-style). Unique constraints. Dedupe tables keyed by message ID. Naturally idempotent operations (`SET` vs `INCREMENT`). Conditional updates with versions.

**Q126. Delivery semantics: at-most-once, at-least-once, exactly-once.**
- "Exactly-once" end-to-end = at-least-once + idempotent processing. Kafka EOS (idempotent producer + transactions) covers Kafka-to-Kafka only.

**Q127. Distributed locking – how and when?**
- Redis (`SET NX PX`, Redisson). Note the Redlock controversy (Kleppmann): use **fencing tokens**. ZooKeeper/etcd leases. DB advisory locks.
- Prefer designs that avoid locks (partitioning by key, single-writer principle).

**Q128. Clock issues in distributed systems.**
- Clock skew and NTP. Never rely on wall clocks for ordering. Lamport timestamps, vector clocks, hybrid logical clocks. Snowflake-style ID generation.

**Q129. Distributed caching strategies and pitfalls.**
- Cache-aside, read-through, write-through, write-behind.
- Problems: **cache stampede** (locking/request coalescing, probabilistic early expiration), **penetration** (cache nulls, Bloom filter), **avalanche** (TTL jitter), hot keys (local L1 cache plus Redis L2), and cache/DB inconsistency (delete-after-write, CDC-driven invalidation).

### 13.4 Service Mesh & Infrastructure

**Q130. What is a service mesh? What does it offload from application code?**
- Istio, Linkerd. Sidecar (Envoy) or ambient mode. mTLS, retries, timeouts, traffic splitting, observability, and authorization policies.
- Trade-off: operational complexity and latency overhead. Avoid duplicated retries (in the app *and* the mesh).

**Q131. Deployment strategies.**
- Rolling, blue-green, canary (with automated analysis — Argo Rollouts, Flagger), shadow/dark traffic, feature flags.
- Database migrations with zero downtime: **expand-and-contract** (Flyway/Liquibase), backward-compatible schema changes.

---

## 14. Messaging & Event-Driven Systems

**Q132. Kafka vs RabbitMQ – when to use which?**
- Kafka: distributed log, retention, replay, high throughput, ordering per partition, consumer groups, stream processing.
- RabbitMQ: smart broker, flexible routing (exchanges), per-message acks, priority queues, lower latency for task queues.

**Q133. Kafka internals: partitions, replication, ISR, `acks`, consumer groups, rebalancing.**
- `acks=all` + `min.insync.replicas=2` + `replication.factor=3` for durability.
- Ordering only within a partition. Choose the key carefully (hot partitions).
- Rebalancing: cooperative sticky assignor, static membership to reduce rebalances. `max.poll.interval.ms` exceeded → consumer kicked out.
- Offset commit strategies: commit after processing (at-least-once).

**Q134. How do you handle poison messages and failures in consumers?**
- Retry topics with backoff (Spring Kafka `@RetryableTopic`), a dead-letter topic (DLT), alerting, and a reprocessing tool. Don't block the partition forever.

**Q135. How do you guarantee message ordering and still scale?**
- Partition by entity ID. A single consumer per partition. Within a consumer, a parallel consumer keyed by ID (Confluent Parallel Consumer).

**Q136. Event schema evolution.**
- Schema Registry with Avro/Protobuf. Backward/forward/full compatibility modes. Event versioning and upcasting.

---

## 15. Observability, Deployment & Cloud-Native

**Q137. The three pillars of observability – how did you implement them?**
- **Logs**: structured JSON, correlation/trace IDs in MDC, centralized (ELK/EFK, Loki). No PII.
- **Metrics**: Micrometer → Prometheus → Grafana. RED (Rate, Errors, Duration) and USE (Utilization, Saturation, Errors) methods. Percentiles (p99) over averages.
- **Traces**: OpenTelemetry, W3C `traceparent` propagation, Micrometer Tracing (Boot 3), Jaeger/Tempo/Zipkin. Sampling strategies (head vs tail).

**Q138. Define SLI, SLO, SLA, and error budgets. How do they drive decisions?**
- Error budget exhausted → freeze features and focus on reliability. Burn-rate alerting.

**Q139. Kubernetes concepts every senior Java dev should know.**
- Liveness vs readiness vs startup probes (don't check DB health in liveness or you get cascading restarts!). Resource requests/limits and JVM container awareness. HPA (CPU vs custom metrics). Graceful shutdown (`server.shutdown=graceful`, `preStop` hook, SIGTERM handling). ConfigMaps/Secrets. PodDisruptionBudgets.

**Q140. 12-Factor App principles – which matter most for Spring Boot services?**
- Config in env, stateless processes, disposability (fast startup and graceful shutdown), logs as event streams, dev/prod parity, backing services as attached resources.

**Q141. Testing strategy for microservices.**
- Test pyramid / honeycomb. Unit → slice tests (`@WebMvcTest`, `@DataJpaTest`) → integration with **Testcontainers** → consumer-driven contracts → minimal E2E. Chaos engineering (Chaos Monkey for Spring Boot, Litmus). Load testing (Gatling, k6).

---

## 16. Scenario-Based & System Design Questions

These are open-ended. Interviewers evaluate structure: **requirements → constraints → design → trade-offs → failure modes → evolution**.

**S1. Design an order-processing system (Order, Payment, Inventory, Shipping) for 10k orders/sec on peak days.**
- Expect: saga (orchestrated), outbox + Kafka, idempotent payment API, inventory reservation with TTL, CQRS read model for order status, partitioning by orderId, back-pressure, DLQ, observability, and a reconciliation job.

**S2. A downstream service is slow and your service's p99 jumped from 200 ms to 10 s, and other endpoints are also failing. Explain why and fix it.**
- Thread pool and connection pool exhaustion from missing timeouts. Fix: timeouts, bulkheads, circuit breaker, fallback, async where suitable.

**S3. Users report occasionally being charged twice. Investigate and fix.**
- Client retries/timeouts without idempotency keys, at-least-once message redelivery, a race in the consumer. Fix: idempotency keys, unique constraints, dedupe, reconciliation.

**S4. After a deployment, memory slowly grows and pods get OOMKilled every ~6 hours.**
- Distinguish heap vs native (`NativeMemoryTracking`, direct buffers, thread count, metaspace). Heap dump analysis. Check the container limit vs `-Xmx` + non-heap. Common causes: unbounded cache, ThreadLocal leak, Netty buffers.

**S5. Design a distributed rate limiter for a public API (per-customer limits, multiple gateway instances).**
- Token bucket in Redis with Lua scripts, local token pre-fetch for throughput, fallback behavior if Redis is down (fail-open vs fail-closed), and response headers.

**S6. Migrate a 15-year-old monolith (Spring 3 + Hibernate 3 + Oracle) to cloud-native microservices with no downtime.**
- Strangler fig, upgrade path (Java/Spring versions first), modular monolith phase, extract by bounded context, CDC from Oracle (Debezium/GoldenGate), anti-corruption layer, parallel run and comparison, feature flags, team topology.

**S7. Two microservices need to keep a "customer profile" consistent. Design the data flow.**
- Single owner service. Others subscribe to `CustomerUpdated` events and maintain local projections. Handle ordering via versions. Backfill via snapshot/compacted topic.

**S8. Design a distributed job scheduler that runs millions of scheduled tasks reliably.**
- Partitioned time buckets, DB polling with `SELECT ... FOR UPDATE SKIP LOCKED`, leases/heartbeats, at-least-once + idempotent jobs, retry/backoff, sharding by job ID. Compare with Quartz clustering and Temporal.

**S9. Hibernate batch job processing 50M rows runs out of memory and takes 9 hours.**
- `StatelessSession` or JDBC, scrollable results/cursor with fetch size, chunked transactions (Spring Batch), `flush/clear`, SEQUENCE IDs for batching, parallel partitioned steps, disabling L2 cache for the job.

**S10. Design the observability stack for 200 microservices. How do you keep costs in check?**
- OpenTelemetry collectors, tail-based sampling, log level discipline, metric cardinality control (no user IDs as labels!), retention tiers, SLO-based alerting instead of per-metric alerts.

**S11. Your Kafka consumer group lag keeps growing during peak.**
- Scale consumers up to the partition count, increase partitions (rehashing implications), optimize processing (batching, async IO), check rebalance storms, `max.poll.records`, slow downstream DB, hot partitions.

**S12. A config change pushed via Spring Cloud Config took down all instances simultaneously. How do you prevent this?**
- Validate config (schema, `@Validated` properties), staged rollout of config like code, canary, rollback, avoid synchronized refresh across the fleet, feature flag systems with gradual rollout.

---

## 17. Leadership & Behavioral (Senior-Level)

At 10+ years, expect equal weight on these questions:

1. Describe the most complex production incident you led. What was the root cause, how did you communicate, and what changed afterward (blameless postmortem)?
2. Tell me about a major architectural decision you made that turned out wrong. How did you detect it, and how did you course-correct?
3. How do you decide between building in-house and adopting a framework or managed service?
4. How do you document and communicate architecture decisions? (ADRs, C4 diagrams, RFC process)
5. How do you handle technical debt prioritization against feature pressure from the business?
6. How do you mentor mid-level engineers into senior engineers? Give a concrete example.
7. How do you enforce code quality and standards across multiple teams without becoming a bottleneck? (Linters, ArchUnit tests, golden paths, platform templates)
8. Describe a time you disagreed with a product manager or architect. How was it resolved?
9. How do you estimate and de-risk a large migration project?
10. What does "production readiness" mean to you? What's on your checklist before a new service goes live?

### Sample Production-Readiness Checklist (great to mention in interviews)
- [ ] Timeouts on every remote call, circuit breakers, retries with jitter
- [ ] Health probes (liveness/readiness/startup) correctly separated
- [ ] Graceful shutdown & connection draining
- [ ] Structured logs with trace IDs; no PII/secrets
- [ ] RED metrics + business metrics + dashboards + SLO alerts
- [ ] Distributed tracing enabled
- [ ] Load tested at 2–3× expected peak
- [ ] Idempotent APIs & consumers; DLQ configured
- [ ] Zero-downtime DB migration strategy
- [ ] Security: authN/authZ, secrets in vault, dependency scanning (CVEs), mTLS
- [ ] Runbook & on-call ownership
- [ ] Capacity plan and autoscaling rules
- [ ] Backup / restore tested; disaster recovery RTO/RPO defined

---

## Rapid-Fire Round (One-Liners Interviewers Love)

| Question | Expected Short Answer |
|---|---|
| Default scope of a Spring bean? | Singleton (per container) |
| Is a singleton bean thread-safe? | No – only if stateless/immutable |
| Default rollback rule of `@Transactional`? | Rollback on `RuntimeException` and `Error`, not checked exceptions |
| Default fetch type of `@ManyToOne`? | EAGER (override to LAZY) |
| Which ID strategy disables JDBC batching? | `IDENTITY` |
| `get()` vs `load()` / `find()` vs `getReference()`? | `find` hits DB/returns null; `getReference` returns proxy, throws on access if missing |
| Why `@Repository`? | Exception translation |
| Can Spring AOP intercept private methods? | No |
| What creates AOP proxies? | `BeanPostProcessor` (auto-proxy creator) after init |
| `HashMap` treeify threshold? | 8 (with capacity ≥ 64) |
| Kafka ordering guarantee? | Per partition only |
| Circuit breaker states? | Closed, Open, Half-Open |
| Outbox solves? | Dual-write problem (DB + broker atomicity) |
| Exactly-once in practice? | At-least-once + idempotency |
| Liveness probe should check DB? | No – use readiness for dependencies |
| Spring Boot 3 minimum Java? | Java 17 |
| `javax` → ? in Boot 3 | `jakarta` |
| Replacement for Sleuth? | Micrometer Tracing (+ OpenTelemetry) |
| Replacement for Hystrix? | Resilience4j |
| Replacement for `WebSecurityConfigurerAdapter`? | `SecurityFilterChain` bean |

---

*Good luck! At this level, the strongest answers combine **"how it works"** with **"what went wrong in production and what I did about it."***
