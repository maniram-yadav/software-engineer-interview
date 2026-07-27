# Online Stock Brokerage System — LLD

## 1. Requirements

**Functional**
- User places orders: Market, Limit, Stop-Loss, Stop-Limit.
- Orders validated (funds check, quantity limits, market hours, risk limits) before hitting the exchange/order book.
- Order lifecycle: Placed → Validated → Open/PartiallyFilled → Filled / Cancelled / Rejected / Expired.
- Order matching against live market price (simplified — in reality this is exchange-side, but we simulate matching engine hooks).
- On execution: update user's portfolio (holdings), update wallet balance, log the trade.
- Real-time price feed per stock symbol; multiple orders/watchers depend on the same feed.
- Portfolio view: holdings, average buy price, current market value, P&L.
- Notify user on order status changes (filled, partially filled, rejected).
- Support order modification and cancellation (only while in cancellable states).

**Non-functional**
- New order types (e.g., Trailing Stop, Iceberg) addable without touching core execution flow.
- Validation rules must be composable/extensible without a giant if/else chain in `OrderService`.
- Price feed is a hot path read by many components — single source of truth, not duplicated polling.
- Order state transitions strictly controlled — no filling a cancelled order, no cancelling a filled one.

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **State** | `OrderState` interface: `PlacedState`, `OpenState`, `PartiallyFilledState`, `FilledState`, `CancelledState`, `RejectedState`, `ExpiredState` | Legal actions (`fill`, `cancel`, `modify`) depend entirely on current order state. Prevents illegal transitions like cancelling a filled order. |
| **Strategy** | `OrderExecutionStrategy`: `MarketOrderStrategy`, `LimitOrderStrategy`, `StopLossOrderStrategy`, `StopLimitOrderStrategy` | Each order type decides *when/how* it becomes eligible to execute against market price completely differently. Isolating this avoids a monolithic matching function with type-checks. |
| **Chain of Responsibility** | `OrderValidator` chain: `FundsCheckValidator` → `QuantityValidator` → `MarketHoursValidator` → `RiskLimitValidator` | Validation is a sequence of independent, addable/removable checks. New compliance rule = new handler in the chain, no existing validator touched. |
| **Observer** | `Order`/`Trade` (Subject) notifies `OrderObserver`: `PortfolioUpdater`, `WalletUpdater`, `NotificationService`, `TradeLogger` | One execution event → several independent reactions (update holdings, deduct/credit cash, notify user, audit log) without the matching engine knowing about any of them. |
| **Singleton** | `MarketDataService` | Single, central live-price feed per symbol; every order/watcher reads from one source of truth rather than each maintaining its own polling connection. |
| **Factory Method** | `OrderFactory.createOrder(type, ...)` | Encapsulates which `OrderExecutionStrategy` + initial validation chain pairs with a given order type. |
| **Builder** | `Order.Builder` | Orders have many optional fields (stop price, limit price, expiry, time-in-force) — avoids telescoping constructors. |
| **Command** | `OrderCommand` (`PlaceOrderCommand`, `CancelOrderCommand`, `ModifyOrderCommand`) with an order history/audit log | Every user action on an order needs to be auditable and potentially undoable (e.g., a modify that needs reverting) — Command captures each action as an object. |

**SOLID**
- **S**: `Order` holds order data + delegates behavior to state; `OrderMatchingEngine` only matches; `PortfolioUpdater` only updates holdings; `MarketDataService` only serves prices.
- **O**: New order type → new `OrderExecutionStrategy` + factory entry. New validation rule → new `OrderValidator` link. New post-trade reaction → new `OrderObserver`. Nothing existing changes.
- **L**: Any `OrderState` substitutable wherever `Order` delegates; any `OrderExecutionStrategy` substitutable in the matching engine.
- **I**: `OrderObserver` exposes only `onOrderEvent`; `OrderValidator` exposes only `validate`/`setNext` — no fat interfaces.
- **D**: `Order` and `OrderMatchingEngine` depend on `OrderExecutionStrategy`/`OrderState` abstractions injected at creation, never concrete classes.

