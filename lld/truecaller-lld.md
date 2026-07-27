# Truecaller-style Caller ID & Spam Detection System — LLD

## 1. Requirements

**Functional**
- Identify an unknown incoming number by resolving a display name — from the user's own contacts, or crowdsourced from what other users have saved that number as, or a verified business directory.
- Community-driven spam reporting: any user can tag a number as Spam/Telemarketer/Scam/Fraud/Safe.
- Aggregate reports into a spam score/category per number; score must reflect report volume, reporter trustworthiness, and recency (old reports matter less).
- Search by name or number.
- Per-user block list: block specific numbers, or auto-block based on rules (e.g., "auto-block anything above 80% spam score", "block all unknown numbers").
- Incoming call screening: given a number, decide **Allow / Flag as spam / Block** before or as the call rings.
- New spam reports should propagate — cached profiles for a number should reflect updated scores.

**Non-functional**
- **Extremely read-heavy**: billions of caller-ID lookups vs. a much smaller volume of spam reports — lookups must be fast/cacheable, independent of how scoring is computed.
- Name resolution must fall back gracefully through multiple sources without the caller knowing/caring which source answered.
- Spam scoring algorithm must be swappable (naive ratio → trust-weighted → ML-based) without touching the reporting or lookup pipeline.
- Block-rule logic must be composable (multiple independent conditions, combined per user) without a hardcoded if/else per rule combination.
- New enrichment data (business info, social profile) addable to a caller profile without modifying the core resolver.

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **Chain of Responsibility** | `CallerIdResolver`: `PersonalContactResolver` → `CrowdsourcedNameResolver` → `BusinessDirectoryResolver` → `UnknownFallbackResolver` | Name resolution is literally "try source 1, if no answer try source 2, ..." — exactly the CoR shape. New data source = new link, no existing resolver touched. |
| **Strategy** | `SpamScoreStrategy`: `SimpleRatioStrategy`, `TrustWeightedStrategy`, `RecencyWeightedStrategy` | *How* reports are aggregated into a score is a separate, swappable concern from *storing* reports or *looking up* numbers. Lets the scoring algorithm evolve (naive → ML-based) independently. |
| **Decorator** | `CallerInfoProvider` base wrapped by `SpamTagDecorator`, `BusinessInfoDecorator` | A caller profile is built by layering independent enrichments (name → spam tag → business info) — decorator avoids one bloated resolver method doing everything, and new enrichments stack without modifying existing ones. |
| **Composite** | `BlockRule` composite: `CompositeBlockRule` (AND/OR) containing `SpamScoreBlockRule`, `SpecificNumberBlockRule`, `UnknownNumberBlockRule` | Each user's block policy is a combination of independent conditions. Composite lets rules nest/combine arbitrarily (e.g., "block if (spam score > 0.8) OR (unknown AND user pref set)") using the same `BlockRule` interface throughout. |
| **Observer** | `SpamRegistry` (Subject) notifies `RegistryObserver`: `CacheInvalidator`, `AutoBlockReevaluator`, `AnalyticsLogger` | A new report is one event with multiple independent downstream effects (invalidate stale cached profile, re-check auto-block thresholds, feed analytics) — none of which the reporting flow itself should know about. |
| **Singleton** | `SpamRegistry` | Must be the single, central source of truth for report data and cached scores per number — the whole system's correctness depends on there being one authoritative registry. |
| **Factory Method** | `CallerProfileFactory.buildProvider()` | Encapsulates which decorator stack (spam tag + business info, etc.) gets wired together to build a profile provider. |
| **Builder** | `Report.Builder`, `CallerProfile.Builder` | Reports/profiles have several optional fields (category, evidence, business metadata). |

