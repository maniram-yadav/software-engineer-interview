# Cricbuzz-like Live Cricket Scoring System — LLD

## 1. Requirements

**Functional**
- Create matches (T20 / ODI / Test) with two teams, venue, playing XI.
- Record ball-by-ball events: runs, extras (wide/no-ball/bye/leg-bye), wickets, strike rotation.
- Auto-generate live scorecard: score, overs, run rate, batsman/bowler stats, partnerships.
- Track match state transitions: Scheduled → Toss → Live → Innings Break → Completed / Abandoned.
- Support format-specific rules (over limits, follow-on in Test, powerplay in ODI/T20).
- Push live updates to subscribed users (real-time score, commentary) — many clients watching one match.
- Ball-by-ball commentary feed.
- Ability to correct/undo the last ball entry (mis-recorded delivery, umpire review reversal).

**Non-functional**
- Many users read one match's live state → efficient fan-out, not per-user polling of raw data.
- New formats (The Hundred, 100-ball) addable without touching scoring engine.
- Scoring engine must not know about notification/commentary/persistence concerns (separation of concerns).

---

## 2. Patterns used & why

| Pattern | Where | Why |
|---|---|---|
| **Observer** | `Match` (Subject) notifies `MatchObserver` implementations (`ScoreNotifier`, `CommentaryFeed`, `ScorecardCache`) | Core of the whole system: one scoring event → many independent things react (push notification, update commentary, refresh cache) without `Match` knowing about any of them. Classic pub-sub. |
| **State** | `MatchState` interface with `ScheduledState`, `TossState`, `LiveState`, `InningsBreakState`, `CompletedState` | Match behavior (what actions are legal — "can I record a ball?", "can I do a toss?") depends entirely on current state. State pattern avoids giant if/else chains and prevents illegal transitions (e.g., recording a ball before toss). |
| **Strategy** | `MatchFormat` interface with `T20Format`, `ODIFormat`, `TestFormat` | Format-specific rules (overs per innings, number of innings, follow-on) vary independently of scoring logic. Innings/over-limit checks delegate to the strategy instead of hardcoding `if format == T20`. |
| **Command** | `BallEvent` as a command object with `apply()` / `undo()`, stored in a stack per innings | Enables undoing the last delivery (common real requirement — mis-click correction, DRS reversal) without re-deriving state from scratch. |
| **Factory Method** | `MatchFactory.createMatch(format, teams, venue)` | Encapsulates which `MatchFormat` + initial `MatchState` to wire up for a given format. |
| **Singleton** | `MatchRegistry` | Single source of truth for "which matches are currently live" — used by controllers/notification dispatch to look up a match by ID. |
| **Builder** | `Player.Builder`, `Match.Builder` | Both have many optional fields (stats, playing role, toss details) — builder avoids telescoping constructors. |

**SOLID**
- **S**: `Match` only orchestrates state + delegates; `Innings` owns the score math; `CommentaryGenerator` only generates text; `ScoreNotifier` only pushes notifications. Each has one reason to change.
- **O**: New format → implement `MatchFormat`. New reaction to a ball event → implement `MatchObserver`. No existing class touched.
- **L**: Any `MatchState` is substitutable — `Match` calls `state.recordBall(...)` polymorphically; any `MatchFormat` is substitutable wherever overs/innings limits are checked.
- **I**: `MatchObserver` exposes only `onBallEvent`/`onStateChange` — observers aren't forced to implement scoring logic. `MatchFormat` is narrow (rules only), not mixed with scoring.
- **D**: `Match` depends on `MatchFormat` and `MatchState` abstractions, injected at creation — not on concrete `T20Format`/`LiveState`.

---

## 3. Class Diagram (textual)

