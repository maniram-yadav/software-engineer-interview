# Design a Multiplayer Game Server Architecture — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Synchronize real-time game state (player positions, actions, world events) across all players in a match
- Handle player actions with responsive, low-latency feel despite real network delay between client and server
- Support matchmaking to group players into balanced sessions
- Maintain authoritative game state that clients cannot manipulate to cheat

### Non-Functional Requirements
- **Extremely low perceived latency:** Players need actions to feel instant, despite real network round-trip time (RTT) working against this
- **Cheat resistance:** The server, not any client, must be the ultimate authority on what actually happened in the game
- **Consistent state across all players:** Every player must see a coherent, fair version of the game world, despite each having different network conditions
- **Scalability:** Must support many concurrent game sessions/matches simultaneously, each potentially with many players

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Concurrent matches | Thousands to tens of thousands |
| Players per match | 2-100, depending on game genre |
| State updates/sec (per match) | 20-60 (tick rate) |
| Acceptable RTT for "responsive" feel | Under ~100-150ms, ideally much lower |

---

## 2. The Core Principle — Authoritative Server, Not Trusting Any Client

```mermaid
flowchart TB
    A["Naive approach: each<br/>client reports its OWN<br/>position/actions, and the<br/>server simply relays this<br/>to other players"] --> A1["Problem: a malicious client<br/>could report ANYTHING —<br/>'I'm at this impossible<br/>location,' 'I dealt 1000<br/>damage instead of 10' —<br/>this is trivially exploitable<br/>for cheating"]

    B["Authoritative server model:<br/>clients send their INTENDED<br/>actions/inputs (e.g., 'I<br/>pressed forward,' 'I fired<br/>my weapon'), but the SERVER<br/>independently SIMULATES the<br/>actual game physics/logic<br/>and determines the REAL,<br/>authoritative outcome"] --> B1["A malicious client can still<br/>send FALSE inputs, but the<br/>server's own simulation<br/>enforces physical/game-logic<br/>constraints (can't move<br/>faster than the game allows,<br/>can't deal more damage than<br/>a weapon's defined stats) —<br/>this is what makes<br/>server-authoritative<br/>architecture fundamentally<br/>cheat-resistant in a way<br/>client-trusting architecture<br/>never can be"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Clients["Player Clients"]
        Client1["Player 1"]
        Client2["Player 2"]
        ClientN["Player N..."]
    end

    subgraph Matchmaking["Matchmaking Service"]
        MatchQueue["Matchmaking Queue<br/>(skill-based grouping)"]
        SessionAllocator["Game Session Allocator"]
    end

    subgraph GameServers["Game Server Fleet"]
        GameServer1["Game Server Instance<br/>(hosts one active match)"]
        GameServer2["Game Server Instance<br/>(hosts another match)"]
    end

    subgraph Persistence["Match & Player State"]
        MatchHistory[("Match History/Stats Store")]
        PlayerProfiles[("Player Profile Store")]
    end

    Client1 --> MatchQueue
    Client2 --> MatchQueue
    MatchQueue --> SessionAllocator
    SessionAllocator --> GameServer1

    Client1 <-->|"UDP, low-latency<br/>state sync"| GameServer1
    Client2 <-->|"UDP"| GameServer1
    ClientN <-->|"UDP"| GameServer2

    GameServer1 --> MatchHistory
    GameServer1 --> PlayerProfiles
```

**Key idea:** Each active match is hosted by ONE dedicated game server instance, which acts as the single authoritative simulation for that match's entire game world — all connected clients communicate exclusively with this server instance (not directly with each other), and the server's simulation is the single source of truth for what actually happened.

---

## 4. Data Model

```mermaid
erDiagram
    MATCH_SESSION {
        string match_id PK
        string game_server_instance_id
        string status "active/completed"
        timestamp started_at
    }
    PLAYER_STATE {
        string match_id FK
        string player_id
        vector3 position
        vector3 velocity
        int health
        long last_processed_input_sequence
    }
    PLAYER_INPUT {
        string match_id FK
        string player_id
        long sequence_number
        map input_data "e.g. movement direction, action buttons"
        timestamp client_timestamp
    }
```

