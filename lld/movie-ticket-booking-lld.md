# Movie Ticket Booking System (BookMyShow) — LLD

## 1. Requirements

**Functional**
- Browse movies by city → theaters → shows (date/time/screen).
- View seat layout per show (seat types: Silver/Gold/Premium/Recliner) with real-time availability.
- Select seats and hold them temporarily (e.g., 5-10 min) while user completes payment — must prevent two users from booking the same seat.
- Confirm booking after successful payment; release seats automatically if hold expires or payment fails.
- Pricing varies by seat type, show timing (weekday/weekend/prime-time), and applicable discounts/coupons.
- Multiple payment methods.
- Cancel booking with refund rules based on time-before-show.
- Notify user on booking confirmed / seat-hold-expiring / cancellation.

**Non-functional**
- **Concurrency is the core challenge**: many users hitting the same show simultaneously must never double-book a seat.
- Seat hold/expiry must be reliable even under load (TTL-based).
- Pricing rules and payment methods pluggable without touching booking core.
- Read-heavy seat-layout queries shouldn't contend with the booking write path more than necessary.

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **State** | `SeatState`: `AvailableState`, `LockedState`, `BookedState`; `BookingState`: `InitiatedState`, `ConfirmedState`, `CancelledState`, `ExpiredState` | Seat and booking behavior depend entirely on current state — a seat can only move Available→Locked→Booked (or back to Available on expiry/cancel). Prevents illegal transitions like booking an already-booked seat. |
| **Singleton** | `SeatLockManager` | Must be the single source of truth for "who currently holds a lock on seat X" across the whole app — this is exactly the concurrency chokepoint, so there can only be one. |
| **Strategy** | `PricingStrategy` (`WeekdayPricing`, `WeekendPricing`, `PrimeTimePricing`); `PaymentStrategy` (`CardPayment`, `WalletPayment`, `UPIPayment`) | Price computation and payment processing both vary independently of booking logic — isolating them keeps `BookingService` stable when new pricing rules or payment gateways are added. |
| **Decorator** | `PriceCalculator` wrapped by `ConvenienceFeeDecorator`, `CouponDiscountDecorator` | Final price = base seat price + optional convenience fee − optional discount, stacked independently, avoiding a combinatorial class explosion. |
| **Observer** | `Booking` (Subject) notifies `BookingObserver`: `UserNotifier`, `SeatReleaseScheduler`, `BookingAnalyticsLogger` | One booking event (confirmed/expired/cancelled) → multiple independent reactions (notify user, release seat lock, log analytics) without `Booking` knowing about any of them. |
| **Factory Method** | `PricingStrategyFactory.get(showTime)` | Encapsulates which pricing strategy applies for a given show's date/time. |
| **Builder** | `Booking.Builder`, `Show.Builder` | Many optional fields (coupon code, seat list, payment details). |
| **Template Method (light)** | `SeatLockManager.lockSeats()` defines lock-attempt-then-rollback-on-partial-failure skeleton | Ensures the "all-or-nothing" seat locking algorithm is consistent and can't be half-implemented differently in different call sites. |

**SOLID**
- **S**: `SeatLockManager` only manages locks; `BookingService` only orchestrates booking flow; `PriceCalculator` only computes price; `PaymentStrategy` only processes payment.
- **O**: New seat type → extend enum + pricing map, no core logic touched. New payment method → new `PaymentStrategy`. New discount rule → new decorator.
- **L**: Any `SeatState`/`BookingState` substitutable wherever delegated to; any `PricingStrategy`/`PaymentStrategy` substitutable at their call sites.
- **I**: `BookingObserver` exposes only `onBookingEvent`; `PricingStrategy` exposes only `calculateBasePrice` — no bloated interfaces.
- **D**: `BookingService` depends on `SeatLockManager`, `PricingStrategy`, `PaymentStrategy` abstractions injected/looked-up, not concrete implementations.

---

## 3. Class Diagram (textual)