**SOLID**
- **S**: `CallerIdResolver` implementations only resolve names; `SpamScoreStrategy` implementations only compute a score; `SpamRegistry` only stores/aggregates reports; `CallScreeningService` only decides the call action.
- **O**: New name source → new resolver link. New scoring algorithm → new strategy. New block condition → new `BlockRule` leaf. New enrichment → new decorator. Nothing existing is modified.
- **L**: Any `CallerIdResolver`/`SpamScoreStrategy`/`BlockRule`/`CallerInfoProvider` is substitutable wherever the interface is expected.
- **I**: `RegistryObserver` exposes only `onNewReport`; `BlockRule` exposes only `shouldBlock` — no fat interfaces forcing unrelated responsibilities.
- **D**: `CallScreeningService` depends on `CallerInfoProvider`, `BlockRule`, and `SpamRegistry` abstractions, injected/looked-up — never on concrete resolver or scoring classes.

---

## 3. Class Diagram (textual)

```
┌───────────────────────┐        ┌────────────────────────┐
│  CallerIdResolver          │◀───────│  CallerIdResolutionChain    │
│ (Chain of Responsibility)    │       │  (builds the chain)            │
│ + resolve(number, user)        │     └────────────────────────┘
│ + setNext(resolver)              │
└──────────▲──────────────┘
   ┌───────┼─────────┬──────────────┬──────────────┐
PersonalContact  Crowdsourced  BusinessDirectory  UnknownFallback
  Resolver         NameResolver    Resolver           Resolver

┌────────────────────┐        ┌──────────────────────────┐
│  SpamScoreStrategy      │      │  SpamRegistry (Singleton, Subject)│
│ (Strategy interface)      │    │  - reportsByNumber: Map<..,List<Report>>│
│ + calculateScore(reports)   │  │  + submitReport(report)                  │
└──────────▲───────────┘      │  + getScore(number): double                │
   ┌───────┼────────┬────────┐│  + getCategory(number): SpamCategory        │
SimpleRatio TrustWeighted Recency└──────────────────────────┘
 Strategy    Strategy      WeightedStrategy

┌────────────────────┐        ┌──────────────────────────┐
│  RegistryObserver       │      │  CallerInfoProvider           │
│ + onNewReport(num,report)  │  │ (base + Decorator interface)     │
└──────────▲───────────┘      │ + getProfile(number, user): CallerProfile│
   ┌───────┼────────┬────────┐└──────────▲───────────┘
CacheInvalidator AutoBlockReevaluator AnalyticsLogger   │
                                          BaseCallerInfoProvider (uses resolver chain)
                                                 ▲
                                    ┌────────────┼─────────────┐
                              SpamTagDecorator          BusinessInfoDecorator

┌────────────────────┐        ┌──────────────────────────┐
│  BlockRule              │      │  CompositeBlockRule (Composite)  │
│ (Composite component)     │◀────│  - rules: List<BlockRule>          │
│ + shouldBlock(profile)      │   │  - operator: AND/OR                  │
└──────────▲───────────┘      └──────────────────────────┘
   ┌───────┼────────┬────────┐
SpamScoreBlockRule SpecificNumberBlockRule UnknownNumberBlockRule

┌────────────────────┐        ┌──────────────────────────┐
│  CallScreeningService     │    │  Report (Builder)              │
│  + screenIncomingCall()      │  └──────────────────────────┘
└────────────────────┘

┌──────────┐  ┌──────────┐  ┌──────────────┐  ┌──────────────┐
│  User        │  │  Contact     │  │  PhoneNumber       │  │  CallerProfile      │
└──────────┘  └──────────┘  └──────────────┘  └──────────────┘
```

---

## 4. Code (Java)

### 4.1 Core entities