---

## 5. The Core Latency-Hiding Techniques

```mermaid
flowchart TB
    A["The fundamental physics<br/>problem: network round-trip<br/>time (RTT) between client<br/>and server is REAL and<br/>UNAVOIDABLE — often 30-100ms<br/>or more — but players need<br/>actions to FEEL instant"] --> B{"Three complementary<br/>techniques address this"}

    B --> C["Client-Side Prediction"]
    B --> D["Server Reconciliation"]
    B --> E["Lag Compensation<br/>(for hit detection)"]

    C --> C1["Client immediately simulates<br/>its OWN action locally,<br/>WITHOUT waiting for server<br/>confirmation — feels instant<br/>to that player"]
    D --> D1["When the server's<br/>authoritative response<br/>arrives (after RTT), the<br/>client RECONCILES any<br/>discrepancy between its<br/>local prediction and the<br/>server's actual result"]
    E --> E1["When determining if a<br/>player's shot HIT another<br/>player, the server accounts<br/>for the SHOOTER'S network<br/>delay — effectively<br/>'rewinding time' to see<br/>what the target's position<br/>ACTUALLY was from the<br/>shooter's perspective"]
```

---

## 6. Client-Side Prediction & Server Reconciliation — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Player Client
    participant LocalSim as Local Prediction<br/>(client-side)
    participant Server as Game Server<br/>(authoritative)

    Client->>Client: Player presses "move forward"
    Client->>LocalSim: IMMEDIATELY simulate this<br/>locally — move the player's<br/>visual position forward<br/>RIGHT NOW, don't wait for<br/>server

    Client->>Server: Send input:<br/>{sequence: 501, action:<br/>move_forward, client_time: T}

    Note over Client: Player continues seeing<br/>responsive, instant movement<br/>locally while the input<br/>travels to the server<br/>(network RTT)

    Server->>Server: Process input 501 through<br/>AUTHORITATIVE simulation<br/>(validates against game<br/>rules — e.g., collision<br/>detection, speed limits)

    Server-->>Client: Authoritative state update:<br/>{sequence: 501, actual_position:<br/>(X,Y,Z), confirmed}

    Client->>Client: RECONCILE: compare server's<br/>authoritative position<br/>against what LOCAL PREDICTION<br/>had assumed

    alt Prediction was correct<br/>(common case)
        Client->>Client: No visible correction needed —<br/>seamless
    else Prediction diverged<br/>(e.g., server rejected due<br/>to collision client didn't<br/>predict)
        Client->>Client: Smoothly CORRECT the visual<br/>position to match server's<br/>authoritative truth<br/>(often blended/smoothed to<br/>avoid a jarring visual snap)
    end
```

**Why this combination gives BOTH responsiveness AND correctness:** The player experiences instant, responsive movement (client-side prediction), while the server's independent, authoritative simulation remains the actual source of truth (preventing cheating) — reconciliation is what bridges these two, silently correcting the rare cases where prediction and authoritative reality diverge, ideally so smoothly the player never notices.

---

## 7. Lag Compensation for Hit Detection — Detailed Sequence

```mermaid
sequenceDiagram
    participant Shooter as Player A (shooter)
    participant Server as Game Server
    participant StateHistory as Historical State Buffer<br/>(recent snapshots)
    participant Target as Player B (target)

    Note over Target: Player B is actually AT<br/>position (10, 5) at time T

    Note over Shooter: Due to network latency,<br/>Player A's CLIENT is<br/>currently displaying Player<br/>B at an OLDER position<br/>(8, 5) — from ~100ms ago

    Shooter->>Server: "I fired at position (8,5)<br/>at my local time T"<br/>(this is what Player A<br/>SAW and aimed at)

    Server->>StateHistory: Look up: where was Player B<br/>ACTUALLY positioned at the<br/>time Player A's client<br/>would have rendered them<br/>(accounting for A's<br/>network latency)?

    StateHistory-->>Server: Player B was at (8,5)<br/>at that EARLIER moment<br/>(the history buffer stores<br/>recent snapshots specifically<br/>for this purpose)

    Server->>Server: Determine hit using<br/>Player B's position AS THE<br/>SHOOTER ACTUALLY SAW IT,<br/>not B's CURRENT real-time<br/>position

    Server-->>Shooter: HIT confirmed
    Server-->>Target: You were hit<br/>(even though your CURRENT<br/>position has since moved<br/>away from where you were<br/>shot)