```
┌──────────────────┐        ┌──────────────────────────┐
│   SeatState          │◀──────│  Seat (Context)             │
│ (State interface)     │       │ - state: SeatState            │
│ + lock() / release()   │      │ - seatType, seatNumber          │
│ + book()                │     └──────────────────────────┘
└────────▲──────────────┘
  ┌──────┼──────┬────────────┐
Available  Locked  Booked
 State     State   State

┌──────────────────┐        ┌──────────────────────────┐
│  BookingState        │      │  Booking (Context/Subject)   │
│ (State interface)     │◀─────│ - state: BookingState          │
│ + confirm()/cancel()/  │     │ - seats, show, user, payment    │
│   expire()              │    │ - observers: List<Obs>           │
└────────▲──────────────┘     └──────────────────────────┘
  ┌──────┼─────┬───────────┬─────────┐
Initiated Confirmed Cancelled  Expired
 State     State     State     State

┌────────────────────────┐        ┌───────────────────┐
│  SeatLockManager            │      │  BookingObserver       │
│  (Singleton)                  │     │ + onBookingEvent(evt)    │
│  + lockSeats(showId, seats,   │     └──────────▲───────────┘
│      userId): boolean          │       ┌────────┼──────────┬───────────────┐
│  + releaseSeats(showId, seats)│   UserNotifier SeatReleaseScheduler BookingAnalyticsLogger
│  + confirmSeats(showId, seats)│
└────────────────────────┘

┌────────────────────┐      ┌────────────────────┐
│  PricingStrategy       │    │  PaymentStrategy       │
│ (Strategy interface)     │  │ (Strategy interface)     │
│ + calculateBasePrice()    │ │ + processPayment(amt)      │
└──────────▲───────────┘    └──────────▲───────────┘
   ┌───────┼────────┐          ┌───────┼────────┐
Weekday  Weekend  PrimeTime   Card   Wallet    UPI

┌────────────────────┐
│  PriceCalculator (base)│◀── decorated by ──┐
└────────────────────┘                     │
      ┌──────────────────────┬─────────────┴────────┐
ConvenienceFeeDecorator  CouponDiscountDecorator

┌────────────┐   ┌────────────┐   ┌────────────┐   ┌────────────┐
│  Movie          │  │  Theater        │  │  Screen        │  │  Show           │
└────────────┘   └────────────┘   └────────────┘   └────────────┘

┌────────────────────┐        ┌─────────────────────┐
│  BookingService         │      │  BookingFactory          │
│  + initiateBooking()       │   │  + createBooking()          │
│  + confirmBooking()         │  └─────────────────────┘
└────────────────────┘
```

---

## 4. Code (Java)

### 4.1 Core entities

```java
public enum SeatType { SILVER, GOLD, PREMIUM, RECLINER }

public class Movie {
    private final String id, title, language;
    private final int durationMin;
    // getters omitted
}

public class Theater {
    private final String id, name, city;
    private final List<Screen> screens;
    // getters omitted
}

public class Screen {
    private final String id;
    private final List<Seat> layout; // fixed physical seats
}

public class Show {
    private final String id;
    private final Movie movie;
    private final Screen screen;
    private final LocalDateTime showTime;
    private final Map<String, Seat> seatsBySeatNumber; // per-show seat state instances
    // getters omitted
}
```

### 4.2 State pattern — Seat lifecycle