```java
public class PhoneNumber {
    private final String e164; // normalized, e.g. "+919876543210"
    public PhoneNumber(String e164) { this.e164 = e164; }
    @Override public boolean equals(Object o) { return o instanceof PhoneNumber && ((PhoneNumber) o).e164.equals(e164); }
    @Override public int hashCode() { return e164.hashCode(); }
    public String getValue() { return e164; }
}

public enum SpamCategory { SAFE, TELEMARKETER, SPAM, SCAM, FRAUD, UNKNOWN }

public class Contact {
    private final PhoneNumber number;
    private final String savedName;
    public Contact(PhoneNumber number, String savedName) { this.number = number; this.savedName = savedName; }
    public PhoneNumber getNumber() { return number; }
    public String getSavedName() { return savedName; }
}

public class User {
    private final String id;
    private final String name;
    private final double trustScore; // reputation, used by TrustWeightedStrategy
    private final List<Contact> contacts;
    private final BlockRule blockPolicy; // per-user composite rule
    // getters omitted
    public User(String id, String name, double trustScore, List<Contact> contacts, BlockRule blockPolicy) {
        this.id = id; this.name = name; this.trustScore = trustScore;
        this.contacts = contacts; this.blockPolicy = blockPolicy;
    }
    public double getTrustScore() { return trustScore; }
    public List<Contact> getContacts() { return contacts; }
    public BlockRule getBlockPolicy() { return blockPolicy; }
    public String getId() { return id; }
}

public class CallerProfile {
    private final PhoneNumber number;
    private String displayName = "Unknown";
    private SpamCategory category = SpamCategory.UNKNOWN;
    private double spamScore = 0.0;
    private String businessName; // nullable
    private boolean businessVerified;

    public CallerProfile(PhoneNumber number) { this.number = number; }
    // getters/setters used by decorators
    public PhoneNumber getNumber() { return number; }
    public String getDisplayName() { return displayName; }
    public void setDisplayName(String n) { this.displayName = n; }
    public SpamCategory getCategory() { return category; }
    public void setCategory(SpamCategory c) { this.category = c; }
    public double getSpamScore() { return spamScore; }
    public void setSpamScore(double s) { this.spamScore = s; }
    public void setBusinessInfo(String name, boolean verified) { this.businessName = name; this.businessVerified = verified; }
}
```

### 4.2 Report (Builder)

```java
public class Report {
    private final String id = UUID.randomUUID().toString();
    private final PhoneNumber number;
    private final String reportedByUserId;
    private final double reporterTrustScore;
    private final SpamCategory category;
    private final long timestamp;

    private Report(Builder b) {
        this.number = b.number; this.reportedByUserId = b.reportedByUserId;
        this.reporterTrustScore = b.reporterTrustScore; this.category = b.category;
        this.timestamp = System.currentTimeMillis();
    }

    public PhoneNumber getNumber() { return number; }
    public SpamCategory getCategory() { return category; }
    public double getReporterTrustScore() { return reporterTrustScore; }
    public long getTimestamp() { return timestamp; }

    public static class Builder {
        private PhoneNumber number; private String reportedByUserId;
        private double reporterTrustScore = 1.0; private SpamCategory category;

        public Builder number(PhoneNumber n) { this.number = n; return this; }
        public Builder reportedBy(User u) { this.reportedByUserId = u.getId(); this.reporterTrustScore = u.getTrustScore(); return this; }
        public Builder category(SpamCategory c) { this.category = c; return this; }
        public Report build() { return new Report(this); }
    }
}
```

### 4.3 Strategy — Spam Score Calculation