```
┌────────────────────┐        ┌────────────────────────┐
│   MatchState        │◀──────│  Match (Context/Subject) │
│ (State interface)   │        │  - state: MatchState     │
│ + recordBall()       │        │  - format: MatchFormat   │
│ + startToss()        │        │  - innings: List<Innings>│
│ + transitionTo()      │        │  - observers: List<Obs>  │
└─────────▲────────────┘        │  + addBall(BallEvent)    │
          │                     │  + notifyObservers()     │
 ┌────────┼─────────┬──────────┬─────────────┐
 │        │          │          │             │
Scheduled Toss     Live    InningsBreak   Completed
 State    State    State      State          State

┌───────────────────┐         ┌──────────────────────┐
│  MatchFormat        │        │   MatchObserver         │
│ (Strategy interface)│        │ (Observer interface)    │
│ + oversPerInnings()  │       │ + onBallEvent(Ball)      │
│ + maxInnings()        │      │ + onStateChange(State)  │
└──────────▲──────────┘        └───────────▲─────────────┘
    ┌──────┼──────┐                        │
  T20Format ODIFormat TestFormat   ┌────────┼─────────────┐
                              ScoreNotifier CommentaryFeed ScorecardCache

┌──────────────────┐   contains   ┌──────────────┐   contains   ┌──────────┐
│ Innings            │────────────▶│ Over          │────────────▶│  Ball    │
│ - battingTeam       │             │ - overNumber   │             │ - runs   │
│ - score, wickets     │             │ - bowler       │             │ - extras │
│ - currentBatsmen     │             │ - balls[]      │             │ - wicket │
│ - ballHistory (Cmd stack)│         └──────────────┘             └──────────┘
└──────────────────┘

┌──────────────────┐          ┌──────────────────┐
│  BallEvent         │          │  MatchFactory      │
│ (Command interface) │         │  + createMatch()    │
│ + apply()            │        └──────────────────┘
│ + undo()              │
└──────────────────┘

┌──────────────────┐          ┌──────────────────┐
│  MatchRegistry      │        │   Player            │
│  (Singleton)         │        │  (Builder)           │
└──────────────────┘          └──────────────────┘
```

---

## 4. Code (Java)

### 4.1 Core domain models

```java
public enum PlayerRole { BATSMAN, BOWLER, ALL_ROUNDER, WICKET_KEEPER }

public class Player {
    private final String id;
    private final String name;
    private final PlayerRole role;
    private final BattingStats battingStats = new BattingStats();
    private final BowlingStats bowlingStats = new BowlingStats();

    private Player(Builder b) {
        this.id = b.id; this.name = b.name; this.role = b.role;
    }

    public static class Builder {
        private String id, name;
        private PlayerRole role;
        public Builder id(String id) { this.id = id; return this; }
        public Builder name(String n) { this.name = n; return this; }
        public Builder role(PlayerRole r) { this.role = r; return this; }
        public Player build() { return new Player(this); }
    }
    // getters omitted
}

public class BattingStats {
    int runs = 0, ballsFaced = 0, fours = 0, sixes = 0;
    boolean isOut = false;
}

public class BowlingStats {
    int ballsBowled = 0, runsConceded = 0, wickets = 0, maidens = 0;
}

public class Team {
    private final String id;
    private final String name;
    private final List<Player> squad;
    private List<Player> playingXI;
    // getters/setters omitted
}
```

### 4.2 Ball / Over (event data, not commands themselves)

```java
public enum ExtraType { NONE, WIDE, NO_BALL, BYE, LEG_BYE }
public enum DismissalType { BOWLED, CAUGHT, LBW, RUN_OUT, STUMPED, NONE }

public class WicketInfo {
    Player batsmanOut;
    Player fielder;      // nullable
    DismissalType type;
}

public class Ball {
    int runsScored;
    ExtraType extraType;
    WicketInfo wicketInfo;  // null if no wicket
    Player striker;
    Player bowler;

    public boolean isLegalDelivery() {
        return extraType != ExtraType.WIDE && extraType != ExtraType.NO_BALL;
    }
    public boolean isWicket() { return wicketInfo != null; }
}

public class Over {
    private final int overNumber;
    private final Player bowler;
    private final List<Ball> balls = new ArrayList<>();

    public Over(int overNumber, Player bowler) {
        this.overNumber = overNumber; this.bowler = bowler;
    }
    public void addBall(Ball b) { balls.add(b); }
    public boolean isComplete() {
        return balls.stream().filter(Ball::isLegalDelivery).count() == 6;
    }
}
```

### 4.3 Command pattern — BallEvent (supports undo)