---

## 3. Class Diagram (textual)

```
┌───────────────────┐        ┌────────────────────────┐
│   OrderState          │◀──────│  Order (Context/Subject)  │
│ (State interface)      │       │ - state: OrderState         │
│ + fill(qty, price)      │      │ - executionStrategy          │
│ + cancel()               │     │ - observers: List<Obs>        │
│ + modify()                │    └────────────────────────┘
└────────▲──────────────┘
  ┌──────┼─────┬───────────┬───────────┬──────────┬─────────┐
Placed  Open  Partially   Filled    Cancelled  Rejected   Expired
State  State  FilledState  State      State      State      State

┌────────────────────────────┐      ┌──────────────────────┐
│ OrderExecutionStrategy         │     │  OrderValidator          │
│ (Strategy interface)            │    │ (Chain of Responsibility) │
│ + isEligibleToExecute(price)      │  │ + validate(order)          │
└─────────────▲───────────────┘      │ + setNext(validator)         │
   ┌──────────┼──────────┬─────────┐ └───────────▲──────────────┘
Market  Limit  StopLoss  StopLimit   ┌────────────┼─────────────┬─────────────┐
Strategy Strategy Strategy Strategy FundsCheck  QuantityCheck MarketHours  RiskLimit

┌───────────────────┐         ┌───────────────────────┐
│  OrderObserver         │      │  MarketDataService        │
│ + onOrderEvent(evt)     │     │  (Singleton)                │
└──────────▲───────────┘        │ + getPrice(symbol)           │
    ┌──────┼───────┬─────────┐  │ + subscribe(symbol, listener)│
PortfolioUpdater WalletUpdater NotificationService TradeLogger └───────────────────────┘

┌───────────────────┐        ┌──────────────────────┐
│  OrderFactory          │      │  OrderMatchingEngine     │
│ + createOrder(...)      │     │ + onPriceUpdate(symbol,px)│
└───────────────────┘        │ + submit(order)             │
                              └──────────────────────┘

┌────────────────┐  ┌────────────────┐  ┌────────────────┐
│  Portfolio        │  │  Wallet           │  │  User             │
│ - holdings: Map     │ │ - balance          │  │ - portfolio         │
└────────────────┘  └────────────────┘  │ - wallet             │
                                          └────────────────┘

┌───────────────────┐
│  OrderCommand          │  (Command: Place/Cancel/Modify, audit trail)
└───────────────────┘
```

---

## 4. Code (Java)

### 4.1 Core entities

```java
public enum OrderSide { BUY, SELL }
public enum OrderType { MARKET, LIMIT, STOP_LOSS, STOP_LIMIT }
public enum TimeInForce { DAY, GTC, IOC } // Good-Till-Cancel, Immediate-Or-Cancel

public class Stock {
    private final String symbol;
    private final String companyName;
    // getters omitted
}

public class Wallet {
    private double balance;
    public Wallet(double balance) { this.balance = balance; }

    public synchronized boolean debit(double amount) {
        if (balance < amount) return false;
        balance -= amount;
        return true;
    }
    public synchronized void credit(double amount) { balance += amount; }
    public double getBalance() { return balance; }
}

public class Holding {
    String symbol;
    int quantity;
    double avgBuyPrice;
}

public class Portfolio {
    private final Map<String, Holding> holdings = new ConcurrentHashMap<>();

    public void addHolding(String symbol, int qty, double price) {
        Holding h = holdings.computeIfAbsent(symbol, s -> {
            Holding nh = new Holding(); nh.symbol = s; return nh;
        });
        double totalCost = (h.avgBuyPrice * h.quantity) + (price * qty);
        h.quantity += qty;
        h.avgBuyPrice = h.quantity == 0 ? 0 : totalCost / h.quantity;
    }

    public void removeHolding(String symbol, int qty) {
        Holding h = holdings.get(symbol);
        if (h == null || h.quantity < qty) throw new IllegalStateException("Insufficient holdings");
        h.quantity -= qty;
        if (h.quantity == 0) holdings.remove(symbol);
    }

    public Map<String, Holding> getHoldings() { return holdings; }
}

public class User {
    private final String id;
    private final String name;
    private final Wallet wallet;
    private final Portfolio portfolio;
    // getters omitted
}
```