```java
public interface SpamScoreStrategy {
    double calculateScore(List<Report> reports); // 0.0 (safe) to 1.0 (definitely spam)
}

public class SimpleRatioStrategy implements SpamScoreStrategy {
    public double calculateScore(List<Report> reports) {
        if (reports.isEmpty()) return 0.0;
        long spamCount = reports.stream()
                .filter(r -> r.getCategory() != SpamCategory.SAFE)
                .count();
        return (double) spamCount / reports.size();
    }
}

public class TrustWeightedStrategy implements SpamScoreStrategy {
    public double calculateScore(List<Report> reports) {
        if (reports.isEmpty()) return 0.0;
        double totalWeight = reports.stream().mapToDouble(Report::getReporterTrustScore).sum();
        double spamWeight = reports.stream()
                .filter(r -> r.getCategory() != SpamCategory.SAFE)
                .mapToDouble(Report::getReporterTrustScore)
                .sum();
        return totalWeight == 0 ? 0.0 : spamWeight / totalWeight;
    }
}

public class RecencyWeightedStrategy implements SpamScoreStrategy {
    private static final long HALF_LIFE_MS = 30L * 24 * 60 * 60 * 1000; // 30 days

    public double calculateScore(List<Report> reports) {
        if (reports.isEmpty()) return 0.0;
        long now = System.currentTimeMillis();
        double totalWeight = 0, spamWeight = 0;
        for (Report r : reports) {
            double age = now - r.getTimestamp();
            double weight = Math.pow(0.5, age / HALF_LIFE_MS); // exponential decay
            totalWeight += weight;
            if (r.getCategory() != SpamCategory.SAFE) spamWeight += weight;
        }
        return totalWeight == 0 ? 0.0 : spamWeight / totalWeight;
    }
}
```

### 4.4 Observer + Singleton — SpamRegistry

```java
public interface RegistryObserver {
    void onNewReport(PhoneNumber number, Report report);
}

public class SpamRegistry {
    private static volatile SpamRegistry instance;

    private final ConcurrentHashMap<PhoneNumber, List<Report>> reportsByNumber = new ConcurrentHashMap<>();
    private final ConcurrentHashMap<PhoneNumber, Double> scoreCache = new ConcurrentHashMap<>();
    private final List<RegistryObserver> observers = new ArrayList<>();
    private SpamScoreStrategy scoreStrategy = new RecencyWeightedStrategy(); // default, swappable

    private SpamRegistry() {}

    public static SpamRegistry getInstance() {
        if (instance == null) {
            synchronized (SpamRegistry.class) {
                if (instance == null) instance = new SpamRegistry();
            }
        }
        return instance;
    }

    public void setScoreStrategy(SpamScoreStrategy strategy) { this.scoreStrategy = strategy; }
    public void subscribe(RegistryObserver o) { observers.add(o); }

    public void submitReport(Report report) {
        reportsByNumber.computeIfAbsent(report.getNumber(), n -> new CopyOnWriteArrayList<>()).add(report);
        scoreCache.remove(report.getNumber()); // invalidate cached score
        for (RegistryObserver o : observers) o.onNewReport(report.getNumber(), report);
    }

    public double getScore(PhoneNumber number) {
        return scoreCache.computeIfAbsent(number, n ->
                scoreStrategy.calculateScore(reportsByNumber.getOrDefault(n, Collections.emptyList())));
    }

    public SpamCategory getCategory(PhoneNumber number) {
        double score = getScore(number);
        if (score >= 0.7) return SpamCategory.SPAM;
        if (score >= 0.4) return SpamCategory.TELEMARKETER;
        if (score > 0) return SpamCategory.UNKNOWN;
        return reportsByNumber.containsKey(number) ? SpamCategory.SAFE : SpamCategory.UNKNOWN;
    }

    /** Crowdsourced name = most frequently saved name for this number across all reports/contacts. */
    public Optional<String> getMostCommonName(PhoneNumber number, Map<PhoneNumber, List<String>> nameVotes) {
        List<String> names = nameVotes.get(number);
        if (names == null || names.isEmpty()) return Optional.empty();
        return names.stream()
                .collect(Collectors.groupingBy(n -> n, Collectors.counting()))
                .entrySet().stream().max(Map.Entry.comparingByValue())
                .map(Map.Entry::getKey);
    }
}
```

### 4.5 Observers reacting to new reports