```

**Why this "rewinding time" approach is considered fair, not a compromise:** Without lag compensation, a player with network latency would experience the deeply frustrating "I clearly hit them, but the game says I missed" problem, because by the time their shot reaches the server, the target has already moved past where the shooter genuinely saw them — lag compensation restores fairness FROM THE SHOOTER'S PERSPECTIVE, which is the perspective that actually matters for a satisfying, fair gameplay experience.

---

## 8. Game State Synchronization (Server to All Clients)

```mermaid
sequenceDiagram
    participant Server as Game Server<br/>(authoritative simulation)
    participant ClientA as Player A
    participant ClientB as Player B

    loop Every game tick (e.g., 60 times/sec)
        Server->>Server: Advance authoritative<br/>simulation by one tick<br/>(process all received inputs,<br/>update physics/game logic)

        Server->>ClientA: Broadcast state update<br/>(positions, health, events —<br/>often DELTA-COMPRESSED,<br/>only what CHANGED since<br/>last update)
        Server->>ClientB: Broadcast state update
    end

    Note over ClientA,ClientB: Each client renders the<br/>world based on received<br/>server state, BLENDED with<br/>their own local prediction<br/>for their own player<br/>character specifically
```

**Why delta compression matters for state updates:** Sending the FULL game state (every player's position, every object) on every single tick, dozens of times per second, to every connected client, would consume substantial bandwidth; sending only what's CHANGED since the last update dramatically reduces data volume, similar in principle to the delta sync approach in the Mobile Offline Caching and Distributed File Storage designs, applied here at a much higher update frequency.

---

## 9. Matchmaking Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Player as Player
    participant Queue as Matchmaking Queue
    participant Allocator as Session Allocator
    participant GameServer as New Game Server Instance

    Player->>Queue: Join matchmaking<br/>{skill_rating, region}

    Queue->>Queue: Group compatible players<br/>(similar skill rating,<br/>similar region for low<br/>latency) into a balanced<br/>match

    Note over Queue: Once enough compatible<br/>players are queued<br/>(or a timeout triggers<br/>a slightly less optimal<br/>match rather than<br/>indefinite waiting)

    Queue->>Allocator: Request new match session<br/>for these N players

    Allocator->>GameServer: Provision/allocate a<br/>game server instance<br/>(from a warm pool, similar<br/>principle to the ML Model<br/>Serving design's warm-up<br/>consideration, to avoid<br/>cold-start delay)

    GameServer-->>Allocator: Ready
    Allocator-->>Player: Connect to<br/>{game_server_address}

    Player->>GameServer: Establish connection,<br/>match begins
```

**Why regional grouping matters alongside skill matching:** Even a perfectly skill-balanced match will feel unfair or unresponsive if players are matched across continents, since fundamental network RTT scales with geographic distance — matchmaking must balance BOTH competitive fairness (skill) and technical fairness (latency), sometimes accepting a slightly less skill-optimal match in exchange for meaningfully better latency for all participants.

---

## 10. Handling Player Disconnection Mid-Match

```mermaid
flowchart TB
    A["Player's connection drops<br/>mid-match (network issue,<br/>app crash)"] --> B["Server detects missed<br/>heartbeats/input packets<br/>(same failure-detection<br/>principle as the Network<br/>Partition Detection design)"]

    B --> C{"Reconnection Policy"}
    C --> D["Grace period: server<br/>continues simulating the<br/>disconnected player's<br/>character with their LAST<br/>known input (or a neutral<br/>'idle' state), giving them<br/>a window to reconnect"]
    C --> E["If reconnection succeeds<br/>within the grace period:<br/>client re-syncs by receiving<br/>a FULL current state snapshot<br/>(not just deltas, since it<br/>missed an unknown amount<br/>of history) and resumes"]
    C --> F["If grace period expires:<br/>player is removed from the<br/>match (possibly replaced by<br/>an AI-controlled bot in<br/>some game types, or the<br/>match simply continues<br/>without them)"]
```

