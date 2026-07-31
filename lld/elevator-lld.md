# Elevator System — LLD Design

## 1. Requirements

**Functional:**
- Multiple elevators (cars) serving multiple floors in a building
- Users can request an elevator from a floor (external request: UP/DOWN)
- Users can select destination floor inside elevator (internal request)
- System dispatches the "best" elevator to a request
- Elevator moves, opens/closes doors, tracks direction and state
- Support for different dispatch strategies (nearest car, least busy, zone-based)

**Non-functional:**
- Extensible to add new dispatch strategies without changing core logic
- Thread-safe request handling (multiple simultaneous requests)
- Easy to add new elevator behaviors (e.g., maintenance mode, express elevator)

**Assumptions:** Single building, N elevators, M floors, requests come from floor panels (Hall calls) and elevator panels (Car calls).

---

## 2. Design Patterns Used (and why)

| Pattern | Where | Why |
|---|---|---|
| **State** | `ElevatorState` (Idle, Moving, DoorOpen, Maintenance) | Elevator behavior changes based on internal state — avoids giant if/else chains, each state encapsulates its own transition logic |
| **Strategy** | `DispatchStrategy` (NearestCar, LeastBusy) | Dispatch algorithm needs to be swappable at runtime without modifying `ElevatorController` |
| **Observer** | `Elevator` notifies `ElevatorController`/Display panels on state change | Decouples elevator internals from external systems (displays, logging, monitoring) that need to react to state changes |
| **Singleton** | `ElevatorController` | Single point of coordination for all elevators in the building — only one controller should exist |
| **Factory** | `RequestFactory` (creates HallRequest/CarRequest) | Centralizes creation logic of different request types |
| **Command** | `Request` objects encapsulate a floor+direction as a command dispatched to elevators | Decouples request creation from execution; makes queuing/undo/logging easy |

---

## 3. SOLID Mapping

- **SRP** — `Elevator` only manages its own movement/door state; `ElevatorController` only handles dispatch coordination; `DispatchStrategy` only decides which elevator to pick.
- **OCP** — New dispatch algorithms or new elevator states can be added without touching existing classes (implement new `DispatchStrategy` or `ElevatorState`).
- **LSP** — Any `ElevatorState` implementation can substitute another; any `DispatchStrategy` can substitute another — `ElevatorController` doesn't care which concrete one it gets.
- **ISP** — Separate interfaces for `Direction`-aware components vs `Dispatchable` vs `Observer` — no class is forced to implement methods it doesn't need.
- **DIP** — `ElevatorController` depends on `DispatchStrategy` interface, not a concrete class; `Elevator` depends on `ElevatorState` interface, not concrete states.

---

## 4. Class Diagram (textual)

```
Enums: Direction {UP, DOWN, IDLE}, DoorState {OPEN, CLOSED}

Request (abstract)
 ├── HallRequest (floor, direction)
 └── CarRequest (floor)      // from inside elevator

ElevatorState (interface)
 ├── IdleState
 ├── MovingState
 ├── DoorOpenState
 └── MaintenanceState

Elevator
 - id, currentFloor, direction, doorState
 - state: ElevatorState
 - requestQueue: TreeSet<Integer> (or two queues: up/down)
 - observers: List<ElevatorObserver>
 + move(), openDoor(), closeDoor(), addRequest(), setState()

DispatchStrategy (interface)
 + selectElevator(List<Elevator>, HallRequest): Elevator
 ├── NearestCarStrategy
 └── LeastBusyStrategy

ElevatorController (Singleton)
 - elevators: List<Elevator>
 - dispatchStrategy: DispatchStrategy
 + requestElevator(HallRequest)
 + submitCarRequest(elevatorId, CarRequest)

ElevatorObserver (interface)
 + onStateChange(Elevator)
 ├── DisplayPanel
 └── Logger
```

---

## 5. Code (Java)

### Enums

```java
public enum Direction { UP, DOWN, IDLE }
public enum DoorState { OPEN, CLOSED }
```