```java
public interface BallEvent {
    void apply();
    void undo();
}

public class NormalBallEvent implements BallEvent {
    private final Innings innings;
    private final Ball ball;
    // snapshot for undo
    private int prevScore, prevWickets, prevStrikerRuns, prevStrikerBalls;

    public NormalBallEvent(Innings innings, Ball ball) {
        this.innings = innings; this.ball = ball;
    }

    @Override
    public void apply() {
        prevScore = innings.getScore();
        prevWickets = innings.getWickets();
        prevStrikerRuns = ball.striker.battingStats.runs;
        prevStrikerBalls = ball.striker.battingStats.ballsFaced;

        innings.applyBall(ball); // mutates score, stats, strike rotation, over
    }

    @Override
    public void undo() {
        innings.setScore(prevScore);
        innings.setWickets(prevWickets);
        ball.striker.battingStats.runs = prevStrikerRuns;
        ball.striker.battingStats.ballsFaced = prevStrikerBalls;
        innings.removeLastBall(ball);
    }
}
```

### 4.4 Innings — owns the actual score computation

```java
public class Innings {
    private final Team battingTeam, bowlingTeam;
    private int score = 0, wickets = 0;
    private final List<Over> overs = new ArrayList<>();
    private Player strikerBatsman, nonStrikerBatsman, currentBowler;
    private final Deque<BallEvent> history = new ArrayDeque<>(); // for undo

    public Innings(Team battingTeam, Team bowlingTeam) {
        this.battingTeam = battingTeam; this.bowlingTeam = bowlingTeam;
    }

    public void recordBall(Ball ball) {
        BallEvent event = new NormalBallEvent(this, ball);
        event.apply();
        history.push(event);
    }

    public void undoLastBall() {
        if (!history.isEmpty()) history.pop().undo();
    }

    // called internally by BallEvent.apply()
    void applyBall(Ball ball) {
        int runs = ball.runsScored;
        score += runs;
        if (ball.extraType == ExtraType.WIDE || ball.extraType == ExtraType.NO_BALL) score += 1;

        if (ball.isLegalDelivery()) {
            ball.striker.battingStats.runs += runs;
            ball.striker.battingStats.ballsFaced += 1;
            ball.bowler.bowlingStats.ballsBowled += 1;
            ball.bowler.bowlingStats.runsConceded += runs;
        }

        if (ball.isWicket()) {
            wickets += 1;
            ball.wicketInfo.batsmanOut.battingStats.isOut = true;
            ball.bowler.bowlingStats.wickets += 1;
        }

        getCurrentOver().addBall(ball);
        rotateStrikeIfNeeded(runs);
    }

    private void rotateStrikeIfNeeded(int runs) {
        if (runs % 2 != 0) swapStrike();
    }
    private void swapStrike() {
        Player tmp = strikerBatsman; strikerBatsman = nonStrikerBatsman; nonStrikerBatsman = tmp;
    }

    private Over getCurrentOver() {
        if (overs.isEmpty() || overs.get(overs.size() - 1).isComplete()) {
            overs.add(new Over(overs.size(), currentBowler));
        }
        return overs.get(overs.size() - 1);
    }

    void removeLastBall(Ball ball) { /* remove from current over's list */ }
    // getters/setters for score, wickets omitted
}
```

### 4.5 Strategy — MatchFormat

```java
public interface MatchFormat {
    int oversPerInnings();     // -1 for unlimited (Test)
    int maxInningsPerTeam();
    boolean supportsFollowOn();
}

public class T20Format implements MatchFormat {
    public int oversPerInnings() { return 20; }
    public int maxInningsPerTeam() { return 1; }
    public boolean supportsFollowOn() { return false; }
}

public class ODIFormat implements MatchFormat {
    public int oversPerInnings() { return 50; }
    public int maxInningsPerTeam() { return 1; }
    public boolean supportsFollowOn() { return false; }
}

public class TestFormat implements MatchFormat {
    public int oversPerInnings() { return -1; }
    public int maxInningsPerTeam() { return 2; }
    public boolean supportsFollowOn() { return true; }
}
```

### 4.6 Observer pattern — Match as Subject