```java
public interface SeatState {
    void lock(Seat seat, String userId);
    void release(Seat seat);
    void book(Seat seat);
    String name();
}

public class AvailableState implements SeatState {
    public void lock(Seat seat, String userId) {
        seat.setLockedBy(userId);
        seat.setLockExpiry(System.currentTimeMillis() + 10 * 60 * 1000); // 10 min hold
        seat.setState(new LockedState());
    }
    public void release(Seat seat) { throw new IllegalStateException("Seat already available"); }
    public void book(Seat seat) { throw new IllegalStateException("Seat must be locked before booking"); }
    public String name() { return "AVAILABLE"; }
}

public class LockedState implements SeatState {
    public void lock(Seat seat, String userId) { throw new IllegalStateException("Seat already locked"); }
    public void release(Seat seat) {
        seat.setLockedBy(null);
        seat.setState(new AvailableState());
    }
    public void book(Seat seat) { seat.setState(new BookedState()); }
    public String name() { return "LOCKED"; }
}

public class BookedState implements SeatState {
    public void lock(Seat seat, String userId) { throw new IllegalStateException("Seat already booked"); }
    public void release(Seat seat) { throw new IllegalStateException("Cannot release a booked seat"); }
    public void book(Seat seat) { throw new IllegalStateException("Already booked"); }
    public String name() { return "BOOKED"; }
}

public class Seat {
    private final String seatNumber;
    private final SeatType type;
    private SeatState state = new AvailableState();
    private String lockedBy;
    private long lockExpiry;

    public Seat(String seatNumber, SeatType type) { this.seatNumber = seatNumber; this.type = type; }

    void setState(SeatState s) { this.state = s; }
    public void lock(String userId) { state.lock(this, userId); }
    public void release() { state.release(this); }
    public void book() { state.book(this); }
    public String getStateName() { return state.name(); }

    public boolean isLockExpired() { return getStateName().equals("LOCKED") && System.currentTimeMillis() > lockExpiry; }

    void setLockedBy(String u) { this.lockedBy = u; }
    void setLockExpiry(long t) { this.lockExpiry = t; }
    public String getLockedBy() { return lockedBy; }
    public String getSeatNumber() { return seatNumber; }
    public SeatType getType() { return type; }
}
```

### 4.3 Singleton — SeatLockManager (the concurrency chokepoint)

This is the class that actually prevents double booking. Uses per-show locking with all-or-nothing semantics (Template-Method-ish skeleton) so a user either gets **all** requested seats or **none**, with no partial locks left dangling.

```java
public class SeatLockManager {
    private static volatile SeatLockManager instance;
    // one lock object per show avoids global contention across unrelated shows
    private final ConcurrentHashMap<String, Object> showLocks = new ConcurrentHashMap<>();

    private SeatLockManager() {}

    public static SeatLockManager getInstance() {
        if (instance == null) {
            synchronized (SeatLockManager.class) {
                if (instance == null) instance = new SeatLockManager();
            }
        }
        return instance;
    }

    public boolean lockSeats(Show show, List<String> seatNumbers, String userId) {
        Object monitor = showLocks.computeIfAbsent(show.getId(), id -> new Object());

        synchronized (monitor) {
            // pre-check: expire stale locks first
            for (String sn : seatNumbers) {
                Seat seat = show.getSeatsBySeatNumber().get(sn);
                if (seat.isLockExpired()) seat.release();
            }

            // all-or-nothing check
            for (String sn : seatNumbers) {
                Seat seat = show.getSeatsBySeatNumber().get(sn);
                if (!seat.getStateName().equals("AVAILABLE")) {
                    return false; // fail fast, nothing locked yet
                }
            }

            // safe to lock all
            for (String sn : seatNumbers) {
                show.getSeatsBySeatNumber().get(sn).lock(userId);
            }
            return true;
        }
    }

    public void releaseSeats(Show show, List<String> seatNumbers) {
        Object monitor = showLocks.computeIfAbsent(show.getId(), id -> new Object());
        synchronized (monitor) {
            for (String sn : seatNumbers) {
                Seat seat = show.getSeatsBySeatNumber().get(sn);
                if (seat.getStateName().equals("LOCKED")) seat.release();
            }
        }
    }

    public void confirmSeats(Show show, List<String> seatNumbers) {
        Object monitor = showLocks.computeIfAbsent(show.getId(), id -> new Object());
        synchronized (monitor) {
            for (String sn : seatNumbers) {
                show.getSeatsBySeatNumber().get(sn).book();
            }
        }
    }
}
```

> In a real multi-server deployment, `synchronized` becomes a **distributed lock** (Redis `SETNX`/Redlock, or DB row-level `SELECT ... FOR UPDATE`) — the interface (`lockSeats`/`releaseSeats`/`confirmSeats`) stays identical, only the implementation swaps, because callers depend on this abstraction, not on `synchronized` directly.

### 4.4 Strategy — Pricing