### Request (Command pattern)

```java
public abstract class Request {
    private final int floor;
    protected Request(int floor) { this.floor = floor; }
    public int getFloor() { return floor; }
}

public class HallRequest extends Request {
    private final Direction direction;
    public HallRequest(int floor, Direction direction) {
        super(floor);
        this.direction = direction;
    }
    public Direction getDirection() { return direction; }
}

public class CarRequest extends Request {
    public CarRequest(int floor) { super(floor); }
}
```

### ElevatorState (State pattern)

```java
public interface ElevatorState {
    void handle(Elevator elevator);
}

public class IdleState implements ElevatorState {
    @Override
    public void handle(Elevator elevator) {
        if (!elevator.hasPendingRequests()) return;
        elevator.setState(new MovingState());
    }
}

public class MovingState implements ElevatorState {
    @Override
    public void handle(Elevator elevator) {
        int target = elevator.getNextStop();
        if (target == elevator.getCurrentFloor()) {
            elevator.setState(new DoorOpenState());
            return;
        }
        elevator.setDirection(target > elevator.getCurrentFloor() ? Direction.UP : Direction.DOWN);
        elevator.moveOneFloor();
    }
}

public class DoorOpenState implements ElevatorState {
    @Override
    public void handle(Elevator elevator) {
        elevator.setDoorState(DoorState.OPEN);
        elevator.removeCurrentFloorRequest();
        // after wait/timer in real system
        elevator.setDoorState(DoorState.CLOSED);
        elevator.setState(elevator.hasPendingRequests() ? new MovingState() : new IdleState());
    }
}

public class MaintenanceState implements ElevatorState {
    @Override
    public void handle(Elevator elevator) {
        // reject new requests, stay put
    }
}
```

### Elevator (Subject in Observer pattern)

```java
import java.util.*;

public class Elevator {
    private final int id;
    private int currentFloor = 0;
    private Direction direction = Direction.IDLE;
    private DoorState doorState = DoorState.CLOSED;
    private ElevatorState state = new IdleState();

    // TreeSet keeps requests sorted for efficient nearest-stop lookup
    private final TreeSet<Integer> upStops = new TreeSet<>();
    private final TreeSet<Integer> downStops = new TreeSet<>(Collections.reverseOrder());

    private final List<ElevatorObserver> observers = new ArrayList<>();

    public Elevator(int id) { this.id = id; }

    public void addRequest(Request request) {
        int floor = request.getFloor();
        if (floor >= currentFloor) upStops.add(floor);
        else downStops.add(floor);
        state.handle(this); // may trigger transition from Idle -> Moving
    }

    public void step() {
        state.handle(this);
        notifyObservers();
    }

    public boolean hasPendingRequests() {
        return !upStops.isEmpty() || !downStops.isEmpty();
    }

    public int getNextStop() {
        if (direction == Direction.UP || direction == Direction.IDLE) {
            if (!upStops.isEmpty()) return upStops.first();
            if (!downStops.isEmpty()) return downStops.first();
        } else {
            if (!downStops.isEmpty()) return downStops.first();
            if (!upStops.isEmpty()) return upStops.first();
        }
        return currentFloor;
    }

    public void moveOneFloor() {
        currentFloor += (direction == Direction.UP) ? 1 : -1;
    }

    public void removeCurrentFloorRequest() {
        upStops.remove(currentFloor);
        downStops.remove(currentFloor);
    }

    public void setState(ElevatorState state) { this.state = state; }
    public void setDirection(Direction direction) { this.direction = direction; }
    public void setDoorState(DoorState doorState) { this.doorState = doorState; }
    public int getCurrentFloor() { return currentFloor; }
    public int getId() { return id; }
    public Direction getDirection() { return direction; }

    // Observer registration
    public void addObserver(ElevatorObserver o) { observers.add(o); }
    private void notifyObservers() {
        for (ElevatorObserver o : observers) o.onStateChange(this);
    }
}
```

### ElevatorObserver (Observer pattern)