```java
public class CacheInvalidator implements RegistryObserver {
    public void onNewReport(PhoneNumber number, Report report) {
        ProfileCache.getInstance().invalidate(number); // drop stale cached CallerProfile
    }
}

public class AutoBlockReevaluator implements RegistryObserver {
    public void onNewReport(PhoneNumber number, Report report) {
        double newScore = SpamRegistry.getInstance().getScore(number);
        if (newScore >= 0.8) {
            System.out.println("[AutoBlock] " + number.getValue() + " crossed high-confidence spam threshold");
            // could push to affected users' block lists if they opted into aggressive auto-block
        }
    }
}

public class AnalyticsLogger implements RegistryObserver {
    public void onNewReport(PhoneNumber number, Report report) {
        // write to analytics pipeline
    }
}

// simple cache shell referenced above
public class ProfileCache {
    private static final ProfileCache instance = new ProfileCache();
    private final ConcurrentHashMap<PhoneNumber, CallerProfile> cache = new ConcurrentHashMap<>();
    public static ProfileCache getInstance() { return instance; }
    public CallerProfile get(PhoneNumber n) { return cache.get(n); }
    public void put(PhoneNumber n, CallerProfile p) { cache.put(n, p); }
    public void invalidate(PhoneNumber n) { cache.remove(n); }
}
```

### 4.6 Chain of Responsibility — Caller ID name resolution

```java
public abstract class CallerIdResolver {
    protected CallerIdResolver next;
    public CallerIdResolver setNext(CallerIdResolver next) { this.next = next; return next; }

    /** @return resolved name, or delegates to next resolver if this source has nothing. */
    public final String resolve(PhoneNumber number, User requestingUser) {
        String result = tryResolve(number, requestingUser);
        if (result != null) return result;
        return next != null ? next.resolve(number, requestingUser) : "Unknown";
    }
    protected abstract String tryResolve(PhoneNumber number, User requestingUser);
}

public class PersonalContactResolver extends CallerIdResolver {
    protected String tryResolve(PhoneNumber number, User user) {
        return user.getContacts().stream()
                .filter(c -> c.getNumber().equals(number))
                .map(Contact::getSavedName)
                .findFirst().orElse(null);
    }
}

public class CrowdsourcedNameResolver extends CallerIdResolver {
    private final Map<PhoneNumber, List<String>> globalNameVotes; // simulated global DB
    public CrowdsourcedNameResolver(Map<PhoneNumber, List<String>> globalNameVotes) { this.globalNameVotes = globalNameVotes; }

    protected String tryResolve(PhoneNumber number, User user) {
        return SpamRegistry.getInstance().getMostCommonName(number, globalNameVotes).orElse(null);
    }
}

public class BusinessDirectoryResolver extends CallerIdResolver {
    private final Map<PhoneNumber, String> verifiedBusinesses;
    public BusinessDirectoryResolver(Map<PhoneNumber, String> verifiedBusinesses) { this.verifiedBusinesses = verifiedBusinesses; }

    protected String tryResolve(PhoneNumber number, User user) {
        return verifiedBusinesses.get(number);
    }
}

public class UnknownFallbackResolver extends CallerIdResolver {
    protected String tryResolve(PhoneNumber number, User user) { return "Unknown"; }
}
```

### 4.7 Decorator — building the enriched CallerProfile