### 4.2 Singleton — MarketDataService (live price feed)

```java
public interface PriceListener {
    void onPriceUpdate(String symbol, double price);
}

public class MarketDataService {
    private static volatile MarketDataService instance;
    private final ConcurrentHashMap<String, Double> latestPrices = new ConcurrentHashMap<>();
    private final ConcurrentHashMap<String, List<PriceListener>> listeners = new ConcurrentHashMap<>();

    private MarketDataService() {}

    public static MarketDataService getInstance() {
        if (instance == null) {
            synchronized (MarketDataService.class) {
                if (instance == null) instance = new MarketDataService();
            }
        }
        return instance;
    }

    public double getPrice(String symbol) {
        return latestPrices.getOrDefault(symbol, 0.0);
    }

    public void subscribe(String symbol, PriceListener listener) {
        listeners.computeIfAbsent(symbol, s -> new CopyOnWriteArrayList<>()).add(listener);
    }

    // called by exchange feed adapter
    public void publishPrice(String symbol, double price) {
        latestPrices.put(symbol, price);
        for (PriceListener l : listeners.getOrDefault(symbol, Collections.emptyList())) {
            l.onPriceUpdate(symbol, price);
        }
    }
}
```

### 4.3 Chain of Responsibility — Order Validation

```java
public abstract class OrderValidator {
    protected OrderValidator next;
    public OrderValidator setNext(OrderValidator next) { this.next = next; return next; }

    public final void validate(Order order) {
        doValidate(order);
        if (next != null) next.validate(order);
    }
    protected abstract void doValidate(Order order);
}

public class FundsCheckValidator extends OrderValidator {
    @Override
    protected void doValidate(Order order) {
        if (order.getSide() == OrderSide.BUY) {
            double estimatedCost = order.getQuantity() * MarketDataService.getInstance().getPrice(order.getSymbol());
            if (order.getUser().getWallet().getBalance() < estimatedCost) {
                throw new OrderRejectedException("Insufficient funds");
            }
        }
    }
}

public class QuantityValidator extends OrderValidator {
    @Override
    protected void doValidate(Order order) {
        if (order.getQuantity() <= 0) throw new OrderRejectedException("Invalid quantity");
        if (order.getSide() == OrderSide.SELL) {
            Holding h = order.getUser().getPortfolio().getHoldings().get(order.getSymbol());
            if (h == null || h.quantity < order.getQuantity()) {
                throw new OrderRejectedException("Insufficient holdings to sell");
            }
        }
    }
}

public class MarketHoursValidator extends OrderValidator {
    @Override
    protected void doValidate(Order order) {
        LocalTime now = LocalTime.now();
        if (now.isBefore(LocalTime.of(9, 15)) || now.isAfter(LocalTime.of(15, 30))) {
            throw new OrderRejectedException("Market closed");
        }
    }
}

public class RiskLimitValidator extends OrderValidator {
    private static final double MAX_ORDER_VALUE = 1_000_000;
    @Override
    protected void doValidate(Order order) {
        double value = order.getQuantity() * MarketDataService.getInstance().getPrice(order.getSymbol());
        if (value > MAX_ORDER_VALUE) throw new OrderRejectedException("Exceeds risk limit");
    }
}

public class OrderValidationChainBuilder {
    public static OrderValidator build() {
        OrderValidator funds = new FundsCheckValidator();
        funds.setNext(new QuantityValidator())
             .setNext(new MarketHoursValidator())
             .setNext(new RiskLimitValidator());
        return funds;
    }
}
```

### 4.4 Strategy — Order Execution Eligibility

