# Food Delivery App (Swiggy/Zomato) — LLD Design

## 1. Requirements

**Functional:**
- Customers browse restaurants/menus, add items to cart, place orders
- Restaurants manage menu (add/update/remove items), accept/reject orders
- Delivery partners get assigned to orders, update delivery status
- Order goes through a lifecycle: PLACED → CONFIRMED → PREPARING → OUT_FOR_DELIVERY → DELIVERED (or CANCELLED)
- Multiple payment methods (UPI, Card, Wallet, COD)
- Search/filter restaurants by cuisine, rating, location
- Notify customer/restaurant/delivery partner on order status changes
- Support discounts/coupons

**Non-functional:**
- Extensible for new payment methods, notification channels, discount types
- Order status changes should notify multiple interested parties without tight coupling
- Delivery partner assignment strategy should be swappable (nearest, load-balanced)

---

## 2. Design Patterns Used (and why)

| Pattern | Where | Why |
|---|---|---|
| **State** | `OrderState` (Placed, Confirmed, Preparing, OutForDelivery, Delivered, Cancelled) | Order behavior/allowed transitions differ per state — avoids scattered if/else validation logic |
| **Observer** | `Order` notifies `OrderObserver`s (Customer, Restaurant, DeliveryPartner) on state change | Decouples order lifecycle from all the parties that need to react — new observers can be added without touching `Order` |
| **Strategy** | `DeliveryAssignmentStrategy` (NearestPartner, LoadBalanced); `DiscountStrategy` (FlatOff, PercentOff, ComboOffer) | Algorithms vary independently and need to be swappable at runtime |
| **Factory** | `PaymentFactory` creates `PaymentMethod` (UPI/Card/Wallet/COD) | Centralizes object creation logic, hides concrete payment class instantiation |
| **Builder** | `OrderBuilder` | Order has many optional fields (coupon, instructions, tip) — builder avoids telescoping constructors |
| **Decorator** | `MenuItem` customization (extra cheese, add-ons) via `MenuItemDecorator` | Add-ons should stack dynamically on a base item without exploding subclasses |
| **Singleton** | `OrderManager` | Single coordination point for all active orders in the system |
| **Chain of Responsibility** | `DiscountValidatorChain` (coupon validity → min order value → user eligibility) | Multiple independent validation steps that can be composed/reordered |

---

## 3. SOLID Mapping

- **SRP** — `Order` manages lifecycle only; `PaymentMethod` handles payment only; `DeliveryAssignmentStrategy` only decides partner assignment.
- **OCP** — New payment types, discount types, or delivery strategies plug in via new implementations, no existing code modified.
- **LSP** — Any `OrderState`, `PaymentMethod`, or `DiscountStrategy` is substitutable without breaking callers.
- **ISP** — `OrderObserver` only has `onStatusChange`; restaurants/customers/delivery partners aren't forced into unrelated interfaces.
- **DIP** — `OrderManager` depends on `DeliveryAssignmentStrategy` and `PaymentMethod` interfaces, not concrete implementations.

---

## 4. Class Diagram (textual)