```java
public interface PricingStrategy {
    double calculateBasePrice(SeatType type);
}

public class WeekdayPricing implements PricingStrategy {
    private static final Map<SeatType, Double> RATES = Map.of(
            SeatType.SILVER, 150.0, SeatType.GOLD, 250.0,
            SeatType.PREMIUM, 350.0, SeatType.RECLINER, 500.0);
    public double calculateBasePrice(SeatType type) { return RATES.get(type); }
}

public class WeekendPricing implements PricingStrategy {
    private static final Map<SeatType, Double> RATES = Map.of(
            SeatType.SILVER, 200.0, SeatType.GOLD, 300.0,
            SeatType.PREMIUM, 450.0, SeatType.RECLINER, 650.0);
    public double calculateBasePrice(SeatType type) { return RATES.get(type); }
}

public class PrimeTimePricing implements PricingStrategy {
    private static final Map<SeatType, Double> RATES = Map.of(
            SeatType.SILVER, 220.0, SeatType.GOLD, 320.0,
            SeatType.PREMIUM, 480.0, SeatType.RECLINER, 700.0);
    public double calculateBasePrice(SeatType type) { return RATES.get(type); }
}

public class PricingStrategyFactory {
    public static PricingStrategy get(LocalDateTime showTime) {
        DayOfWeek day = showTime.getDayOfWeek();
        int hour = showTime.getHour();
        boolean isWeekend = day == DayOfWeek.SATURDAY || day == DayOfWeek.SUNDAY;
        boolean isPrimeTime = hour >= 18 && hour <= 22;

        if (isPrimeTime) return new PrimeTimePricing();
        if (isWeekend) return new WeekendPricing();
        return new WeekdayPricing();
    }
}
```

### 4.5 Decorator — final price (fees/discounts)

```java
public interface PriceCalculator {
    double calculate(List<Seat> seats);
}

public class BasePriceCalculator implements PriceCalculator {
    private final PricingStrategy strategy;
    public BasePriceCalculator(PricingStrategy strategy) { this.strategy = strategy; }
    public double calculate(List<Seat> seats) {
        return seats.stream().mapToDouble(s -> strategy.calculateBasePrice(s.getType())).sum();
    }
}

public abstract class PriceDecorator implements PriceCalculator {
    protected final PriceCalculator wrapped;
    protected PriceDecorator(PriceCalculator wrapped) { this.wrapped = wrapped; }
}

public class ConvenienceFeeDecorator extends PriceDecorator {
    private final double feePerSeat;
    public ConvenienceFeeDecorator(PriceCalculator wrapped, double feePerSeat) {
        super(wrapped); this.feePerSeat = feePerSeat;
    }
    public double calculate(List<Seat> seats) {
        return wrapped.calculate(seats) + (feePerSeat * seats.size());
    }
}

public class CouponDiscountDecorator extends PriceDecorator {
    private final double discountPercent;
    public CouponDiscountDecorator(PriceCalculator wrapped, double discountPercent) {
        super(wrapped); this.discountPercent = discountPercent;
    }
    public double calculate(List<Seat> seats) {
        double price = wrapped.calculate(seats);
        return price - (price * discountPercent);
    }
}
```

### 4.6 Strategy — Payment

```java
public interface PaymentStrategy {
    boolean processPayment(double amount);
}

public class CardPayment implements PaymentStrategy {
    private final String cardToken;
    public CardPayment(String cardToken) { this.cardToken = cardToken; }
    public boolean processPayment(double amount) {
        System.out.println("Charging card for ₹" + amount);
        return true;
    }
}

public class WalletPayment implements PaymentStrategy {
    public boolean processPayment(double amount) { return true; }
}

public class UPIPayment implements PaymentStrategy {
    public boolean processPayment(double amount) { return true; }
}
```

### 4.7 Observer — Booking event reactions

```java
public interface BookingObserver {
    void onBookingEvent(Booking booking, String eventType); // CONFIRMED, EXPIRED, CANCELLED
}

public class UserNotifier implements BookingObserver {
    public void onBookingEvent(Booking booking, String eventType) {
        System.out.println("[Notify] Booking " + booking.getId() + " -> " + eventType);
    }
}

public class SeatReleaseScheduler implements BookingObserver {
    public void onBookingEvent(Booking booking, String eventType) {
        if (eventType.equals("EXPIRED") || eventType.equals("CANCELLED")) {
            SeatLockManager.getInstance().releaseSeats(booking.getShow(), booking.getSeatNumbers());
        }
    }
}

public class BookingAnalyticsLogger implements BookingObserver {
    public void onBookingEvent(Booking booking, String eventType) {
        // write to analytics pipeline
    }
}
```