```java
public interface OrderExecutionStrategy {
    boolean isEligibleToExecute(Order order, double currentMarketPrice);
    double getExecutionPrice(Order order, double currentMarketPrice);
}

public class MarketOrderStrategy implements OrderExecutionStrategy {
    public boolean isEligibleToExecute(Order order, double currentMarketPrice) { return true; }
    public double getExecutionPrice(Order order, double currentMarketPrice) { return currentMarketPrice; }
}

public class LimitOrderStrategy implements OrderExecutionStrategy {
    public boolean isEligibleToExecute(Order order, double currentMarketPrice) {
        if (order.getSide() == OrderSide.BUY) return currentMarketPrice <= order.getLimitPrice();
        return currentMarketPrice >= order.getLimitPrice();
    }
    public double getExecutionPrice(Order order, double currentMarketPrice) {
        return order.getLimitPrice(); // fills at limit or better
    }
}

public class StopLossOrderStrategy implements OrderExecutionStrategy {
    public boolean isEligibleToExecute(Order order, double currentMarketPrice) {
        // SELL stop-loss triggers when price falls to/below stop price
        if (order.getSide() == OrderSide.SELL) return currentMarketPrice <= order.getStopPrice();
        return currentMarketPrice >= order.getStopPrice(); // BUY stop (breakout)
    }
    public double getExecutionPrice(Order order, double currentMarketPrice) {
        return currentMarketPrice; // converts to market order once triggered
    }
}

public class StopLimitOrderStrategy implements OrderExecutionStrategy {
    public boolean isEligibleToExecute(Order order, double currentMarketPrice) {
        boolean triggered = order.getSide() == OrderSide.SELL
                ? currentMarketPrice <= order.getStopPrice()
                : currentMarketPrice >= order.getStopPrice();
        if (!triggered) return false;
        return order.getSide() == OrderSide.BUY
                ? currentMarketPrice <= order.getLimitPrice()
                : currentMarketPrice >= order.getLimitPrice();
    }
    public double getExecutionPrice(Order order, double currentMarketPrice) { return order.getLimitPrice(); }
}
```

### 4.5 Observer — post-trade reactions

```java
public interface OrderObserver {
    void onOrderEvent(Order order, String eventType); // "FILLED", "PARTIALLY_FILLED", "REJECTED", "CANCELLED"
}

public class PortfolioUpdater implements OrderObserver {
    public void onOrderEvent(Order order, String eventType) {
        if (!eventType.equals("FILLED") && !eventType.equals("PARTIALLY_FILLED")) return;
        Portfolio portfolio = order.getUser().getPortfolio();
        if (order.getSide() == OrderSide.BUY) {
            portfolio.addHolding(order.getSymbol(), order.getFilledQuantity(), order.getExecutionPrice());
        } else {
            portfolio.removeHolding(order.getSymbol(), order.getFilledQuantity());
        }
    }
}

public class WalletUpdater implements OrderObserver {
    public void onOrderEvent(Order order, String eventType) {
        if (!eventType.equals("FILLED") && !eventType.equals("PARTIALLY_FILLED")) return;
        Wallet wallet = order.getUser().getWallet();
        double amount = order.getFilledQuantity() * order.getExecutionPrice();
        if (order.getSide() == OrderSide.BUY) wallet.debit(amount);
        else wallet.credit(amount);
    }
}

public class NotificationService implements OrderObserver {
    public void onOrderEvent(Order order, String eventType) {
        System.out.println("[Notify] Order " + order.getId() + " -> " + eventType);
    }
}

public class TradeLogger implements OrderObserver {
    public void onOrderEvent(Order order, String eventType) {
        // append to audit/trade log store
    }
}
```

### 4.6 State pattern — Order lifecycle