```java
public interface CallerInfoProvider {
    CallerProfile getProfile(PhoneNumber number, User requestingUser);
}

public class BaseCallerInfoProvider implements CallerInfoProvider {
    private final CallerIdResolver resolverChain;
    public BaseCallerInfoProvider(CallerIdResolver resolverChain) { this.resolverChain = resolverChain; }

    public CallerProfile getProfile(PhoneNumber number, User requestingUser) {
        CallerProfile cached = ProfileCache.getInstance().get(number);
        if (cached != null) return cached;

        CallerProfile profile = new CallerProfile(number);
        profile.setDisplayName(resolverChain.resolve(number, requestingUser));
        ProfileCache.getInstance().put(number, profile);
        return profile;
    }
}

public abstract class CallerInfoDecorator implements CallerInfoProvider {
    protected final CallerInfoProvider wrapped;
    protected CallerInfoDecorator(CallerInfoProvider wrapped) { this.wrapped = wrapped; }
}

public class SpamTagDecorator extends CallerInfoDecorator {
    public SpamTagDecorator(CallerInfoProvider wrapped) { super(wrapped); }
    public CallerProfile getProfile(PhoneNumber number, User requestingUser) {
        CallerProfile profile = wrapped.getProfile(number, requestingUser);
        profile.setSpamScore(SpamRegistry.getInstance().getScore(number));
        profile.setCategory(SpamRegistry.getInstance().getCategory(number));
        return profile;
    }
}

public class BusinessInfoDecorator extends CallerInfoDecorator {
    private final Map<PhoneNumber, String> verifiedBusinesses;
    public BusinessInfoDecorator(CallerInfoProvider wrapped, Map<PhoneNumber, String> verifiedBusinesses) {
        super(wrapped); this.verifiedBusinesses = verifiedBusinesses;
    }
    public CallerProfile getProfile(PhoneNumber number, User requestingUser) {
        CallerProfile profile = wrapped.getProfile(number, requestingUser);
        String biz = verifiedBusinesses.get(number);
        if (biz != null) profile.setBusinessInfo(biz, true);
        return profile;
    }
}
```

### 4.8 Composite — per-user block rules

```java
public interface BlockRule {
    boolean shouldBlock(CallerProfile profile);
}

public class SpamScoreBlockRule implements BlockRule {
    private final double threshold;
    public SpamScoreBlockRule(double threshold) { this.threshold = threshold; }
    public boolean shouldBlock(CallerProfile profile) { return profile.getSpamScore() >= threshold; }
}

public class SpecificNumberBlockRule implements BlockRule {
    private final Set<PhoneNumber> manuallyBlocked;
    public SpecificNumberBlockRule(Set<PhoneNumber> manuallyBlocked) { this.manuallyBlocked = manuallyBlocked; }
    public boolean shouldBlock(CallerProfile profile) { return manuallyBlocked.contains(profile.getNumber()); }
}

public class UnknownNumberBlockRule implements BlockRule {
    public boolean shouldBlock(CallerProfile profile) { return "Unknown".equals(profile.getDisplayName()); }
}

public enum LogicalOperator { AND, OR }

public class CompositeBlockRule implements BlockRule {
    private final List<BlockRule> rules = new ArrayList<>();
    private final LogicalOperator operator;

    public CompositeBlockRule(LogicalOperator operator) { this.operator = operator; }
    public CompositeBlockRule add(BlockRule rule) { rules.add(rule); return this; }

    public boolean shouldBlock(CallerProfile profile) {
        if (rules.isEmpty()) return false;
        return operator == LogicalOperator.AND
                ? rules.stream().allMatch(r -> r.shouldBlock(profile))
                : rules.stream().anyMatch(r -> r.shouldBlock(profile));
    }
}
```

### 4.9 Call screening — putting resolver + spam tag + block rules together

```java
public enum CallAction { ALLOW, FLAG_AS_SPAM, BLOCK }

public class CallScreeningService {
    private final CallerInfoProvider profileProvider;

    public CallScreeningService(CallerInfoProvider profileProvider) {
        this.profileProvider = profileProvider;
    }

    public CallAction screenIncomingCall(PhoneNumber caller, User recipient) {
        CallerProfile profile = profileProvider.getProfile(caller, recipient);

        if (recipient.getBlockPolicy().shouldBlock(profile)) {
            return CallAction.BLOCK;
        }
        if (profile.getCategory() == SpamCategory.SPAM || profile.getCategory() == SpamCategory.SCAM) {
            return CallAction.FLAG_AS_SPAM; // show warning but still let it ring
        }
        return CallAction.ALLOW;
    }
}
```

### 4.10 Factory Method — wiring the resolver chain + decorator stack