### 4.8 State pattern — Booking lifecycle

```java
public interface BookingState {
    void confirm(Booking booking);
    void cancel(Booking booking);
    void expire(Booking booking);
    String name();
}

public class InitiatedState implements BookingState {
    public void confirm(Booking booking) {
        SeatLockManager.getInstance().confirmSeats(booking.getShow(), booking.getSeatNumbers());
        booking.setState(new ConfirmedState());
        booking.notifyObservers("CONFIRMED");
    }
    public void cancel(Booking booking) {
        booking.setState(new CancelledState());
        booking.notifyObservers("CANCELLED");
    }
    public void expire(Booking booking) {
        booking.setState(new ExpiredState());
        booking.notifyObservers("EXPIRED");
    }
    public String name() { return "INITIATED"; }
}

public class ConfirmedState implements BookingState {
    public void confirm(Booking booking) { throw new IllegalStateException("Already confirmed"); }
    public void cancel(Booking booking) {
        // release seats back to pool (theater policy dependent)
        booking.setState(new CancelledState());
        booking.notifyObservers("CANCELLED");
    }
    public void expire(Booking booking) { throw new IllegalStateException("Cannot expire a confirmed booking"); }
    public String name() { return "CONFIRMED"; }
}

public class CancelledState implements BookingState {
    public void confirm(Booking booking) { throw new IllegalStateException("Booking cancelled"); }
    public void cancel(Booking booking) { throw new IllegalStateException("Already cancelled"); }
    public void expire(Booking booking) { throw new IllegalStateException("Booking cancelled"); }
    public String name() { return "CANCELLED"; }
}

public class ExpiredState implements BookingState {
    public void confirm(Booking booking) { throw new IllegalStateException("Booking expired"); }
    public void cancel(Booking booking) { throw new IllegalStateException("Booking expired"); }
    public void expire(Booking booking) { throw new IllegalStateException("Already expired"); }
    public String name() { return "EXPIRED"; }
}
```

### 4.9 Booking — Context + Subject

```java
public class Booking {
    private final String id;
    private final User user;
    private final Show show;
    private final List<String> seatNumbers;
    private final PaymentStrategy paymentStrategy;
    private double finalAmount;
    private BookingState state = new InitiatedState();
    private final List<BookingObserver> observers = new ArrayList<>();

    private Booking(Builder b) {
        this.id = b.id; this.user = b.user; this.show = b.show;
        this.seatNumbers = b.seatNumbers; this.paymentStrategy = b.paymentStrategy;
    }

    public void subscribe(BookingObserver o) { observers.add(o); }
    void notifyObservers(String eventType) {
        for (BookingObserver o : observers) o.onBookingEvent(this, eventType);
    }

    void setState(BookingState s) { this.state = s; }
    public void confirm() { state.confirm(this); }
    public void cancel() { state.cancel(this); }
    public void expire() { state.expire(this); }
    public String getStateName() { return state.name(); }

    public void setFinalAmount(double amt) { this.finalAmount = amt; }
    public double getFinalAmount() { return finalAmount; }
    public PaymentStrategy getPaymentStrategy() { return paymentStrategy; }
    public String getId() { return id; }
    public Show getShow() { return show; }
    public List<String> getSeatNumbers() { return seatNumbers; }

    public static class Builder {
        private String id; private User user; private Show show;
        private List<String> seatNumbers; private PaymentStrategy paymentStrategy;

        public Builder id(String id) { this.id = id; return this; }
        public Builder user(User u) { this.user = u; return this; }
        public Builder show(Show s) { this.show = s; return this; }
        public Builder seatNumbers(List<String> sn) { this.seatNumbers = sn; return this; }
        public Builder paymentStrategy(PaymentStrategy p) { this.paymentStrategy = p; return this; }
        public Booking build() { return new Booking(this); }
    }
}
```