```java
public interface OrderState {
    void fill(Order order, int quantity, double price);
    void cancel(Order order);
    void reject(Order order, String reason);
    String name();
}

public class PlacedState implements OrderState {
    public void fill(Order order, int qty, double price) { order.setState(new OpenState()); order.getState().fill(order, qty, price); }
    public void cancel(Order order) { order.setState(new CancelledState()); }
    public void reject(Order order, String reason) { order.setState(new RejectedState()); order.notifyObservers("REJECTED"); }
    public String name() { return "PLACED"; }
}

public class OpenState implements OrderState {
    public void fill(Order order, int qty, double price) {
        order.applyFill(qty, price);
        if (order.getFilledQuantity() == order.getQuantity()) {
            order.setState(new FilledState());
            order.notifyObservers("FILLED");
        } else {
            order.setState(new PartiallyFilledState());
            order.notifyObservers("PARTIALLY_FILLED");
        }
    }
    public void cancel(Order order) { order.setState(new CancelledState()); order.notifyObservers("CANCELLED"); }
    public void reject(Order order, String reason) { order.setState(new RejectedState()); order.notifyObservers("REJECTED"); }
    public String name() { return "OPEN"; }
}

public class PartiallyFilledState implements OrderState {
    public void fill(Order order, int qty, double price) {
        order.applyFill(qty, price);
        if (order.getFilledQuantity() == order.getQuantity()) {
            order.setState(new FilledState());
            order.notifyObservers("FILLED");
        } else {
            order.notifyObservers("PARTIALLY_FILLED");
        }
    }
    public void cancel(Order order) { order.setState(new CancelledState()); order.notifyObservers("CANCELLED"); } // cancels remaining qty
    public void reject(Order order, String reason) { throw new IllegalStateException("Cannot reject a partially filled order"); }
    public String name() { return "PARTIALLY_FILLED"; }
}

public class FilledState implements OrderState {
    public void fill(Order order, int qty, double price) { throw new IllegalStateException("Already filled"); }
    public void cancel(Order order) { throw new IllegalStateException("Cannot cancel a filled order"); }
    public void reject(Order order, String reason) { throw new IllegalStateException("Already filled"); }
    public String name() { return "FILLED"; }
}

public class CancelledState implements OrderState {
    public void fill(Order order, int qty, double price) { throw new IllegalStateException("Order cancelled"); }
    public void cancel(Order order) { throw new IllegalStateException("Already cancelled"); }
    public void reject(Order order, String reason) { throw new IllegalStateException("Order cancelled"); }
    public String name() { return "CANCELLED"; }
}

public class RejectedState implements OrderState {
    public void fill(Order order, int qty, double price) { throw new IllegalStateException("Order rejected"); }
    public void cancel(Order order) { throw new IllegalStateException("Order rejected"); }
    public void reject(Order order, String reason) { throw new IllegalStateException("Already rejected"); }
    public String name() { return "REJECTED"; }
}

public class ExpiredState implements OrderState {
    public void fill(Order order, int qty, double price) { throw new IllegalStateException("Order expired"); }
    public void cancel(Order order) { throw new IllegalStateException("Order expired"); }
    public void reject(Order order, String reason) { throw new IllegalStateException("Order expired"); }
    public String name() { return "EXPIRED"; }
}
```

### 4.7 Order — Context + Subject