```java
public class CallerProfileFactory {
    public static CallerInfoProvider buildDefaultProvider(Map<PhoneNumber, List<String>> globalNameVotes,
                                                            Map<PhoneNumber, String> verifiedBusinesses) {
        CallerIdResolver chain = new PersonalContactResolver();
        chain.setNext(new CrowdsourcedNameResolver(globalNameVotes))
             .setNext(new BusinessDirectoryResolver(verifiedBusinesses))
             .setNext(new UnknownFallbackResolver());

        CallerInfoProvider provider = new BaseCallerInfoProvider(chain);
        provider = new SpamTagDecorator(provider);
        provider = new BusinessInfoDecorator(provider, verifiedBusinesses);
        return provider;
    }
}
```

### 4.11 Putting it together

```java
public class TruecallerDemo {
    public static void main(String[] args) {
        SpamRegistry registry = SpamRegistry.getInstance();
        registry.subscribe(new CacheInvalidator());
        registry.subscribe(new AutoBlockReevaluator());
        registry.subscribe(new AnalyticsLogger());
        registry.setScoreStrategy(new TrustWeightedStrategy()); // swap algorithm easily

        PhoneNumber telemarketer = new PhoneNumber("+911234567890");

        // simulate crowdsourced name votes and business directory
        Map<PhoneNumber, List<String>> nameVotes = new HashMap<>();
        nameVotes.put(telemarketer, List.of("Loan Offers Inc", "Loan Offers Inc", "Spam Caller"));
        Map<PhoneNumber, String> businesses = new HashMap<>();

        CallerInfoProvider provider = CallerProfileFactory.buildDefaultProvider(nameVotes, businesses);

        User alice = new User("u1", "Alice", 0.9, List.of(),
                new CompositeBlockRule(LogicalOperator.OR)
                        .add(new SpamScoreBlockRule(0.7))
                        .add(new SpecificNumberBlockRule(Set.of())));

        // several users report this number as spam
        for (int i = 0; i < 5; i++) {
            User reporter = new User("reporter" + i, "R" + i, 1.0, List.of(), null);
            registry.submitReport(new Report.Builder()
                    .number(telemarketer).reportedBy(reporter).category(SpamCategory.SPAM).build());
        }

        CallScreeningService screening = new CallScreeningService(provider);
        CallAction action = screening.screenIncomingCall(telemarketer, alice);
        System.out.println("Incoming call from " + telemarketer.getValue() + " -> " + action);
        // -> BLOCK, since spam score now exceeds Alice's 0.7 threshold
    }
}
```

---

## 5. Why this shape holds up under follow-ups

- **"Add social-media-linked profile pictures to caller ID"** → another `CallerInfoDecorator`; nothing else changes.
- **"Switch spam scoring to an ML model"** → new `SpamScoreStrategy` implementation calling out to a model service; `SpamRegistry.setScoreStrategy(...)` swaps it in, zero changes to reporting or lookup paths.
- **"Add SMS spam filtering alongside calls"** → reuse `CallerInfoProvider` + `BlockRule` composite entirely; only a new `SmsScreeningService` sibling to `CallScreeningService` is needed.
- **"Let users whitelist specific numbers even if flagged spam"** → add a `WhitelistBlockRule` returning `false` unconditionally, combined via a `NOT`/priority-override composite — Composite pattern already supports this shape.
- **"Rate-limit spam reports to prevent report-flooding abuse"** → wrap `submitReport` with a **Chain of Responsibility** validator (reuse the same pattern as the Rate Limiter / Brokerage designs) before it reaches `SpamRegistry`.
- **"Scale lookups to billions of queries/day"** → `ProfileCache` is already the seam; swap in-memory map for a distributed cache (Redis/CDN edge cache) — `CallerInfoProvider`'s contract is unchanged, so decorators/resolvers need no modification.

---

Want me to extend this with **a distributed cache/CDN strategy for the lookup hot path, report-abuse rate-limiting (reusing the earlier Rate Limiter design), a trust-score computation engine for reporters, or a numbering-plan-aware phone number validation layer**, or move to a different LLD problem?