### 4.10 BookingService — orchestration

```java
public class BookingService {

    public Booking initiateBooking(User user, Show show, List<String> seatNumbers,
                                    PaymentStrategy paymentStrategy, Double couponDiscount) {

        boolean locked = SeatLockManager.getInstance().lockSeats(show, seatNumbers, user.getId());
        if (!locked) {
            throw new RuntimeException("One or more selected seats are no longer available");
        }

        List<Seat> seats = seatNumbers.stream()
                .map(sn -> show.getSeatsBySeatNumber().get(sn))
                .collect(Collectors.toList());

        PricingStrategy pricingStrategy = PricingStrategyFactory.get(show.getShowTime());
        PriceCalculator calculator = new BasePriceCalculator(pricingStrategy);
        calculator = new ConvenienceFeeDecorator(calculator, 20.0);
        if (couponDiscount != null) calculator = new CouponDiscountDecorator(calculator, couponDiscount);

        double amount = calculator.calculate(seats);

        Booking booking = new Booking.Builder()
                .id(UUID.randomUUID().toString())
                .user(user).show(show).seatNumbers(seatNumbers)
                .paymentStrategy(paymentStrategy)
                .build();
        booking.setFinalAmount(amount);

        booking.subscribe(new UserNotifier());
        booking.subscribe(new SeatReleaseScheduler());
        booking.subscribe(new BookingAnalyticsLogger());

        // schedule auto-expiry if payment doesn't complete within hold window
        scheduleExpiry(booking, 10 * 60 * 1000);

        return booking;
    }

    public void completePayment(Booking booking) {
        boolean paid = booking.getPaymentStrategy().processPayment(booking.getFinalAmount());
        if (paid) {
            booking.confirm();
        } else {
            booking.cancel();
        }
    }

    private void scheduleExpiry(Booking booking, long delayMs) {
        // in production: delayed queue (Redis TTL keyspace notification, or scheduled executor)
        new Timer().schedule(new TimerTask() {
            public void run() {
                if (booking.getStateName().equals("INITIATED")) {
                    booking.expire();
                }
            }
        }, delayMs);
    }
}
```

### 4.11 Putting it together

```java
public class BookMyShowDemo {
    public static void main(String[] args) {
        Screen screen = new Screen(/* id, layout with Seat objects */);
        Show show = new Show(/* id, movie, screen, showTime */);

        User user = new User(/* ... */);
        BookingService bookingService = new BookingService();

        List<String> chosenSeats = List.of("G1", "G2");
        Booking booking = bookingService.initiateBooking(
                user, show, chosenSeats, new CardPayment("tok_abc"), 0.1);

        // user completes payment within hold window
        bookingService.completePayment(booking);
        // -> observers fire: seats marked BOOKED, user notified, analytics logged
    }
}
```

---

## 5. Why this shape holds up under follow-ups

- **"Prevent two users from booking the same seat under high concurrency"** → this is the design's centerpiece: `SeatLockManager`'s all-or-nothing locking (Singleton + per-show monitor) is exactly the mechanism; swapping to Redis/DB-based distributed locks changes only its internals.
- **"Add group discounts for 4+ tickets"** → another `PriceDecorator` — stacks with existing ones, no change elsewhere.
- **"Add food/beverage add-ons to a booking"** → extend `Booking` with an `addOns` list and a corresponding `PriceCalculator` decorator; core seat-locking flow untouched.
- **"Waitlist when show is full"** → new `WaitlistState`/service reacting to `SeatReleaseScheduler`'s release events (already an observer hook).
- **"Dynamic surge pricing for high-demand shows"** → new `PricingStrategy` implementation + factory branch.
- **"Refund percentage based on cancellation time"** → extend `ConfirmedState.cancel()` to compute a refund percentage; state pattern already isolates where this logic belongs.

---

Want me to extend this with **distributed locking implementation (Redis Redlock/SETNX with TTL), a waitlist/queue system for sold-out shows, seat-map rendering data structure, or a notification fan-out for "show about to start" reminders**, or move to a different LLD problem (Splitwise, Elevator, Chess, Library Management)?