```java
public class Order {
    private final String id;
    private final User user;
    private final String symbol;
    private final OrderSide side;
    private final OrderType type;
    private final int quantity;
    private final Double limitPrice;   // nullable
    private final Double stopPrice;    // nullable
    private final TimeInForce timeInForce;
    private final OrderExecutionStrategy executionStrategy;

    private OrderState state = new PlacedState();
    private int filledQuantity = 0;
    private double executionPrice;
    private final List<OrderObserver> observers = new ArrayList<>();

    private Order(Builder b) {
        this.id = b.id; this.user = b.user; this.symbol = b.symbol; this.side = b.side;
        this.type = b.type; this.quantity = b.quantity; this.limitPrice = b.limitPrice;
        this.stopPrice = b.stopPrice; this.timeInForce = b.timeInForce;
        this.executionStrategy = b.executionStrategy;
    }

    public void subscribe(OrderObserver o) { observers.add(o); }
    void notifyObservers(String eventType) {
        for (OrderObserver o : observers) o.onOrderEvent(this, eventType);
    }

    void setState(OrderState s) { this.state = s; }
    OrderState getState() { return state; }

    void applyFill(int qty, double price) {
        this.filledQuantity += qty;
        this.executionPrice = price; // simplistic: last fill price; could be weighted avg
    }

    // delegate to state
    public void fill(int qty, double price) { state.fill(this, qty, price); }
    public void cancel() { state.cancel(this); }
    public void reject(String reason) { state.reject(this, reason); }

    public boolean isEligibleToExecute(double marketPrice) { return executionStrategy.isEligibleToExecute(this, marketPrice); }
    public double resolveExecutionPrice(double marketPrice) { return executionStrategy.getExecutionPrice(this, marketPrice); }

    // getters
    public String getId() { return id; }
    public User getUser() { return user; }
    public String getSymbol() { return symbol; }
    public OrderSide getSide() { return side; }
    public int getQuantity() { return quantity; }
    public int getFilledQuantity() { return filledQuantity; }
    public double getExecutionPrice() { return executionPrice; }
    public Double getLimitPrice() { return limitPrice; }
    public Double getStopPrice() { return stopPrice; }
    public String getStateName() { return state.name(); }

    public static class Builder {
        private String id; private User user; private String symbol; private OrderSide side;
        private OrderType type; private int quantity; private Double limitPrice, stopPrice;
        private TimeInForce timeInForce = TimeInForce.DAY;
        private OrderExecutionStrategy executionStrategy;

        public Builder id(String id) { this.id = id; return this; }
        public Builder user(User u) { this.user = u; return this; }
        public Builder symbol(String s) { this.symbol = s; return this; }
        public Builder side(OrderSide s) { this.side = s; return this; }
        public Builder type(OrderType t) { this.type = t; return this; }
        public Builder quantity(int q) { this.quantity = q; return this; }
        public Builder limitPrice(Double p) { this.limitPrice = p; return this; }
        public Builder stopPrice(Double p) { this.stopPrice = p; return this; }
        public Builder timeInForce(TimeInForce t) { this.timeInForce = t; return this; }
        public Builder executionStrategy(OrderExecutionStrategy s) { this.executionStrategy = s; return this; }
        public Order build() { return new Order(this); }
    }
}
```

### 4.8 Factory Method — OrderFactory

```java
public class OrderFactory {
    public static Order createOrder(String id, User user, String symbol, OrderSide side,
                                     OrderType type, int qty, Double limitPrice, Double stopPrice) {

        OrderExecutionStrategy strategy;
        switch (type) {
            case MARKET: strategy = new MarketOrderStrategy(); break;
            case LIMIT: strategy = new LimitOrderStrategy(); break;
            case STOP_LOSS: strategy = new StopLossOrderStrategy(); break;
            case STOP_LIMIT: strategy = new StopLimitOrderStrategy(); break;
            default: throw new IllegalArgumentException("Unsupported order type: " + type);
        }

        Order order = new Order.Builder()
                .id(id).user(user).symbol(symbol).side(side).type(type)
                .quantity(qty).limitPrice(limitPrice).stopPrice(stopPrice)
                .executionStrategy(strategy)
                .build();

        order.subscribe(new PortfolioUpdater());
        order.subscribe(new WalletUpdater());
        order.subscribe(new NotificationService());
        order.subscribe(new TradeLogger());
        return order;
    }
}
```

### 4.9 Order Matching Engine — ties validation + price feed + execution together