```java
public interface MatchObserver {
    void onBallEvent(Match match, Ball ball);
    void onStateChange(Match match, String newState);
}

public class ScoreNotifier implements MatchObserver {
    @Override
    public void onBallEvent(Match match, Ball ball) {
        // push to subscribed users' devices (WebSocket/FCM etc.)
        System.out.println("[Push] " + match.getId() + ": " + summarize(ball));
    }
    @Override
    public void onStateChange(Match match, String newState) {
        System.out.println("[Push] Match " + match.getId() + " -> " + newState);
    }
    private String summarize(Ball b) { return b.runsScored + " run(s)"; }
}

public class CommentaryFeed implements MatchObserver {
    private final List<String> commentary = new ArrayList<>();
    @Override
    public void onBallEvent(Match match, Ball ball) {
        commentary.add(generateLine(ball));
    }
    @Override
    public void onStateChange(Match match, String newState) {
        commentary.add("Match state: " + newState);
    }
    private String generateLine(Ball b) {
        if (b.isWicket()) return "WICKET! " + b.wicketInfo.batsmanOut.getClass();
        return b.striker.getClass() + " scores " + b.runsScored;
    }
}

public class ScorecardCache implements MatchObserver {
    @Override
    public void onBallEvent(Match match, Ball ball) {
        // refresh cached scorecard snapshot for fast reads by many clients
    }
    @Override
    public void onStateChange(Match match, String newState) { }
}
```

### 4.7 State pattern — Match lifecycle

```java
public interface MatchState {
    void startToss(Match match);
    void recordBall(Match match, Ball ball);
    void endInnings(Match match);
    String name();
}

public class ScheduledState implements MatchState {
    public void startToss(Match match) { match.setState(new TossState()); }
    public void recordBall(Match match, Ball ball) {
        throw new IllegalStateException("Cannot record ball before toss/live");
    }
    public void endInnings(Match match) { throw new IllegalStateException("Match not started"); }
    public String name() { return "SCHEDULED"; }
}

public class TossState implements MatchState {
    public void startToss(Match match) { throw new IllegalStateException("Toss already in progress"); }
    public void recordBall(Match match, Ball ball) { throw new IllegalStateException("Toss not concluded"); }
    public void endInnings(Match match) { throw new IllegalStateException("Match not live"); }
    public String name() { return "TOSS"; }
    // a separate method concludeToss(match) would transition to LiveState
}

public class LiveState implements MatchState {
    public void startToss(Match match) { throw new IllegalStateException("Already live"); }
    public void recordBall(Match match, Ball ball) {
        match.getCurrentInnings().recordBall(ball);
        match.notifyBallEvent(ball);
        if (match.isInningsComplete()) {
            endInnings(match);
        }
    }
    public void endInnings(Match match) {
        if (match.hasMoreInnings()) {
            match.setState(new InningsBreakState());
        } else {
            match.setState(new CompletedState());
        }
        match.notifyStateChange();
    }
    public String name() { return "LIVE"; }
}

public class InningsBreakState implements MatchState {
    public void startToss(Match match) { throw new IllegalStateException("Toss already done"); }
    public void recordBall(Match match, Ball ball) { throw new IllegalStateException("Innings break in progress"); }
    public void endInnings(Match match) { match.startNextInnings(); match.setState(new LiveState()); }
    public String name() { return "INNINGS_BREAK"; }
}

public class CompletedState implements MatchState {
    public void startToss(Match match) { throw new IllegalStateException("Match completed"); }
    public void recordBall(Match match, Ball ball) { throw new IllegalStateException("Match completed"); }
    public void endInnings(Match match) { throw new IllegalStateException("Match completed"); }
    public String name() { return "COMPLETED"; }
}
```

### 4.8 Match — Context + Subject, wired via Strategy/State/Observer