```
Enums: OrderStatus, PaymentType, VehicleType

User (abstract) — id, name, phone
 ├── Customer — address, cart
 ├── RestaurantOwner
 └── DeliveryPartner — currentLocation, isAvailable, vehicleType

Restaurant
 - id, name, location, menu: List<MenuItem>, rating
 + addMenuItem(), updateAvailability()

MenuItem (Component - Decorator base)
 - id, name, price, isVeg
 ├── MenuItemDecorator (abstract) — wraps MenuItem
 │    ├── ExtraCheeseDecorator
 │    └── ExtraToppingDecorator

Cart
 - customer, items: Map<MenuItem, Integer>
 + addItem(), removeItem(), getTotal()

OrderState (interface)
 ├── PlacedState, ConfirmedState, PreparingState,
 │    OutForDeliveryState, DeliveredState, CancelledState

Order
 - id, customer, restaurant, items, deliveryPartner
 - state: OrderState, payment: PaymentMethod
 - observers: List<OrderObserver>
 + confirm(), cancel(), nextState(), notifyObservers()

OrderBuilder → builds Order

OrderObserver (interface)
 + onStatusChange(Order)
 ├── CustomerNotifier
 ├── RestaurantNotifier
 └── DeliveryPartnerNotifier

PaymentMethod (interface)
 ├── UpiPayment, CardPayment, WalletPayment, CODPayment
PaymentFactory → creates PaymentMethod

DiscountStrategy (interface)
 ├── FlatDiscount, PercentDiscount, ComboOffer

DeliveryAssignmentStrategy (interface)
 ├── NearestPartnerStrategy, LoadBalancedStrategy

OrderManager (Singleton)
 - orders: Map<orderId, Order>
 - deliveryStrategy, restaurants
 + placeOrder(), assignDeliveryPartner(), trackOrder()
```

---

## 5. Code (Java)

### Enums

```java
public enum OrderStatus { PLACED, CONFIRMED, PREPARING, OUT_FOR_DELIVERY, DELIVERED, CANCELLED }
public enum PaymentType { UPI, CARD, WALLET, COD }
```

### MenuItem + Decorator pattern

```java
public interface MenuItem {
    String getName();
    double getPrice();
}

public class BaseMenuItem implements MenuItem {
    private final String name;
    private final double price;

    public BaseMenuItem(String name, double price) {
        this.name = name;
        this.price = price;
    }
    @Override public String getName() { return name; }
    @Override public double getPrice() { return price; }
}

public abstract class MenuItemDecorator implements MenuItem {
    protected final MenuItem wrapped;
    protected MenuItemDecorator(MenuItem wrapped) { this.wrapped = wrapped; }
}

public class ExtraCheeseDecorator extends MenuItemDecorator {
    public ExtraCheeseDecorator(MenuItem wrapped) { super(wrapped); }
    @Override public String getName() { return wrapped.getName() + " + Extra Cheese"; }
    @Override public double getPrice() { return wrapped.getPrice() + 30; }
}
```

### Cart

```java
import java.util.*;

public class Cart {
    private final Map<MenuItem, Integer> items = new LinkedHashMap<>();

    public void addItem(MenuItem item, int qty) {
        items.merge(item, qty, Integer::sum);
    }
    public void removeItem(MenuItem item) { items.remove(item); }
    public double getTotal() {
        return items.entrySet().stream()
            .mapToDouble(e -> e.getKey().getPrice() * e.getValue())
            .sum();
    }
    public Map<MenuItem, Integer> getItems() { return items; }
    public void clear() { items.clear(); }
}
```

### OrderState (State pattern)

```java
public interface OrderState {
    void next(Order order);
    void cancel(Order order);
    OrderStatus getStatus();
}

public class PlacedState implements OrderState {
    @Override public void next(Order order) { order.setState(new ConfirmedState()); }
    @Override public void cancel(Order order) { order.setState(new CancelledState()); }
    @Override public OrderStatus getStatus() { return OrderStatus.PLACED; }
}

public class ConfirmedState implements OrderState {
    @Override public void next(Order order) { order.setState(new PreparingState()); }
    @Override public void cancel(Order order) { order.setState(new CancelledState()); }
    @Override public OrderStatus getStatus() { return OrderStatus.CONFIRMED; }
}

public class PreparingState implements OrderState {
    @Override public void next(Order order) { order.setState(new OutForDeliveryState()); }
    @Override public void cancel(Order order) {
        throw new IllegalStateException("Cannot cancel order once preparing started");
    }
    @Override public OrderStatus getStatus() { return OrderStatus.PREPARING; }
}

public class OutForDeliveryState implements OrderState {
    @Override public void next(Order order) { order.setState(new DeliveredState()); }
    @Override public void cancel(Order order) {
        throw new IllegalStateException("Cannot cancel order in transit");
    }
    @Override public OrderStatus getStatus() { return OrderStatus.OUT_FOR_DELIVERY; }
}

public class DeliveredState implements OrderState {
    @Override public void next(Order order) { /* terminal */ }
    @Override public void cancel(Order order) {
        throw new IllegalStateException("Cannot cancel delivered order");
    }
    @Override public OrderStatus getStatus() { return OrderStatus.DELIVERED; }
}

public class CancelledState implements OrderState {
    @Override public void next(Order order) { /* terminal */ }
    @Override public void cancel(Order order) { /* already cancelled */ }
    @Override public OrderStatus getStatus() { return OrderStatus.CANCELLED; }
}
```

