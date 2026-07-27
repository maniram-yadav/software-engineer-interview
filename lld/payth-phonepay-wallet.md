# Paytm/PhonePe-style Wallet System — LLD

## 1. Requirements

**Functional**
- Create wallet for a user; link funding sources (bank account, debit/credit card, UPI).
- Add money to wallet (top-up) from a funding source.
- Transfer money wallet-to-wallet (P2P), pay a merchant, pay a bill.
- Transaction lifecycle with full auditability: Initiated → Processing → Success / Failed → Reversed (refund).
- **Idempotency**: retried/duplicate requests (network retry, double-tap) must not double-debit or double-credit.
- **Concurrency safety**: concurrent transactions on the same wallet must never corrupt balance (classic lost-update problem).
- Validation pipeline: KYC tier limits, balance sufficiency, daily/monthly limits, fraud checks — before any money moves.
- Fee/charges (platform fee, GST) computed on top of transaction amount for certain transaction types.
- Refund/reversal of a completed transaction.
- Transaction history per wallet.
- Notify user on each transaction state change.

**Non-functional**
- **Correctness over speed** — this is money; every operation must be atomic and auditable (double-entry ledger, not just a balance field).
- New funding sources / transaction types pluggable without touching core transfer logic.
- New validation rules pluggable independently.
- Every money movement must be traceable and reversible.

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **Command** | `Transaction` as a command with `execute()` / `reverse()` | A financial transaction is fundamentally "an action that happened and might need to be undone." Refund is literally the inverse of the original command — Command pattern makes that symmetry explicit instead of writing separate ad-hoc refund logic. |
| **State** | `TransactionState`: `InitiatedState`, `ProcessingState`, `SuccessState`, `FailedState`, `ReversedState` | Legal actions depend entirely on current state (can't reverse a failed transaction, can't retry a successful one). Prevents illegal transitions — critical when real money is on the line. |
| **Chain of Responsibility** | `TransactionValidator` chain: `KYCLimitValidator` → `BalanceValidator` → `DailyLimitValidator` → `FraudCheckValidator` | Every transaction must pass a sequence of independent checks before money moves. New compliance rule = new link, no existing validator touched — important since regulatory rules change often and independently. |
| **Strategy** | `FundingSourceStrategy` (`BankTransferFunding`, `CardFunding`, `UPIFunding`); `TransactionTypeStrategy` implicit via distinct transaction classes | How money enters the wallet varies by source; each has different processing/settlement mechanics. Isolating this keeps `WalletService` unaware of funding-source-specific details. |
| **Decorator** | `FeeCalculator` wrapped by `PlatformFeeDecorator`, `GSTDecorator` | Total deduction = base amount + optional platform fee + optional GST, stacked independently per transaction type (P2P is usually fee-free, merchant payments aren't) — avoids a combinatorial explosion of fee classes. |
| **Observer** | `Transaction` (Subject) notifies `TransactionObserver`: `NotificationService`, `LedgerWriter`, `FraudAnalyticsLogger` | One state change → multiple independent reactions (push notification, ledger entry, fraud model feed) without `Transaction` knowing about any of them. |
| **Singleton** | `IdempotencyKeyStore`, `WalletLockManager` | Both must be single, central sources of truth process-wide — idempotency keys must be checked/stored in one place; wallet locks must be the one chokepoint preventing concurrent balance corruption. |
| **Factory Method** | `TransactionFactory.create(type, ...)` | Encapsulates which fee decorator stack + validator chain + funding strategy pairs with a given transaction type. |
| **Builder** | `Transaction.Builder` | Many optional fields (fee breakdown, funding source, metadata, idempotency key). |

**SOLID**
- **S**: `WalletService` orchestrates; `LedgerService` only records double-entry postings; `WalletLockManager` only manages concurrency locks; `FeeCalculator` only computes fees.
- **O**: New funding source → new `FundingSourceStrategy`. New validation rule → new chain link. New transaction reaction → new observer. Nothing existing changes.
- **L**: Any `TransactionState`/`FundingSourceStrategy`/`FeeCalculator` decorator substitutable wherever used.
- **I**: `TransactionObserver` exposes only `onStateChange`; `TransactionValidator` exposes only `validate`/`setNext` — narrow contracts.
- **D**: `WalletService` depends on `TransactionValidator`, `FundingSourceStrategy`, `FeeCalculator` abstractions injected at creation, never concrete classes.

---

## 3. Class Diagram (textual)

```
┌──────────────────┐        ┌──────────────────────────┐
│  TransactionState      │◀──────│  Transaction (Command, Context, Subject)│
│ (State interface)        │      │ - state: TransactionState                  │
│ + process()/succeed()/     │    │ - fromWallet, toWallet, amount               │
│   fail()/reverse()           │  │ - fundingSource, feeCalculator                 │
└────────▲──────────────┘      │ - observers: List<Obs>                           │
  ┌──────┼───────┬───────────┬─│ + execute() / reverse()                            │
Initiated Processing Success Failed  Reversed  └──────────────────────────┘
 State     State      State   State    State

┌────────────────────────┐      ┌──────────────────────┐
│  TransactionValidator       │    │  FundingSourceStrategy    │
│ (Chain of Responsibility)     │  │ (Strategy interface)         │
│ + validate(txn)                 │ │ + pullFunds(amount)            │
│ + setNext(validator)              │└──────────▲───────────┘
└──────────▲──────────────┘      ┌───────┼────────┐
   ┌───────┼────┬───────────┐ Bank Transfer  CardFunding  UPIFunding
KYCLimit  Balance DailyLimit FraudCheck   Funding
Validator Validator Validator Validator

┌──────────────────┐        ┌──────────────────────┐
│  FeeCalculator (base)  │◀── decorated by ──┐        │  TransactionObserver     │
└──────────────────┘                     │        │ + onStateChange(txn,evt)   │
   ┌──────────────────────┬──────────────┘        └──────────▲───────────┘
PlatformFeeDecorator  GSTDecorator                     ┌──────┼───────┬──────────┐
                                             NotificationService LedgerWriter FraudAnalyticsLogger

┌──────────────────┐        ┌──────────────────────┐
│  IdempotencyKeyStore   │      │  WalletLockManager       │
│  (Singleton)              │  │  (Singleton)                │
│  + checkAndStore(key)       │ │  + acquireLocks(walletIds)    │
└──────────────────┘        │  + releaseLocks(walletIds)       │
                              └──────────────────────┘

┌──────────────────┐        ┌──────────────────────┐
│  Wallet                │      │  LedgerService (double-entry)│
│  - balance                │  │  + postEntry(debit, credit)     │
└──────────────────┘        └──────────────────────┘

┌──────────────────┐        ┌──────────────────────┐
│  WalletService          │    │  TransactionFactory       │
│  + transfer()/topUp()      │  └──────────────────────┘
│  + refund()
└──────────────────┘
```

---

## 4. Code (Java)

### 4.1 Core entities

```java
public enum TransactionType { TOP_UP, P2P_TRANSFER, MERCHANT_PAYMENT, BILL_PAYMENT, REFUND }

public class Wallet {
    private final String id;
    private final String userId;
    private long balanceInPaise; // always use integer minor units for money — never double/float
    private final Object lock = new Object(); // used only internally by WalletLockManager's per-wallet monitor

    public Wallet(String id, String userId, long initialBalance) {
        this.id = id; this.userId = userId; this.balanceInPaise = initialBalance;
    }

    // mutations only happen through WalletLockManager-guarded paths
    void debit(long amount) {
        if (balanceInPaise < amount) throw new InsufficientBalanceException("Insufficient balance");
        balanceInPaise -= amount;
    }
    void credit(long amount) { balanceInPaise += amount; }

    public long getBalance() { return balanceInPaise; }
    public String getId() { return id; }
    public String getUserId() { return userId; }
}

class InsufficientBalanceException extends RuntimeException {
    public InsufficientBalanceException(String msg) { super(msg); }
}
```

### 4.2 Singleton — WalletLockManager (prevents concurrent balance corruption)

This is the payment-system equivalent of `SeatLockManager` from the BookMyShow design — the actual concurrency chokepoint. Locks are acquired in a **consistent global order** (sorted by wallet ID) to prevent deadlock when two transactions touch the same pair of wallets in opposite directions.

```java
public class WalletLockManager {
    private static volatile WalletLockManager instance;
    private final ConcurrentHashMap<String, ReentrantLock> walletLocks = new ConcurrentHashMap<>();

    private WalletLockManager() {}

    public static WalletLockManager getInstance() {
        if (instance == null) {
            synchronized (WalletLockManager.class) {
                if (instance == null) instance = new WalletLockManager();
            }
        }
        return instance;
    }

    private ReentrantLock lockFor(String walletId) {
        return walletLocks.computeIfAbsent(walletId, id -> new ReentrantLock());
    }

    /** Acquires locks on all given wallets in a fixed order to prevent deadlock. */
    public List<ReentrantLock> acquireLocks(List<String> walletIds) {
        List<String> sorted = new ArrayList<>(new TreeSet<>(walletIds)); // dedupe + consistent order
        List<ReentrantLock> acquired = new ArrayList<>();
        for (String id : sorted) {
            ReentrantLock lock = lockFor(id);
            lock.lock();
            acquired.add(lock);
        }
        return acquired;
    }

    public void releaseLocks(List<ReentrantLock> locks) {
        for (int i = locks.size() - 1; i >= 0; i--) locks.get(i).unlock(); // reverse order
    }
}
```

> In a distributed deployment (multiple app servers), this becomes a **DB row-level lock** (`SELECT ... FOR UPDATE` on the wallet row, ordered by wallet ID) or a distributed lock (Redis Redlock). Same seam — callers depend on `acquireLocks`/`releaseLocks`, only the internals change.

### 4.3 Singleton — IdempotencyKeyStore (prevents duplicate processing)

```java
public class IdempotencyKeyStore {
    private static volatile IdempotencyKeyStore instance;
    // maps idempotency key -> transaction ID already processed for it
    private final ConcurrentHashMap<String, String> processedKeys = new ConcurrentHashMap<>();

    private IdempotencyKeyStore() {}

    public static IdempotencyKeyStore getInstance() {
        if (instance == null) {
            synchronized (IdempotencyKeyStore.class) {
                if (instance == null) instance = new IdempotencyKeyStore();
            }
        }
        return instance;
    }

    /** @return existing transactionId if this key was already processed, else null and records it atomically. */
    public String checkAndReserve(String idempotencyKey, String newTransactionId) {
        String existing = processedKeys.putIfAbsent(idempotencyKey, newTransactionId);
        return existing; // null means this call just reserved the key (first time seeing it)
    }
}
```

> In production this is backed by a DB unique constraint on `idempotency_key`, with a TTL (e.g., 24 hours) — the in-memory map here illustrates the contract only.

### 4.4 Double-entry Ledger (correctness backbone)

Rather than just mutating `balance` fields, every transaction posts **two entries** (debit + credit) to an immutable ledger. This is what makes the system auditable and reconcilable — a wallet's balance is derivable/verifiable from its ledger entries, not just trusted blindly.

```java
public enum EntryType { DEBIT, CREDIT }

public class LedgerEntry {
    private final String id = UUID.randomUUID().toString();
    private final String walletId;
    private final EntryType type;
    private final long amount;
    private final String transactionId;
    private final long timestamp = System.currentTimeMillis();

    public LedgerEntry(String walletId, EntryType type, long amount, String transactionId) {
        this.walletId = walletId; this.type = type; this.amount = amount; this.transactionId = transactionId;
    }
    // getters omitted
}

public class LedgerService {
    private static final LedgerService instance = new LedgerService();
    private final List<LedgerEntry> entries = new CopyOnWriteArrayList<>(); // append-only

    public static LedgerService getInstance() { return instance; }

    public void postDoubleEntry(String debitWalletId, String creditWalletId, long amount, String transactionId) {
        entries.add(new LedgerEntry(debitWalletId, EntryType.DEBIT, amount, transactionId));
        entries.add(new LedgerEntry(creditWalletId, EntryType.CREDIT, amount, transactionId));
    }

    public List<LedgerEntry> getEntriesForWallet(String walletId) {
        return entries.stream().filter(e -> e.getWalletId().equals(walletId)).collect(Collectors.toList());
    }

    public List<LedgerEntry> getEntriesForTransaction(String transactionId) {
        return entries.stream().filter(e -> e.getTransactionId().equals(transactionId)).collect(Collectors.toList());
    }
}
```

### 4.5 Chain of Responsibility — Transaction Validation

```java
public abstract class TransactionValidator {
    protected TransactionValidator next;
    public TransactionValidator setNext(TransactionValidator next) { this.next = next; return next; }

    public final void validate(Transaction txn) {
        doValidate(txn);
        if (next != null) next.validate(txn);
    }
    protected abstract void doValidate(Transaction txn);
}

public class BalanceValidator extends TransactionValidator {
    protected void doValidate(Transaction txn) {
        if (txn.getFromWallet() != null && txn.getFromWallet().getBalance() < txn.getTotalDebitAmount()) {
            throw new TransactionRejectedException("Insufficient wallet balance");
        }
    }
}

public class KYCLimitValidator extends TransactionValidator {
    protected void doValidate(Transaction txn) {
        // e.g. non-KYC wallets capped at ₹10,000 balance / ₹5,000 per transaction
        long maxPerTxn = txn.getFromWallet().isKycVerified() ? 200_00000L : 5_000_00L; // paise
        if (txn.getAmount() > maxPerTxn) {
            throw new TransactionRejectedException("Amount exceeds KYC tier limit");
        }
    }
}

public class DailyLimitValidator extends TransactionValidator {
    protected void doValidate(Transaction txn) {
        long todaysTotal = txn.getFromWallet().getTodaysDebitTotal(); // hypothetical accessor
        if (todaysTotal + txn.getAmount() > 100_000_00L) { // ₹1,00,000/day
            throw new TransactionRejectedException("Daily transaction limit exceeded");
        }
    }
}

public class FraudCheckValidator extends TransactionValidator {
    protected void doValidate(Transaction txn) {
        if (isSuspicious(txn)) throw new TransactionRejectedException("Flagged as potentially fraudulent");
    }
    private boolean isSuspicious(Transaction txn) {
        return false; // placeholder for velocity checks, device fingerprinting, etc.
    }
}

public class ValidationChainBuilder {
    public static TransactionValidator build() {
        TransactionValidator kyc = new KYCLimitValidator();
        kyc.setNext(new BalanceValidator())
           .setNext(new DailyLimitValidator())
           .setNext(new FraudCheckValidator());
        return kyc;
    }
}

class TransactionRejectedException extends RuntimeException {
    public TransactionRejectedException(String msg) { super(msg); }
}
```

### 4.6 Strategy — Funding Source (for top-ups)

```java
public interface FundingSourceStrategy {
    boolean pullFunds(long amount); // debits external source, credits wallet on success
}

public class BankTransferFunding implements FundingSourceStrategy {
    private final String accountNumber;
    public BankTransferFunding(String accountNumber) { this.accountNumber = accountNumber; }
    public boolean pullFunds(long amount) {
        System.out.println("NEFT/IMPS pull of ₹" + (amount / 100.0) + " from " + accountNumber);
        return true; // call bank API in reality
    }
}

public class CardFunding implements FundingSourceStrategy {
    private final String cardToken;
    public CardFunding(String cardToken) { this.cardToken = cardToken; }
    public boolean pullFunds(long amount) {
        System.out.println("Charging card " + cardToken + " for ₹" + (amount / 100.0));
        return true;
    }
}

public class UPIFunding implements FundingSourceStrategy {
    private final String vpa;
    public UPIFunding(String vpa) { this.vpa = vpa; }
    public boolean pullFunds(long amount) {
        System.out.println("UPI collect request sent to " + vpa);
        return true;
    }
}
```

### 4.7 Decorator — Fee Calculation

```java
public interface FeeCalculator {
    long calculateTotalDeduction(long baseAmount); // returns baseAmount + fees, in paise
}

public class NoFeeCalculator implements FeeCalculator {
    public long calculateTotalDeduction(long baseAmount) { return baseAmount; }
}

public abstract class FeeDecorator implements FeeCalculator {
    protected final FeeCalculator wrapped;
    protected FeeDecorator(FeeCalculator wrapped) { this.wrapped = wrapped; }
}

public class PlatformFeeDecorator extends FeeDecorator {
    private final double feePercent; // e.g. 0.02 = 2%
    public PlatformFeeDecorator(FeeCalculator wrapped, double feePercent) {
        super(wrapped); this.feePercent = feePercent;
    }
    public long calculateTotalDeduction(long baseAmount) {
        long fee = (long) (baseAmount * feePercent);
        return wrapped.calculateTotalDeduction(baseAmount) + fee;
    }
}

public class GSTDecorator extends FeeDecorator {
    private static final double GST_RATE = 0.18; // applied on the fee, not principal
    private final double feePercent;
    public GSTDecorator(FeeCalculator wrapped, double feePercent) { super(wrapped); this.feePercent = feePercent; }
    public long calculateTotalDeduction(long baseAmount) {
        long fee = (long) (baseAmount * feePercent);
        long gst = (long) (fee * GST_RATE);
        return wrapped.calculateTotalDeduction(baseAmount) + gst;
    }
}
```

### 4.8 Observer — Transaction event reactions

```java
public interface TransactionObserver {
    void onStateChange(Transaction txn, String eventType);
}

public class NotificationService implements TransactionObserver {
    public void onStateChange(Transaction txn, String eventType) {
        System.out.println("[Push] Txn " + txn.getId() + " -> " + eventType);
    }
}

public class LedgerWriter implements TransactionObserver {
    public void onStateChange(Transaction txn, String eventType) {
        if (eventType.equals("SUCCESS")) {
            LedgerService.getInstance().postDoubleEntry(
                    txn.getFromWallet() != null ? txn.getFromWallet().getId() : "EXTERNAL",
                    txn.getToWallet().getId(),
                    txn.getAmount(), txn.getId());
        } else if (eventType.equals("REVERSED")) {
            // reverse ledger entries: swap debit/credit
            LedgerService.getInstance().postDoubleEntry(
                    txn.getToWallet().getId(),
                    txn.getFromWallet() != null ? txn.getFromWallet().getId() : "EXTERNAL",
                    txn.getAmount(), txn.getId() + "-REV");
        }
    }
}

public class FraudAnalyticsLogger implements TransactionObserver {
    public void onStateChange(Transaction txn, String eventType) {
        // feed into fraud/ML pipeline
    }
}
```

### 4.9 State pattern — Transaction lifecycle

```java
public interface TransactionState {
    void process(Transaction txn);
    void succeed(Transaction txn);
    void fail(Transaction txn, String reason);
    void reverse(Transaction txn);
    String name();
}

public class InitiatedState implements TransactionState {
    public void process(Transaction txn) { txn.setState(new ProcessingState()); }
    public void succeed(Transaction txn) { throw new IllegalStateException("Must process first"); }
    public void fail(Transaction txn, String reason) { txn.setState(new FailedState()); txn.notifyObservers("FAILED"); }
    public void reverse(Transaction txn) { throw new IllegalStateException("Not yet successful"); }
    public String name() { return "INITIATED"; }
}

public class ProcessingState implements TransactionState {
    public void process(Transaction txn) { throw new IllegalStateException("Already processing"); }
    public void succeed(Transaction txn) { txn.setState(new SuccessState()); txn.notifyObservers("SUCCESS"); }
    public void fail(Transaction txn, String reason) { txn.setState(new FailedState()); txn.notifyObservers("FAILED"); }
    public void reverse(Transaction txn) { throw new IllegalStateException("Not yet successful"); }
    public String name() { return "PROCESSING"; }
}

public class SuccessState implements TransactionState {
    public void process(Transaction txn) { throw new IllegalStateException("Already succeeded"); }
    public void succeed(Transaction txn) { throw new IllegalStateException("Already succeeded"); }
    public void fail(Transaction txn, String reason) { throw new IllegalStateException("Already succeeded"); }
    public void reverse(Transaction txn) {
        txn.setState(new ReversedState());
        txn.notifyObservers("REVERSED");
    }
    public String name() { return "SUCCESS"; }
}

public class FailedState implements TransactionState {
    public void process(Transaction txn) { throw new IllegalStateException("Transaction failed"); }
    public void succeed(Transaction txn) { throw new IllegalStateException("Transaction failed"); }
    public void fail(Transaction txn, String reason) { throw new IllegalStateException("Already failed"); }
    public void reverse(Transaction txn) { throw new IllegalStateException("Cannot reverse a failed transaction"); }
    public String name() { return "FAILED"; }
}

public class ReversedState implements TransactionState {
    public void process(Transaction txn) { throw new IllegalStateException("Transaction reversed"); }
    public void succeed(Transaction txn) { throw new IllegalStateException("Transaction reversed"); }
    public void fail(Transaction txn, String reason) { throw new IllegalStateException("Transaction reversed"); }
    public void reverse(Transaction txn) { throw new IllegalStateException("Already reversed"); }
    public String name() { return "REVERSED"; }
}
```

### 4.10 Transaction — Command + Context + Subject

```java
public class Transaction {
    private final String id;
    private final TransactionType type;
    private final Wallet fromWallet; // null for top-up (external funding source)
    private final Wallet toWallet;
    private final long amount; // base amount in paise, excluding fees
    private final FeeCalculator feeCalculator;
    private final FundingSourceStrategy fundingSource; // null unless TOP_UP
    private final String idempotencyKey;

    private TransactionState state = new InitiatedState();
    private final List<TransactionObserver> observers = new ArrayList<>();

    private Transaction(Builder b) {
        this.id = b.id; this.type = b.type; this.fromWallet = b.fromWallet; this.toWallet = b.toWallet;
        this.amount = b.amount; this.feeCalculator = b.feeCalculator;
        this.fundingSource = b.fundingSource; this.idempotencyKey = b.idempotencyKey;
    }

    public void subscribe(TransactionObserver o) { observers.add(o); }
    void notifyObservers(String eventType) { for (TransactionObserver o : observers) o.onStateChange(this, eventType); }
    void setState(TransactionState s) { this.state = s; }

    public long getTotalDebitAmount() { return feeCalculator.calculateTotalDeduction(amount); }

    /** Command.execute() — performs the actual money movement under lock. */
    public void execute() {
        state.process(this);

        List<String> walletIds = new ArrayList<>();
        if (fromWallet != null) walletIds.add(fromWallet.getId());
        walletIds.add(toWallet.getId());

        List<ReentrantLock> locks = WalletLockManager.getInstance().acquireLocks(walletIds);
        try {
            if (type == TransactionType.TOP_UP) {
                boolean pulled = fundingSource.pullFunds(amount);
                if (!pulled) { state.fail(this, "Funding source declined"); return; }
                toWallet.credit(amount);
            } else {
                long totalDebit = getTotalDebitAmount();
                fromWallet.debit(totalDebit);
                toWallet.credit(amount); // fees go to platform, not counted in recipient credit
            }
            state.succeed(this);
        } catch (InsufficientBalanceException e) {
            state.fail(this, e.getMessage());
        } finally {
            WalletLockManager.getInstance().releaseLocks(locks);
        }
    }

    /** Command.reverse() — refund, the inverse operation. */
    public void reverse() {
        List<String> walletIds = new ArrayList<>();
        if (fromWallet != null) walletIds.add(fromWallet.getId());
        walletIds.add(toWallet.getId());

        List<ReentrantLock> locks = WalletLockManager.getInstance().acquireLocks(walletIds);
        try {
            toWallet.debit(amount);
            if (fromWallet != null) fromWallet.credit(getTotalDebitAmount());
            state.reverse(this);
        } finally {
            WalletLockManager.getInstance().releaseLocks(locks);
        }
    }

    // getters
    public String getId() { return id; }
    public Wallet getFromWallet() { return fromWallet; }
    public Wallet getToWallet() { return toWallet; }
    public long getAmount() { return amount; }
    public String getStateName() { return state.name(); }
    public String getIdempotencyKey() { return idempotencyKey; }

    public static class Builder {
        private String id = UUID.randomUUID().toString();
        private TransactionType type; private Wallet fromWallet, toWallet;
        private long amount; private FeeCalculator feeCalculator = new NoFeeCalculator();
        private FundingSourceStrategy fundingSource; private String idempotencyKey;

        public Builder type(TransactionType t) { this.type = t; return this; }
        public Builder fromWallet(Wallet w) { this.fromWallet = w; return this; }
        public Builder toWallet(Wallet w) { this.toWallet = w; return this; }
        public Builder amount(long a) { this.amount = a; return this; }
        public Builder feeCalculator(FeeCalculator f) { this.feeCalculator = f; return this; }
        public Builder fundingSource(FundingSourceStrategy f) { this.fundingSource = f; return this; }
        public Builder idempotencyKey(String k) { this.idempotencyKey = k; return this; }
        public Transaction build() { return new Transaction(this); }
    }
}
```

### 4.11 Factory Method + WalletService — orchestration entry point

```java
public class TransactionFactory {
    public static Transaction createP2PTransfer(Wallet from, Wallet to, long amount, String idempotencyKey) {
        return new Transaction.Builder()
                .type(TransactionType.P2P_TRANSFER)
                .fromWallet(from).toWallet(to).amount(amount)
                .feeCalculator(new NoFeeCalculator()) // P2P is typically fee-free
                .idempotencyKey(idempotencyKey)
                .build();
    }

    public static Transaction createMerchantPayment(Wallet from, Wallet merchantWallet, long amount, String idempotencyKey) {
        FeeCalculator fee = new GSTDecorator(new PlatformFeeDecorator(new NoFeeCalculator(), 0.02), 0.02);
        return new Transaction.Builder()
                .type(TransactionType.MERCHANT_PAYMENT)
                .fromWallet(from).toWallet(merchantWallet).amount(amount)
                .feeCalculator(fee)
                .idempotencyKey(idempotencyKey)
                .build();
    }

    public static Transaction createTopUp(Wallet wallet, long amount, FundingSourceStrategy source, String idempotencyKey) {
        return new Transaction.Builder()
                .type(TransactionType.TOP_UP)
                .toWallet(wallet).amount(amount)
                .fundingSource(source)
                .idempotencyKey(idempotencyKey)
                .build();
    }
}

public class WalletService {
    private final TransactionValidator validationChain = ValidationChainBuilder.build();
    // in-memory registry for demo purposes; production = DB lookup by ID
    private final ConcurrentHashMap<String, Transaction> transactionsByIdempotencyKey = new ConcurrentHashMap<>();

    public Transaction submit(Transaction txn) {
        String existingTxnId = IdempotencyKeyStore.getInstance()
                .checkAndReserve(txn.getIdempotencyKey(), txn.getId());

        if (existingTxnId != null) {
            // duplicate request — return the original transaction's outcome, don't reprocess
            return transactionsByIdempotencyKey.get(txn.getIdempotencyKey());
        }

        transactionsByIdempotencyKey.put(txn.getIdempotencyKey(), txn);

        txn.subscribe(new NotificationService());
        txn.subscribe(new LedgerWriter());
        txn.subscribe(new FraudAnalyticsLogger());

        try {
            if (txn.getFromWallet() != null) validationChain.validate(txn); // top-ups skip balance/limit checks on sender
            txn.execute();
        } catch (TransactionRejectedException e) {
            txn.notifyObservers("REJECTED");
        }
        return txn;
    }

    public Transaction refund(Transaction original) {
        original.reverse();
        return original;
    }
}
```

### 4.12 Putting it together

```java
public class WalletDemo {
    public static void main(String[] args) {
        Wallet alice = new Wallet("W1", "alice", 500_00); // ₹500
        Wallet bob = new Wallet("W2", "bob", 0);
        Wallet merchant = new Wallet("W3", "merchant-xyz", 0);

        WalletService service = new WalletService();

        // Top-up bob's wallet from bank
        Transaction topUp = TransactionFactory.createTopUp(
                bob, 1000_00, new BankTransferFunding("HDFC-XXXX1234"), "idem-key-001");
        service.submit(topUp);

        // P2P transfer alice -> bob
        Transaction p2p = TransactionFactory.createP2PTransfer(alice, bob, 100_00, "idem-key-002");
        service.submit(p2p);

        // Merchant payment bob -> merchant (with platform fee + GST)
        Transaction payment = TransactionFactory.createMerchantPayment(bob, merchant, 200_00, "idem-key-003");
        service.submit(payment);

        System.out.println("Alice balance: " + alice.getBalance());
        System.out.println("Bob balance: " + bob.getBalance());
        System.out.println("Merchant balance: " + merchant.getBalance());

        // Refund the merchant payment
        service.refund(payment);
        System.out.println("Bob balance after refund: " + bob.getBalance());

        // Retry with same idempotency key -> returns original result, doesn't double-process
        Transaction duplicateAttempt = TransactionFactory.createP2PTransfer(alice, bob, 100_00, "idem-key-002");
        Transaction result = service.submit(duplicateAttempt);
        System.out.println("Duplicate submit returned state: " + result.getStateName());
    }
}
```

---

## 5. Why this shape holds up under follow-ups

- **"Prevent double-debit on network retry"** → `IdempotencyKeyStore` is exactly this seam; production backing is a DB unique constraint, contract unchanged.
- **"Prevent race condition when two transactions touch the same wallet simultaneously"** → `WalletLockManager`'s ordered lock acquisition is the core answer; interviewers often probe deadlock scenarios here — the sorted-wallet-ID lock ordering is the standard fix.
- **"Add split payments (bill split among N people)"** → new `TransactionType` + a coordinating class that creates N `Transaction` commands, one per participant — core `Transaction`/lock/ledger logic untouched.
- **"Add scheduled/recurring payments (subscriptions)"** → combine with the earlier **Delayed Job Scheduler** design: a recurring `TriggerStrategy` job whose `doExecute()` calls `WalletService.submit(...)`.
- **"Support partial refunds"** → extend `Transaction.reverse()` to accept an amount parameter instead of always reversing in full; State pattern already isolates where this belongs.
- **"Reconcile ledger against bank settlement files"** → `LedgerService`'s append-only double-entry log is precisely what makes reconciliation possible — this is why balance is never trusted as a standalone mutable field.
- **"Add multi-currency wallets"** → `Wallet` gains a `currency` field; `FeeCalculator`/`Transaction` would need an FX-rate strategy — isolated addition, doesn't touch locking/idempotency/ledger mechanics.

---

Want me to extend this with **a distributed locking implementation (DB `SELECT FOR UPDATE` or Redis Redlock) for multi-server deployment, saga/2-phase-commit for cross-wallet-service consistency, KYC tier state machine, or a reconciliation engine comparing ledger vs bank settlement**, or move to a different LLD problem?