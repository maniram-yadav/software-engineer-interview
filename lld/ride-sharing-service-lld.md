# Ride-Sharing Service (Uber/Lyft) — LLD

## 1. Requirements

**Functional**
- Rider requests a ride from pickup → drop location; system matches nearest available driver.
- Support multiple ride types (Mini, Sedan, SUV, Pool) with different fare rules.
- Real-time driver location tracking; efficient "find nearby drivers" queries.
- Trip lifecycle: Requested → Driver Assigned → Driver Arrived → In Progress → Completed / Cancelled.
- Fare calculation: base fare + distance + time + surge pricing + promos/coupons.
- Multiple payment methods (card, wallet, cash) processed at trip completion.
- Notify rider and driver at each state change (driver assigned, arrived, trip started, etc.).
- Rating system after trip completion.
- Cancellation with cancellation-fee rules depending on trip state.

**Non-functional**
- Driver matching must scale to "find K nearest drivers among millions" efficiently.
- Fare rules and matching strategy must be pluggable (new city, new pricing model) without touching core trip logic.
- Trip state transitions must be strictly controlled — no illegal jumps (e.g., can't complete a trip that was never started).

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **State** | `TripState` interface: `RequestedState`, `DriverAssignedState`, `ArrivedState`, `InProgressState`, `CompletedState`, `CancelledState` | Trip behavior and legal next-actions depend entirely on current state. Prevents illegal transitions (e.g., ending a trip that hasn't started) and removes giant conditionals. |
| **Strategy** | `DriverMatchingStrategy` (`NearestDriverStrategy`, `HighestRatedDriverStrategy`); `FareCalculationStrategy` (per `RideType`) | Matching logic and pricing logic both vary independently and change often (business/city-specific tuning) — isolating them means core `RideRequestManager`/`Trip` code never changes when pricing or matching algorithms are tweaked. |
| **Decorator** | `FareCalculator` wrapped by `SurgePricingDecorator`, `PromoDiscountDecorator` | Fare = base calculation + optional surge + optional promo, stacked independently. Decorator lets each concern be added/removed without a combinatorial explosion of fare classes. |
| **Observer** | `Trip` (Subject) notifies `TripObserver` (`RiderNotifier`, `DriverNotifier`, `TripAnalyticsLogger`) | One state change → multiple independent reactions (push notification to both parties, log analytics) without `Trip` knowing about any of them. |
| **Factory Method** | `VehicleFactory` / `RideTypeFactory` | Encapsulates which `FareCalculationStrategy` + vehicle constraints go with a given `RideType`. |
| **Singleton** | `DriverLocationService` | One central geo-index of all driver locations; must be a single source of truth for proximity queries. |
| **Strategy (again)** | `PaymentStrategy` (`CardPayment`, `WalletPayment`, `CashPayment`) | Payment method varies per rider/trip; `Trip` shouldn't know how each payment type actually processes. |
| **Builder** | `Trip.Builder`, `RideRequest.Builder` | Many optional fields (promo code, scheduled time, ride type). |

**SOLID**
- **S**: `Trip` orchestrates state; `FareCalculator` only computes fare; `DriverLocationService` only tracks/queries locations; `PaymentProcessor` only handles payment.
- **O**: New ride type → new `FareCalculationStrategy` + factory entry. New matching algorithm → new `DriverMatchingStrategy`. No existing class edited.
- **L**: Any `TripState` is substitutable wherever `Trip` delegates; any `PaymentStrategy` substitutable at checkout.
- **I**: `TripObserver` only exposes `onStateChange`; `DriverMatchingStrategy` only exposes `findDriver`; no bloated interfaces forcing unrelated implementations.
- **D**: `Trip` depends on `TripState`, `FareCalculationStrategy`, `PaymentStrategy` abstractions injected at creation — never on concrete classes.

---

## 3. Class Diagram (textual)

```
┌─────────────────┐         ┌───────────────────────┐
│   TripState        │◀───────│  Trip (Context/Subject)  │
│ (State interface)   │        │ - state: TripState        │
│ + requestDriver()    │       │ - fareCalculator            │
│ + arrive()            │      │ - paymentStrategy           │
│ + start()              │     │ - observers: List<Obs>      │
│ + complete()            │    │ + notifyObservers()          │
│ + cancel()               │   └───────────────────────┘
└────────▲────────────┘
         │
┌────────┼────────┬────────────┬──────────────┬───────────┐
Requested DriverAssigned  Arrived    InProgress   Completed  Cancelled
 State       State         State       State        State      State

┌─────────────────────────┐      ┌──────────────────────┐
│  DriverMatchingStrategy    │     │  FareCalculationStrategy│
│  (Strategy interface)       │    │  (Strategy interface)   │
│  + findDriver(location)      │   │  + calculate(trip)        │
└──────────▲──────────────┘      └───────────▲───────────┘
   ┌───────┼─────────┐                ┌───────┼──────────┐
NearestDriver  HighestRated       MiniFare  SedanFare  SUVFare  PoolFare

┌────────────────────────┐
│ FareCalculator (base)     │◀── decorated by ──┐
└────────────────────────┘                     │
        ┌───────────────────────────┬──────────┴──────────┐
   SurgePricingDecorator       PromoDiscountDecorator  (both implement same interface)

┌──────────────────┐        ┌────────────────────┐
│  TripObserver       │       │  PaymentStrategy      │
│ + onStateChange()    │      │ + processPayment(amt)  │
└──────────▲─────────┘       └──────────▲───────────┘
    ┌──────┼─────────┐          ┌───────┼────────┐
RiderNotifier DriverNotifier  CardPayment WalletPayment CashPayment

┌────────────────────────┐    ┌──────────────────────┐
│  DriverLocationService     │  │  RideRequestManager     │
│  (Singleton, geo-index)     │ │  + requestRide()          │
└────────────────────────┘    └──────────────────────┘

┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│  Driver         │  │   Rider         │  │  Vehicle        │
└──────────────┘   └──────────────┘   └──────────────┘
```

---

## 4. Code (Java)

### 4.1 Core entities

```java
public class Location {
    private final double lat, lng;
    public Location(double lat, double lng) { this.lat = lat; this.lng = lng; }

    public double distanceTo(Location other) {
        // Haversine formula (simplified placeholder)
        double dx = this.lat - other.lat, dy = this.lng - other.lng;
        return Math.sqrt(dx * dx + dy * dy) * 111; // approx km
    }
    public double getLat() { return lat; }
    public double getLng() { return lng; }
}

public enum RideType { MINI, SEDAN, SUV, POOL }
public enum DriverStatus { AVAILABLE, ON_TRIP, OFFLINE }

public class Vehicle {
    private final String plateNumber;
    private final RideType type;
    private final String model;
    // getters omitted
}

public class Driver {
    private final String id;
    private final String name;
    private Vehicle vehicle;
    private Location currentLocation;
    private DriverStatus status = DriverStatus.AVAILABLE;
    private double rating = 5.0;
    // getters/setters omitted
}

public class Rider {
    private final String id;
    private final String name;
    private double rating = 5.0;
    private String defaultPaymentMethodId;
    // getters/setters omitted
}
```

### 4.2 Singleton — DriverLocationService (geo-index)

In production this would be backed by a geohash/quad-tree/Redis GEO index; here a simplified linear-scan illustrates the seam clearly.

```java
public class DriverLocationService {
    private static volatile DriverLocationService instance;
    private final ConcurrentHashMap<String, Driver> availableDrivers = new ConcurrentHashMap<>();

    private DriverLocationService() {}

    public static DriverLocationService getInstance() {
        if (instance == null) {
            synchronized (DriverLocationService.class) {
                if (instance == null) instance = new DriverLocationService();
            }
        }
        return instance;
    }

    public void updateLocation(Driver driver, Location location) {
        driver.setCurrentLocation(location);
        if (driver.getStatus() == DriverStatus.AVAILABLE) {
            availableDrivers.put(driver.getId(), driver);
        }
    }

    public void markUnavailable(Driver driver) { availableDrivers.remove(driver.getId()); }
    public void markAvailable(Driver driver) { availableDrivers.put(driver.getId(), driver); }

    public List<Driver> findNearby(Location pickup, RideType type, double radiusKm) {
        return availableDrivers.values().stream()
                .filter(d -> d.getVehicle().getType() == type)
                .filter(d -> d.getCurrentLocation().distanceTo(pickup) <= radiusKm)
                .collect(Collectors.toList());
    }
}
```

### 4.3 Strategy — Driver Matching

```java
public interface DriverMatchingStrategy {
    Optional<Driver> findDriver(Location pickup, RideType type);
}

public class NearestDriverStrategy implements DriverMatchingStrategy {
    private final DriverLocationService locationService = DriverLocationService.getInstance();

    @Override
    public Optional<Driver> findDriver(Location pickup, RideType type) {
        return locationService.findNearby(pickup, type, 5.0).stream()
                .min(Comparator.comparingDouble(d -> d.getCurrentLocation().distanceTo(pickup)));
    }
}

public class HighestRatedDriverStrategy implements DriverMatchingStrategy {
    private final DriverLocationService locationService = DriverLocationService.getInstance();

    @Override
    public Optional<Driver> findDriver(Location pickup, RideType type) {
        return locationService.findNearby(pickup, type, 5.0).stream()
                .max(Comparator.comparingDouble(Driver::getRating));
    }
}
```

### 4.4 Strategy — Fare Calculation

```java
public interface FareCalculationStrategy {
    double calculateBaseFare(double distanceKm, double durationMin);
}

public class MiniFareStrategy implements FareCalculationStrategy {
    public double calculateBaseFare(double distanceKm, double durationMin) {
        return 30 + (distanceKm * 8) + (durationMin * 1.5);
    }
}

public class SedanFareStrategy implements FareCalculationStrategy {
    public double calculateBaseFare(double distanceKm, double durationMin) {
        return 50 + (distanceKm * 12) + (durationMin * 2);
    }
}

public class SUVFareStrategy implements FareCalculationStrategy {
    public double calculateBaseFare(double distanceKm, double durationMin) {
        return 80 + (distanceKm * 16) + (durationMin * 2.5);
    }
}
```

### 4.5 Decorator — Surge / Promo layered on fare

```java
public interface FareCalculator {
    double calculate(double distanceKm, double durationMin);
}

public class BaseFareCalculator implements FareCalculator {
    private final FareCalculationStrategy strategy;
    public BaseFareCalculator(FareCalculationStrategy strategy) { this.strategy = strategy; }
    @Override
    public double calculate(double distanceKm, double durationMin) {
        return strategy.calculateBaseFare(distanceKm, durationMin);
    }
}

public abstract class FareDecorator implements FareCalculator {
    protected final FareCalculator wrapped;
    protected FareDecorator(FareCalculator wrapped) { this.wrapped = wrapped; }
}

public class SurgePricingDecorator extends FareDecorator {
    private final double surgeMultiplier;
    public SurgePricingDecorator(FareCalculator wrapped, double multiplier) {
        super(wrapped); this.surgeMultiplier = multiplier;
    }
    @Override
    public double calculate(double distanceKm, double durationMin) {
        return wrapped.calculate(distanceKm, durationMin) * surgeMultiplier;
    }
}

public class PromoDiscountDecorator extends FareDecorator {
    private final double discountPercent; // e.g. 0.1 = 10% off
    public PromoDiscountDecorator(FareCalculator wrapped, double discountPercent) {
        super(wrapped); this.discountPercent = discountPercent;
    }
    @Override
    public double calculate(double distanceKm, double durationMin) {
        double fare = wrapped.calculate(distanceKm, durationMin);
        return fare - (fare * discountPercent);
    }
}
```

Usage: `new PromoDiscountDecorator(new SurgePricingDecorator(new BaseFareCalculator(new SedanFareStrategy()), 1.5), 0.1)` — each concern stacks independently, no subclass explosion.

### 4.6 Strategy — Payment

```java
public interface PaymentStrategy {
    boolean processPayment(double amount);
}

public class CardPayment implements PaymentStrategy {
    private final String cardToken;
    public CardPayment(String cardToken) { this.cardToken = cardToken; }
    public boolean processPayment(double amount) {
        System.out.println("Charging card " + cardToken + " ₹" + amount);
        return true; // call payment gateway in reality
    }
}

public class WalletPayment implements PaymentStrategy {
    private final Wallet wallet;
    public WalletPayment(Wallet wallet) { this.wallet = wallet; }
    public boolean processPayment(double amount) {
        return wallet.deduct(amount);
    }
}

public class CashPayment implements PaymentStrategy {
    public boolean processPayment(double amount) {
        System.out.println("Collect ₹" + amount + " in cash");
        return true;
    }
}
```

### 4.7 Observer — Trip notifications

```java
public interface TripObserver {
    void onStateChange(Trip trip, String newState);
}

public class RiderNotifier implements TripObserver {
    public void onStateChange(Trip trip, String newState) {
        System.out.println("[Rider Push] Trip " + trip.getId() + " -> " + newState);
    }
}

public class DriverNotifier implements TripObserver {
    public void onStateChange(Trip trip, String newState) {
        System.out.println("[Driver Push] Trip " + trip.getId() + " -> " + newState);
    }
}

public class TripAnalyticsLogger implements TripObserver {
    public void onStateChange(Trip trip, String newState) {
        // write to analytics pipeline
    }
}
```

### 4.8 State pattern — Trip lifecycle

```java
public interface TripState {
    void assignDriver(Trip trip, Driver driver);
    void markArrived(Trip trip);
    void start(Trip trip);
    void complete(Trip trip);
    void cancel(Trip trip);
    String name();
}

public class RequestedState implements TripState {
    public void assignDriver(Trip trip, Driver driver) {
        trip.setDriver(driver);
        DriverLocationService.getInstance().markUnavailable(driver);
        trip.setState(new DriverAssignedState());
    }
    public void markArrived(Trip trip) { throw new IllegalStateException("No driver assigned yet"); }
    public void start(Trip trip) { throw new IllegalStateException("Driver not assigned yet"); }
    public void complete(Trip trip) { throw new IllegalStateException("Trip not started"); }
    public void cancel(Trip trip) { trip.setState(new CancelledState()); trip.setCancellationFee(0); }
    public String name() { return "REQUESTED"; }
}

public class DriverAssignedState implements TripState {
    public void assignDriver(Trip trip, Driver driver) { throw new IllegalStateException("Driver already assigned"); }
    public void markArrived(Trip trip) { trip.setState(new ArrivedState()); }
    public void start(Trip trip) { throw new IllegalStateException("Driver hasn't arrived"); }
    public void complete(Trip trip) { throw new IllegalStateException("Trip not started"); }
    public void cancel(Trip trip) {
        DriverLocationService.getInstance().markAvailable(trip.getDriver());
        trip.setState(new CancelledState());
        trip.setCancellationFee(20); // small fee after driver assigned
    }
    public String name() { return "DRIVER_ASSIGNED"; }
}

public class ArrivedState implements TripState {
    public void assignDriver(Trip trip, Driver driver) { throw new IllegalStateException("Already assigned"); }
    public void markArrived(Trip trip) { throw new IllegalStateException("Already arrived"); }
    public void start(Trip trip) { trip.setStartTime(System.currentTimeMillis()); trip.setState(new InProgressState()); }
    public void complete(Trip trip) { throw new IllegalStateException("Trip not started"); }
    public void cancel(Trip trip) {
        DriverLocationService.getInstance().markAvailable(trip.getDriver());
        trip.setState(new CancelledState());
        trip.setCancellationFee(50); // higher fee — driver already waited
    }
    public String name() { return "ARRIVED"; }
}

public class InProgressState implements TripState {
    public void assignDriver(Trip trip, Driver driver) { throw new IllegalStateException("Trip in progress"); }
    public void markArrived(Trip trip) { throw new IllegalStateException("Trip in progress"); }
    public void start(Trip trip) { throw new IllegalStateException("Already started"); }
    public void complete(Trip trip) {
        trip.setEndTime(System.currentTimeMillis());
        double fare = trip.getFareCalculator().calculate(trip.getDistanceKm(), trip.getDurationMin());
        trip.setFinalFare(fare);
        boolean paid = trip.getPaymentStrategy().processPayment(fare);
        if (!paid) throw new IllegalStateException("Payment failed");
        DriverLocationService.getInstance().markAvailable(trip.getDriver());
        trip.setState(new CompletedState());
    }
    public void cancel(Trip trip) { throw new IllegalStateException("Cannot cancel an in-progress trip"); }
    public String name() { return "IN_PROGRESS"; }
}

public class CompletedState implements TripState {
    public void assignDriver(Trip trip, Driver driver) { throw new IllegalStateException("Trip completed"); }
    public void markArrived(Trip trip) { throw new IllegalStateException("Trip completed"); }
    public void start(Trip trip) { throw new IllegalStateException("Trip completed"); }
    public void complete(Trip trip) { throw new IllegalStateException("Already completed"); }
    public void cancel(Trip trip) { throw new IllegalStateException("Trip completed"); }
    public String name() { return "COMPLETED"; }
}

public class CancelledState implements TripState {
    public void assignDriver(Trip trip, Driver driver) { throw new IllegalStateException("Trip cancelled"); }
    public void markArrived(Trip trip) { throw new IllegalStateException("Trip cancelled"); }
    public void start(Trip trip) { throw new IllegalStateException("Trip cancelled"); }
    public void complete(Trip trip) { throw new IllegalStateException("Trip cancelled"); }
    public void cancel(Trip trip) { throw new IllegalStateException("Already cancelled"); }
    public String name() { return "CANCELLED"; }
}
```

### 4.9 Trip — Context + Subject

```java
public class Trip {
    private final String id;
    private final Rider rider;
    private Driver driver;
    private final Location pickup, drop;
    private final RideType rideType;
    private TripState state;
    private final FareCalculator fareCalculator;
    private final PaymentStrategy paymentStrategy;
    private final List<TripObserver> observers = new ArrayList<>();

    private long startTime, endTime;
    private double finalFare, cancellationFee;

    private Trip(Builder b) {
        this.id = b.id; this.rider = b.rider; this.pickup = b.pickup; this.drop = b.drop;
        this.rideType = b.rideType; this.fareCalculator = b.fareCalculator;
        this.paymentStrategy = b.paymentStrategy;
        this.state = new RequestedState();
    }

    public void subscribe(TripObserver o) { observers.add(o); }
    void setState(TripState s) { this.state = s; notifyObservers(); }
    private void notifyObservers() {
        for (TripObserver o : observers) o.onStateChange(this, state.name());
    }

    // delegate all actions to current state
    public void assignDriver(Driver d) { state.assignDriver(this, d); }
    public void markArrived() { state.markArrived(this); }
    public void start() { state.start(this); }
    public void complete() { state.complete(this); }
    public void cancel() { state.cancel(this); }

    public double getDistanceKm() { return pickup.distanceTo(drop); }
    public double getDurationMin() { return (endTime - startTime) / 60000.0; }

    // getters/setters
    public String getId() { return id; }
    public Driver getDriver() { return driver; }
    void setDriver(Driver d) { this.driver = d; }
    public FareCalculator getFareCalculator() { return fareCalculator; }
    public PaymentStrategy getPaymentStrategy() { return paymentStrategy; }
    void setStartTime(long t) { startTime = t; }
    void setEndTime(long t) { endTime = t; }
    void setFinalFare(double f) { finalFare = f; }
    void setCancellationFee(double f) { cancellationFee = f; }

    public static class Builder {
        private String id; private Rider rider; private Location pickup, drop;
        private RideType rideType; private FareCalculator fareCalculator; private PaymentStrategy paymentStrategy;

        public Builder id(String id) { this.id = id; return this; }
        public Builder rider(Rider r) { this.rider = r; return this; }
        public Builder pickup(Location l) { this.pickup = l; return this; }
        public Builder drop(Location l) { this.drop = l; return this; }
        public Builder rideType(RideType t) { this.rideType = t; return this; }
        public Builder fareCalculator(FareCalculator f) { this.fareCalculator = f; return this; }
        public Builder paymentStrategy(PaymentStrategy p) { this.paymentStrategy = p; return this; }
        public Trip build() { return new Trip(this); }
    }
}
```

### 4.10 RideRequestManager — orchestrates request → match → assign

```java
public class RideRequestManager {
    private final DriverMatchingStrategy matchingStrategy;

    public RideRequestManager(DriverMatchingStrategy matchingStrategy) {
        this.matchingStrategy = matchingStrategy;
    }

    public Trip requestRide(Rider rider, Location pickup, Location drop, RideType type,
                             PaymentStrategy paymentStrategy, Double surgeMultiplier, Double promoDiscount) {

        Optional<Driver> matchedDriver = matchingStrategy.findDriver(pickup, type);
        if (matchedDriver.isEmpty()) {
            throw new RuntimeException("No drivers available nearby");
        }

        FareCalculationStrategy fareStrategy = FareStrategyFactory.get(type);
        FareCalculator calculator = new BaseFareCalculator(fareStrategy);
        if (surgeMultiplier != null) calculator = new SurgePricingDecorator(calculator, surgeMultiplier);
        if (promoDiscount != null) calculator = new PromoDiscountDecorator(calculator, promoDiscount);

        Trip trip = new Trip.Builder()
                .id(UUID.randomUUID().toString())
                .rider(rider).pickup(pickup).drop(drop).rideType(type)
                .fareCalculator(calculator).paymentStrategy(paymentStrategy)
                .build();

        trip.subscribe(new RiderNotifier());
        trip.subscribe(new DriverNotifier());
        trip.subscribe(new TripAnalyticsLogger());

        trip.assignDriver(matchedDriver.get());
        return trip;
    }
}
```

### 4.11 Factory Method — fare strategy lookup

```java
public class FareStrategyFactory {
    private static final Map<RideType, FareCalculationStrategy> STRATEGIES = Map.of(
            RideType.MINI, new MiniFareStrategy(),
            RideType.SEDAN, new SedanFareStrategy(),
            RideType.SUV, new SUVFareStrategy()
    );

    public static FareCalculationStrategy get(RideType type) {
        FareCalculationStrategy s = STRATEGIES.get(type);
        if (s == null) throw new IllegalArgumentException("Unsupported ride type: " + type);
        return s;
    }
}
```

### 4.12 Putting it together

```java
public class RideSharingDemo {
    public static void main(String[] args) {
        Driver driver = new Driver(/* ... */);
        DriverLocationService.getInstance().updateLocation(driver, new Location(12.9716, 77.5946));

        RideRequestManager manager = new RideRequestManager(new NearestDriverStrategy());

        Rider rider = new Rider(/* ... */);
        Trip trip = manager.requestRide(
                rider,
                new Location(12.9700, 77.5900),
                new Location(12.9800, 77.6100),
                RideType.SEDAN,
                new CardPayment("tok_123"),
                1.2,   // surge
                0.1    // 10% promo
        );

        trip.markArrived();
        trip.start();
        trip.complete(); // triggers fare calc + payment + observer notifications
    }
}
```

---

## 5. Why this shape holds up under follow-ups

- **"Add Pool rides with multiple riders per trip"** → new `RideType` + `PoolFareStrategy`; `Trip` would hold `List<Rider>` — a variant that fits cleanly since fare/matching are already isolated.
- **"Change matching to consider driver rating + proximity together"** → new `DriverMatchingStrategy` implementation, swap in `RideRequestManager` constructor. Nothing else changes.
- **"Add scheduled/future rides"** → new `ScheduledState` before `RequestedState`, or a separate `ScheduledTrip` wrapper — State pattern already isolates lifecycle rules.
- **"Add ETA-based cancellation fee tiers"** → only touches the `cancel()` implementations in each state class; other logic untouched.
- **"Support UPI/multiple payment methods"** → new `PaymentStrategy` implementation; `Trip` and states never change.
- **Scaling driver lookup to millions of drivers** → swap `DriverLocationService`'s internal linear scan for a geohash/quad-tree/Redis GEO index — the public interface (`findNearby`) stays the same, so `DriverMatchingStrategy` implementations don't change.

---

Want me to extend this with **surge-zone computation (heatmap-based pricing), a distributed geo-index (geohashing/Redis GEO) implementation, driver-side trip acceptance timeout/re-matching, or a Pool-ride matching algorithm**, or move to a different LLD problem (Parking Lot, Elevator, BookMyShow, Splitwise)?