### OrderObserver (Observer pattern)

```java
public interface OrderObserver {
    void onStatusChange(Order order);
}

public class CustomerNotifier implements OrderObserver {
    @Override public void onStatusChange(Order order) {
        System.out.println("Notify customer " + order.getCustomer().getName() +
            ": order " + order.getId() + " is now " + order.getStatus());
    }
}

public class RestaurantNotifier implements OrderObserver {
    @Override public void onStatusChange(Order order) {
        System.out.println("Notify restaurant " + order.getRestaurant().getName() +
            ": order " + order.getId() + " is now " + order.getStatus());
    }
}

public class DeliveryPartnerNotifier implements OrderObserver {
    @Override public void onStatusChange(Order order) {
        if (order.getDeliveryPartner() != null) {
            System.out.println("Notify delivery partner: order " + order.getId() +
                " is now " + order.getStatus());
        }
    }
}
```

### Order (Subject + State context)

```java
import java.util.*;

public class Order {
    private final String id;
    private final Customer customer;
    private final Restaurant restaurant;
    private final Map<MenuItem, Integer> items;
    private final double totalAmount;
    private DeliveryPartner deliveryPartner;
    private PaymentMethod payment;
    private OrderState state = new PlacedState();
    private final List<OrderObserver> observers = new ArrayList<>();

    public Order(String id, Customer customer, Restaurant restaurant,
                 Map<MenuItem, Integer> items, double totalAmount, PaymentMethod payment) {
        this.id = id;
        this.customer = customer;
        this.restaurant = restaurant;
        this.items = items;
        this.totalAmount = totalAmount;
        this.payment = payment;
    }

    public void addObserver(OrderObserver o) { observers.add(o); }
    private void notifyObservers() {
        for (OrderObserver o : observers) o.onStatusChange(this);
    }

    public void proceedToNextState() {
        state.next(this);
        notifyObservers();
    }

    public void cancelOrder() {
        state.cancel(this);
        notifyObservers();
    }

    public void setState(OrderState state) { this.state = state; }
    public void assignDeliveryPartner(DeliveryPartner dp) { this.deliveryPartner = dp; }

    public OrderStatus getStatus() { return state.getStatus(); }
    public String getId() { return id; }
    public Customer getCustomer() { return customer; }
    public Restaurant getRestaurant() { return restaurant; }
    public DeliveryPartner getDeliveryPartner() { return deliveryPartner; }
    public double getTotalAmount() { return totalAmount; }
}
```

### OrderBuilder (Builder pattern)

```java
public class OrderBuilder {
    private String id;
    private Customer customer;
    private Restaurant restaurant;
    private Map<MenuItem, Integer> items;
    private double totalAmount;
    private PaymentMethod payment;

    public OrderBuilder setId(String id) { this.id = id; return this; }
    public OrderBuilder setCustomer(Customer c) { this.customer = c; return this; }
    public OrderBuilder setRestaurant(Restaurant r) { this.restaurant = r; return this; }
    public OrderBuilder setItems(Map<MenuItem, Integer> items) { this.items = items; return this; }
    public OrderBuilder setTotalAmount(double amount) { this.totalAmount = amount; return this; }
    public OrderBuilder setPayment(PaymentMethod payment) { this.payment = payment; return this; }

    public Order build() {
        if (customer == null || restaurant == null || items == null || items.isEmpty()) {
            throw new IllegalStateException("Missing required order fields");
        }
        return new Order(id, customer, restaurant, items, totalAmount, payment);
    }
}
```