```java
public class Match {
    private final String id;
    private final Team teamA, teamB;
    private final Venue venue;
    private final MatchFormat format;             // Strategy
    private MatchState state;                       // State
    private final List<Innings> innings = new ArrayList<>();
    private int currentInningsIndex = -1;
    private final List<MatchObserver> observers = new ArrayList<>(); // Observer

    public Match(String id, Team a, Team b, Venue v, MatchFormat format) {
        this.id = id; this.teamA = a; this.teamB = b; this.venue = v;
        this.format = format;
        this.state = new ScheduledState();
    }

    public void subscribe(MatchObserver o) { observers.add(o); }
    public void unsubscribe(MatchObserver o) { observers.remove(o); }

    public void notifyBallEvent(Ball ball) {
        for (MatchObserver o : observers) o.onBallEvent(this, ball);
    }
    public void notifyStateChange() {
        for (MatchObserver o : observers) o.onStateChange(this, state.name());
    }

    // delegated to current state — illegal actions throw from within state
    public void startToss() { state.startToss(this); }
    public void recordBall(Ball ball) { state.recordBall(this, ball); }

    void setState(MatchState s) { this.state = s; notifyStateChange(); }
    Innings getCurrentInnings() { return innings.get(currentInningsIndex); }

    boolean isInningsComplete() {
        Innings cur = getCurrentInnings();
        int oversLimit = format.oversPerInnings();
        boolean allOut = cur.getWickets() >= 10;
        boolean oversUp = oversLimit != -1 && cur.getOversCompleted() >= oversLimit;
        return allOut || oversUp;
    }

    boolean hasMoreInnings() {
        return innings.size() < format.maxInningsPerTeam() * 2;
    }

    void startNextInnings() {
        currentInningsIndex++;
        Team batting = (currentInningsIndex % 2 == 0) ? teamA : teamB;
        Team bowling = (batting == teamA) ? teamB : teamA;
        innings.add(new Innings(batting, bowling));
    }

    public String getId() { return id; }
}
```

### 4.9 Factory Method

```java
public class MatchFactory {
    public static Match createMatch(String matchId, Team a, Team b, Venue venue, String formatType) {
        MatchFormat format;
        switch (formatType) {
            case "T20": format = new T20Format(); break;
            case "ODI": format = new ODIFormat(); break;
            case "TEST": format = new TestFormat(); break;
            default: throw new IllegalArgumentException("Unknown format: " + formatType);
        }
        Match match = new Match(matchId, a, b, venue, format);
        // wire default observers
        match.subscribe(new ScoreNotifier());
        match.subscribe(new CommentaryFeed());
        match.subscribe(new ScorecardCache());
        return match;
    }
}
```

### 4.10 Singleton — MatchRegistry

```java
public class MatchRegistry {
    private static volatile MatchRegistry instance;
    private final ConcurrentHashMap<String, Match> liveMatches = new ConcurrentHashMap<>();

    private MatchRegistry() {}

    public static MatchRegistry getInstance() {
        if (instance == null) {
            synchronized (MatchRegistry.class) {
                if (instance == null) instance = new MatchRegistry();
            }
        }
        return instance;
    }

    public void register(Match match) { liveMatches.put(match.getId(), match); }
    public Match get(String matchId) { return liveMatches.get(matchId); }
    public Collection<Match> getAllLive() { return liveMatches.values(); }
}
```

### 4.11 Putting it together

```java
public class CricbuzzDemo {
    public static void main(String[] args) {
        Team india = new Team(/* ... */);
        Team australia = new Team(/* ... */);
        Venue mcg = new Venue(/* ... */);

        Match match = MatchFactory.createMatch("M100", india, australia, mcg, "T20");
        MatchRegistry.getInstance().register(match);

        match.startToss();
        // ... toss logic transitions to LiveState, startNextInnings() called ...

        Ball ball = new Ball();
        ball.runsScored = 4;
        ball.extraType = ExtraType.NONE;
        // set striker/bowler...

        match.recordBall(ball); // triggers Innings update + notifies all observers
    }
}
```

---

## 5. Why this shape holds up under follow-ups

- **"Add a live win-probability predictor"** → just another `MatchObserver` implementation. Zero changes to `Match`, `Innings`, or existing observers.
- **"Support The Hundred (100-ball format)"** → new `MatchFormat` implementation. Scoring/state logic untouched.
- **"Add DRS review that can overturn last ball's wicket"** → `undoLastBall()` already exists via the Command stack; extend with a `ReviewOverturnEvent` command.
- **"Rain delay"** → new `RainDelayState` implementing `MatchState`, transitions in/out of `LiveState`.
- **Scaling reads** (millions of users watching one match) → `ScorecardCache` observer means score reads never hit the live scoring path; can swap to Redis-backed cache without touching `Match`.

---

Want me to extend this with **partnership tracking, run-rate/required-run-rate calculation, or a distributed pub-sub design (Kafka/WebSocket fan-out) for the notification layer**, or move to a different LLD problem?