```java
public interface ElevatorObserver {
    void onStateChange(Elevator elevator);
}

public class DisplayPanel implements ElevatorObserver {
    @Override
    public void onStateChange(Elevator elevator) {
        System.out.println("Elevator " + elevator.getId() +
            " at floor " + elevator.getCurrentFloor() +
            " direction " + elevator.getDirection());
    }
}
```

### DispatchStrategy (Strategy pattern)

```java
public interface DispatchStrategy {
    Elevator selectElevator(List<Elevator> elevators, HallRequest request);
}

public class NearestCarStrategy implements DispatchStrategy {
    @Override
    public Elevator selectElevator(List<Elevator> elevators, HallRequest request) {
        Elevator best = null;
        int minDistance = Integer.MAX_VALUE;
        for (Elevator e : elevators) {
            int distance = Math.abs(e.getCurrentFloor() - request.getFloor());
            if (distance < minDistance) {
                minDistance = distance;
                best = e;
            }
        }
        return best;
    }
}

public class LeastBusyStrategy implements DispatchStrategy {
    @Override
    public Elevator selectElevator(List<Elevator> elevators, HallRequest request) {
        return elevators.stream()
            .min(Comparator.comparingInt(e -> e.hasPendingRequests() ? 1 : 0))
            .orElse(elevators.get(0));
    }
}
```

### ElevatorController (Singleton + coordinator)

```java
public class ElevatorController {
    private static ElevatorController instance;
    private final List<Elevator> elevators = new ArrayList<>();
    private DispatchStrategy dispatchStrategy;

    private ElevatorController() {}

    public static synchronized ElevatorController getInstance() {
        if (instance == null) instance = new ElevatorController();
        return instance;
    }

    public void addElevator(Elevator elevator) { elevators.add(elevator); }
    public void setDispatchStrategy(DispatchStrategy strategy) { this.dispatchStrategy = strategy; }

    public void requestElevator(HallRequest request) {
        Elevator chosen = dispatchStrategy.selectElevator(elevators, request);
        chosen.addRequest(request);
    }

    public void submitCarRequest(int elevatorId, CarRequest request) {
        elevators.stream()
            .filter(e -> e.getId() == elevatorId)
            .findFirst()
            .ifPresent(e -> e.addRequest(request));
    }

    public void step() {
        for (Elevator e : elevators) e.step();
    }
}
```

### Usage

```java
public class Main {
    public static void main(String[] args) {
        ElevatorController controller = ElevatorController.getInstance();
        controller.setDispatchStrategy(new NearestCarStrategy());

        Elevator e1 = new Elevator(1);
        Elevator e2 = new Elevator(2);
        e1.addObserver(new DisplayPanel());
        e2.addObserver(new DisplayPanel());

        controller.addElevator(e1);
        controller.addElevator(e2);

        controller.requestElevator(new HallRequest(5, Direction.UP));
        controller.submitCarRequest(1, new CarRequest(8));

        for (int i = 0; i < 10; i++) controller.step();
    }
}
```

---

## 6. Extensibility Notes

- **New dispatch algorithm** (e.g., zone-based, ML-predicted demand) → just implement `DispatchStrategy`, no change to `ElevatorController`.
- **New elevator behavior** (e.g., express elevator skipping floors, fire emergency mode) → new `ElevatorState` implementation.
- **Multi-building support** → make `ElevatorController` per-building (drop Singleton, use a `BuildingManager` holding one controller per building) — shows why Singleton here is a deliberate simplification, not a permanent constraint.
- **Concurrency** → in production, wrap `addRequest`/`step` with locks per elevator, or use an actor-model/message-queue approach so each elevator processes its own request queue independently.
- **Load/weight constraints** → add a `CapacityStrategy` decorator or check in `addRequest` for a "elevator full" condition.

Want me to go deeper on any part — e.g., concurrency handling with `ReentrantLock`, a scan algorithm (like disk elevator/SCAN algorithm) for optimal stop ordering, or extend this to a REST API layer?