---

## 11. Component Responsibilities Summary

```mermaid
mindmap
  root((Multiplayer Game Server HLD))
    Matchmaking Queue
      Skill and region based grouping
      Balances fairness and latency
    Game Server Instance
      Authoritative simulation
      One instance per active match
    Client-Side Prediction
      Instant local responsiveness
      Reconciled against server truth
    Lag Compensation
      Historical state buffer
      Fair hit detection from shooter's view
    State Synchronization
      Delta-compressed broadcasts
      Fixed tick rate
    Reconnection Handling
      Grace period simulation
      Full snapshot resync
```

---

## 12. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Authority model | Server-authoritative, client-predictive | The only architecture that's both cheat-resistant (server enforces real rules) and responsive-feeling (client doesn't wait for round-trip confirmation) |
| Hit detection | Lag-compensated, shooter's-perspective rewind | Restores fairness from the perspective that determines whether the shot genuinely felt like a hit to the player who fired it |
| State updates | Fixed tick rate, delta-compressed | Bounds bandwidth and server processing cost while keeping all clients reasonably synchronized |
| Session hosting | One dedicated server instance per match | Keeps each match's authoritative simulation isolated and simple, scaling horizontally across many concurrent matches rather than one shared mega-simulation |
| Matchmaking | Joint skill + regional balancing | Optimizing purely for skill balance while ignoring latency would undermine the responsiveness the rest of the architecture works hard to achieve |
| Disconnection handling | Grace period + full snapshot resync | Tolerates brief, common connectivity blips without immediately ejecting players, while ensuring a genuinely lost sync state is fully repaired on reconnect, not just patched with deltas |

---

## 13. Bottlenecks & Scaling Considerations

- **Server tick processing time vs tick rate** — the server must complete all simulation work for a tick WITHIN that tick's time budget (e.g., under ~16ms for 60Hz) or updates start lagging; this bounds how much game logic complexity or how many players a single match can support before requiring architecture changes (e.g., splitting simulation across multiple threads/processes for very large player counts, a much harder problem than the embarrassingly-parallel per-security isolation in the Trading Matching Engine design).
- **Network protocol choice (UDP vs TCP)** — most real-time games use UDP specifically because TCP's guaranteed-delivery, in-order semantics can cause a single lost packet to STALL delivery of all subsequent packets (head-of-line blocking) — for real-time state updates, a slightly stale but promptly-delivered update is often preferable to waiting for guaranteed, in-order delivery; this requires the application layer to handle its own reliability/ordering logic where genuinely needed (e.g., critical events) rather than relying on the transport layer.
- **Historical state buffer size for lag compensation** — storing recent snapshots for rewind-based hit detection requires bounded memory per match; the buffer only needs to cover the maximum reasonable player latency (e.g., a few hundred milliseconds of history), not unlimited history.
- **Server fleet capacity planning for matchmaking spikes** — popular games experience highly variable concurrent player counts (peak evening hours, weekend spikes); the game server fleet needs auto-scaling capability with fast provisioning (warm pools) to avoid matchmaking queue delays during demand surges.
- **Cheat detection beyond basic authority** — while server-authoritative architecture prevents the most blatant cheats (teleporting, infinite health), more subtle cheats (aim-assist bots reacting inhumanly fast to legitimate server data) require ADDITIONAL detection layers analyzing player behavior patterns, connecting to similar statistical anomaly-detection principles as the Bot Detection and Fraud Detection designs.
- **Cross-region latency for global matchmaking** — for games with a smaller player population in certain regions, strict regional matchmaking can create excessively long queue times; systems often need a graduated fallback (expand acceptable latency range gradually the longer a player waits) rather than a single rigid regional boundary.
- **Testing under realistic network conditions** — client-side prediction, reconciliation, and lag compensation are all specifically designed to handle network imperfection — but this makes them notoriously hard to test correctly under IDEAL (low-latency, no packet loss) development/testing conditions; realistic testing requires deliberately simulating latency, jitter, and packet loss to verify these systems behave correctly under the actual conditions they're designed for.