### PaymentMethod (Factory pattern)

```java
public interface PaymentMethod {
    boolean pay(double amount);
}

public class UpiPayment implements PaymentMethod {
    @Override public boolean pay(double amount) {
        System.out.println("Paid " + amount + " via UPI");
        return true;
    }
}

public class CardPayment implements PaymentMethod {
    @Override public boolean pay(double amount) {
        System.out.println("Paid " + amount + " via Card");
        return true;
    }
}

public class WalletPayment implements PaymentMethod {
    @Override public boolean pay(double amount) {
        System.out.println("Paid " + amount + " via Wallet");
        return true;
    }
}

public class CODPayment implements PaymentMethod {
    @Override public boolean pay(double amount) {
        System.out.println("Cash on delivery for " + amount);
        return true;
    }
}

public class PaymentFactory {
    public static PaymentMethod create(PaymentType type) {
        return switch (type) {
            case UPI -> new UpiPayment();
            case CARD -> new CardPayment();
            case WALLET -> new WalletPayment();
            case COD -> new CODPayment();
        };
    }
}
```

### DiscountStrategy (Strategy pattern)

```java
public interface DiscountStrategy {
    double applyDiscount(double amount);
}

public class FlatDiscount implements DiscountStrategy {
    private final double flatOff;
    public FlatDiscount(double flatOff) { this.flatOff = flatOff; }
    @Override public double applyDiscount(double amount) {
        return Math.max(0, amount - flatOff);
    }
}

public class PercentDiscount implements DiscountStrategy {
    private final double percent;
    public PercentDiscount(double percent) { this.percent = percent; }
    @Override public double applyDiscount(double amount) {
        return amount - (amount * percent / 100);
    }
}
```

### DeliveryAssignmentStrategy (Strategy pattern)

```java
import java.util.*;

public interface DeliveryAssignmentStrategy {
    DeliveryPartner assign(List<DeliveryPartner> partners, Restaurant restaurant);
}

public class NearestPartnerStrategy implements DeliveryAssignmentStrategy {
    @Override
    public DeliveryPartner assign(List<DeliveryPartner> partners, Restaurant restaurant) {
        return partners.stream()
            .filter(DeliveryPartner::isAvailable)
            .min(Comparator.comparingDouble(p -> distance(p, restaurant)))
            .orElse(null);
    }
    private double distance(DeliveryPartner p, Restaurant r) {
        return Math.abs(p.getLocation() - r.getLocation()); // simplified
    }
}
```

### User hierarchy, Restaurant, DeliveryPartner

```java
public abstract class User {
    protected String id;
    protected String name;
    protected String phone;
    public String getName() { return name; }
}

public class Customer extends User {
    private String address;
    private Cart cart = new Cart();
    public Cart getCart() { return cart; }
}

public class DeliveryPartner extends User {
    private double location; // simplified as 1D for distance calc
    private boolean available = true;
    public double getLocation() { return location; }
    public boolean isAvailable() { return available; }
    public void setAvailable(boolean available) { this.available = available; }
}

public class Restaurant {
    private String id;
    private String name;
    private double location;
    private final List<MenuItem> menu = new ArrayList<>();

    public void addMenuItem(MenuItem item) { menu.add(item); }
    public String getName() { return name; }
    public double getLocation() { return location; }
}
```

### OrderManager (Singleton)