```java
public class OrderMatchingEngine implements PriceListener {
    private final OrderValidator validationChain = OrderValidationChainBuilder.build();
    private final Map<String, List<Order>> pendingOrdersBySymbol = new ConcurrentHashMap<>();

    public void submit(Order order) {
        try {
            validationChain.validate(order);
        } catch (OrderRejectedException e) {
            order.reject(e.getMessage());
            return;
        }
        pendingOrdersBySymbol.computeIfAbsent(order.getSymbol(), s -> new CopyOnWriteArrayList<>()).add(order);
        MarketDataService.getInstance().subscribe(order.getSymbol(), this);

        // check immediately in case current price already satisfies (e.g. market order)
        double currentPrice = MarketDataService.getInstance().getPrice(order.getSymbol());
        tryExecute(order, currentPrice);
    }

    @Override
    public void onPriceUpdate(String symbol, double price) {
        List<Order> orders = pendingOrdersBySymbol.getOrDefault(symbol, Collections.emptyList());
        for (Order order : new ArrayList<>(orders)) {
            tryExecute(order, price);
        }
    }

    private void tryExecute(Order order, double marketPrice) {
        if (order.getStateName().equals("FILLED") || order.getStateName().equals("CANCELLED")) return;
        if (marketPrice <= 0) return;

        if (order.isEligibleToExecute(marketPrice)) {
            double execPrice = order.resolveExecutionPrice(marketPrice);
            int remainingQty = order.getQuantity() - order.getFilledQuantity();
            order.fill(remainingQty, execPrice); // simplified: full fill; partial-fill logic would check available liquidity
            if (order.getStateName().equals("FILLED")) {
                pendingOrdersBySymbol.get(order.getSymbol()).remove(order);
            }
        }
    }
}

class OrderRejectedException extends RuntimeException {
    public OrderRejectedException(String msg) { super(msg); }
}
```

### 4.10 Command pattern — auditable user actions

```java
public interface OrderCommand {
    void execute();
}

public class PlaceOrderCommand implements OrderCommand {
    private final OrderMatchingEngine engine;
    private final Order order;
    public PlaceOrderCommand(OrderMatchingEngine engine, Order order) { this.engine = engine; this.order = order; }
    public void execute() { engine.submit(order); }
}

public class CancelOrderCommand implements OrderCommand {
    private final Order order;
    public CancelOrderCommand(Order order) { this.order = order; }
    public void execute() { order.cancel(); }
}

public class OrderCommandInvoker {
    private final Deque<OrderCommand> history = new ArrayDeque<>();
    public void run(OrderCommand cmd) { cmd.execute(); history.push(cmd); }
}
```

### 4.11 Putting it together

```java
public class BrokerageDemo {
    public static void main(String[] args) {
        User user = new User(/* id, name, new Wallet(100000), new Portfolio() */);

        MarketDataService.getInstance().publishPrice("INFY", 1500.0);

        OrderMatchingEngine engine = new OrderMatchingEngine();
        OrderCommandInvoker invoker = new OrderCommandInvoker();

        Order buyOrder = OrderFactory.createOrder(
                UUID.randomUUID().toString(), user, "INFY", OrderSide.BUY,
                OrderType.LIMIT, 10, 1495.0, null);

        invoker.run(new PlaceOrderCommand(engine, buyOrder));

        // price drops to trigger the limit order
        MarketDataService.getInstance().publishPrice("INFY", 1494.0);
        // -> observers fire: PortfolioUpdater adds holding, WalletUpdater debits, NotificationService prints
    }
}
```

---

## 5. Why this shape holds up under follow-ups

- **"Add Trailing Stop order"** → new `TrailingStopOrderStrategy` implementing `OrderExecutionStrategy` + factory entry. Matching engine untouched.
- **"Add SEBI/compliance check before order submission"** → new `OrderValidator` link appended to the chain. Nothing else changes.
- **"Add SMS notification in addition to push"** → new `OrderObserver` implementation subscribed alongside existing ones.
- **"Support Good-Till-Cancel orders auto-expiring after N days"** → add an `ExpiredState`-triggering scheduled job that calls a new `expire()` transition — State pattern already has the slot (`ExpiredState` stub included above).
- **"Partial fills based on available market liquidity"** → `tryExecute` already separates "eligible to execute" from "how much fills" — extend `applyFill` logic without touching state classes.
- **"Multiple exchanges (NSE/BSE) with different price feeds"** → `MarketDataService` singleton already isolates the feed source; can be extended to route by exchange without touching `Order`/`OrderMatchingEngine`.

---

Want me to extend this with **order book depth (bid/ask matching engine internals), margin trading / leverage rules, real-time portfolio P&L calculation, or a distributed event-driven architecture (Kafka for trade events)**, or move to a different LLD problem?