```java
import java.util.*;

public class OrderManager {
    private static OrderManager instance;
    private final Map<String, Order> orders = new HashMap<>();
    private final List<DeliveryPartner> deliveryPartners = new ArrayList<>();
    private DeliveryAssignmentStrategy deliveryStrategy = new NearestPartnerStrategy();

    private OrderManager() {}

    public static synchronized OrderManager getInstance() {
        if (instance == null) instance = new OrderManager();
        return instance;
    }

    public void registerDeliveryPartner(DeliveryPartner dp) { deliveryPartners.add(dp); }
    public void setDeliveryStrategy(DeliveryAssignmentStrategy strategy) { this.deliveryStrategy = strategy; }

    public Order placeOrder(Customer customer, Restaurant restaurant, PaymentType paymentType) {
        Cart cart = customer.getCart();
        double total = cart.getTotal();
        PaymentMethod payment = PaymentFactory.create(paymentType);

        Order order = new OrderBuilder()
            .setId(UUID.randomUUID().toString())
            .setCustomer(customer)
            .setRestaurant(restaurant)
            .setItems(cart.getItems())
            .setTotalAmount(total)
            .setPayment(payment)
            .build();

        order.addObserver(new CustomerNotifier());
        order.addObserver(new RestaurantNotifier());
        order.addObserver(new DeliveryPartnerNotifier());

        if (payment.pay(total)) {
            orders.put(order.getId(), order);
            cart.clear();
        }
        return order;
    }

    public void confirmOrder(String orderId) {
        Order order = orders.get(orderId);
        order.proceedToNextState(); // -> CONFIRMED
        DeliveryPartner dp = deliveryStrategy.assign(deliveryPartners, order.getRestaurant());
        if (dp != null) {
            dp.setAvailable(false);
            order.assignDeliveryPartner(dp);
        }
    }

    public void advanceOrder(String orderId) {
        orders.get(orderId).proceedToNextState();
    }

    public void cancelOrder(String orderId) {
        orders.get(orderId).cancelOrder();
    }

    public Order trackOrder(String orderId) {
        return orders.get(orderId);
    }
}
```

### Usage

```java
public class Main {
    public static void main(String[] args) {
        Restaurant restaurant = new Restaurant();
        MenuItem pizza = new BaseMenuItem("Margherita Pizza", 250);
        MenuItem pizzaWithCheese = new ExtraCheeseDecorator(pizza);
        restaurant.addMenuItem(pizzaWithCheese);

        Customer customer = new Customer();
        customer.getCart().addItem(pizzaWithCheese, 2);

        OrderManager manager = OrderManager.getInstance();
        manager.registerDeliveryPartner(new DeliveryPartner());

        Order order = manager.placeOrder(customer, restaurant, PaymentType.UPI);
        manager.confirmOrder(order.getId());   // PLACED -> CONFIRMED + assign partner
        manager.advanceOrder(order.getId());   // -> PREPARING
        manager.advanceOrder(order.getId());   // -> OUT_FOR_DELIVERY
        manager.advanceOrder(order.getId());   // -> DELIVERED
    }
}
```

---

## 6. Extensibility Notes

- **New payment method** (e.g., BNPL) → implement `PaymentMethod`, register in `PaymentFactory`. No changes elsewhere.
- **New discount type** (e.g., first-order discount) → implement `DiscountStrategy`; can even chain multiple via a `CompositeDiscountStrategy`.
- **New delivery assignment logic** (batching multiple orders, EV-only routing) → implement `DeliveryAssignmentStrategy`.
- **Real-time tracking** → `DeliveryPartnerNotifier` observer can be extended to push live GPS coordinates via WebSocket without touching `Order`.
- **Rating/review system** → add as a separate `ReviewService` observing `DeliveredState` transition, keeping `Order` free of review logic (SRP).
- **Surge pricing** → decorate `PaymentMethod.pay()` amount calculation with a `SurgePricingDecorator`, similar to menu item decorators.

Want me to extend this with **coupon/promo code validation chain (Chain of Responsibility)** in detail, **restaurant search/filter (Strategy + Specification pattern)**, or a **concurrency-safe cart/inventory model** for handling item stock during high-traffic flash sales?