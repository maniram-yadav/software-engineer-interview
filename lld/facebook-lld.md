# Social Network (Facebook-like) — LLD Design

## 1. Requirements

**Functional:**
- User signup, profile management, friend requests (send/accept/reject)
- Create posts (text/image/video) with privacy settings (Public, Friends-only, Private)
- Like, comment (with nested replies) on posts
- News feed showing friends' posts, ranked by some algorithm
- Notifications (like, comment, friend request, tag) delivered to relevant users
- Search users

**Non-functional:**
- Feed ranking algorithm should be swappable (chronological vs engagement-based)
- Privacy/visibility rules should be pluggable per post
- Notification delivery decoupled from the action that triggered it
- Comment threads should support arbitrary nesting

---

## 2. Design Patterns Used (and why)

| Pattern | Where | Why |
|---|---|---|
| **Observer** | `User`/`NotificationService` observe actions on `Post` (like, comment) | Decouples the action (liking a post) from all the parties who need to react (notify author, update feed cache) — new reactions can be added without touching `Post` |
| **Strategy** | `FeedRankingStrategy` (Chronological, EdgeRank/Engagement); `PrivacyStrategy` (Public, FriendsOnly, Private) | Ranking algorithm and visibility rules vary independently and must change without touching `NewsFeedGenerator`/`Post` |
| **Composite** | `Comment` containing nested `Comment` replies | Comment threads are naturally a tree — treat a comment and a reply uniformly |
| **Factory** | `NotificationFactory` creates `LikeNotification`, `CommentNotification`, `FriendRequestNotification`, `TagNotification` | Centralizes creation logic for varying notification types |
| **Builder** | `PostBuilder` | Post has many optional fields (media, tags, privacy, location) — avoids telescoping constructors |
| **State** | `FriendRequestState` (Pending, Accepted, Rejected) | Friend request has valid/invalid transitions per state, avoids scattered conditionals |
| **Singleton** | `SocialGraphService` | Single source of truth for the friendship graph across the system |
| **Decorator** | `PostContent` (TextContent, wrapped by `ImageContentDecorator`, `VideoContentDecorator`) | Post content types can combine/stack without an explosion of subclasses |

---

## 3. SOLID Mapping

- **SRP** — `Post` manages content/metadata only; `NotificationService` only handles notification dispatch; `FeedRankingStrategy` only ranks.
- **OCP** — New ranking algorithms, privacy rules, or notification types plug in via new implementations without modifying existing classes.
- **LSP** — Any `PrivacyStrategy`/`FeedRankingStrategy`/`FriendRequestState` is substitutable — callers don't care about the concrete type.
- **ISP** — `PostObserver` only has `onPostEvent`; `NotificationObserver` only has what it needs — no fat interfaces.
- **DIP** — `NewsFeedGenerator` depends on `FeedRankingStrategy` interface; `Post` depends on `PrivacyStrategy` interface, not concrete checks.

---

## 4. Class Diagram (textual)

```
Enums: PrivacyType, NotificationType, FriendRequestStatus

User
 - id, name, email
 - friends: Set<User>, friendRequests: List<FriendRequest>
 - notifications: List<Notification>
 + sendFriendRequest(), acceptFriendRequest(), createPost()

SocialGraphService (Singleton)
 - friendships: Map<userId, Set<userId>>
 + addFriendship(), areFriends(), getFriends()

FriendRequest
 - sender, receiver, state: FriendRequestState
 + accept(), reject()

FriendRequestState (interface)
 ├── PendingState, AcceptedState, RejectedState

PostContent (interface) — Decorator base
 ├── TextContent
 ├── ImageContentDecorator (wraps PostContent)
 └── VideoContentDecorator (wraps PostContent)

Post
 - id, author, content: PostContent, privacy: PrivacyStrategy
 - likes: Set<User>, comments: List<Comment>
 - observers: List<PostObserver>
 + like(), addComment(), notifyObservers()

PostBuilder → builds Post

Comment (Composite)
 - id, author, text, replies: List<Comment>
 + addReply(), getAllReplies()

PrivacyStrategy (interface)
 + isVisible(Post, viewer): boolean
 ├── PublicPrivacy, FriendsOnlyPrivacy, PrivateOnlyMePrivacy

PostObserver (interface)
 + onPostEvent(Post, eventType, actor)
 ├── NotificationTrigger
 └── FeedCacheUpdater

Notification (abstract)
 ├── LikeNotification, CommentNotification,
 │    FriendRequestNotification, TagNotification
NotificationFactory → creates Notification
NotificationService
 + send(User, Notification)

FeedRankingStrategy (interface)
 + rank(List<Post>): List<Post>
 ├── ChronologicalStrategy, EngagementStrategy

NewsFeedGenerator
 - rankingStrategy: FeedRankingStrategy
 + generateFeed(User): List<Post>
```

---

## 5. Code (Java)

### Enums

```java
public enum PrivacyType { PUBLIC, FRIENDS_ONLY, PRIVATE }
public enum NotificationType { LIKE, COMMENT, FRIEND_REQUEST, TAG }
public enum FriendRequestStatus { PENDING, ACCEPTED, REJECTED }
public enum PostEventType { LIKE, COMMENT, TAG }
```

### SocialGraphService (Singleton)

```java
import java.util.*;

public class SocialGraphService {
    private static SocialGraphService instance;
    private final Map<String, Set<String>> friendships = new HashMap<>();

    private SocialGraphService() {}

    public static synchronized SocialGraphService getInstance() {
        if (instance == null) instance = new SocialGraphService();
        return instance;
    }

    public void addFriendship(String userId1, String userId2) {
        friendships.computeIfAbsent(userId1, k -> new HashSet<>()).add(userId2);
        friendships.computeIfAbsent(userId2, k -> new HashSet<>()).add(userId1);
    }

    public boolean areFriends(String userId1, String userId2) {
        return friendships.getOrDefault(userId1, Collections.emptySet()).contains(userId2);
    }

    public Set<String> getFriends(String userId) {
        return friendships.getOrDefault(userId, Collections.emptySet());
    }
}
```

### FriendRequest (State pattern)

```java
public interface FriendRequestState {
    void accept(FriendRequest request);
    void reject(FriendRequest request);
    FriendRequestStatus getStatus();
}

public class PendingState implements FriendRequestState {
    @Override
    public void accept(FriendRequest request) {
        SocialGraphService.getInstance().addFriendship(
            request.getSender().getId(), request.getReceiver().getId());
        request.setState(new AcceptedState());
    }
    @Override
    public void reject(FriendRequest request) { request.setState(new RejectedState()); }
    @Override
    public FriendRequestStatus getStatus() { return FriendRequestStatus.PENDING; }
}

public class AcceptedState implements FriendRequestState {
    @Override public void accept(FriendRequest request) { /* no-op, already accepted */ }
    @Override public void reject(FriendRequest request) {
        throw new IllegalStateException("Cannot reject an accepted request");
    }
    @Override public FriendRequestStatus getStatus() { return FriendRequestStatus.ACCEPTED; }
}

public class RejectedState implements FriendRequestState {
    @Override public void accept(FriendRequest request) {
        throw new IllegalStateException("Cannot accept a rejected request");
    }
    @Override public void reject(FriendRequest request) { /* no-op */ }
    @Override public FriendRequestStatus getStatus() { return FriendRequestStatus.REJECTED; }
}

public class FriendRequest {
    private final User sender;
    private final User receiver;
    private FriendRequestState state = new PendingState();

    public FriendRequest(User sender, User receiver) {
        this.sender = sender;
        this.receiver = receiver;
    }

    public void accept() { state.accept(this); }
    public void reject() { state.reject(this); }
    public void setState(FriendRequestState state) { this.state = state; }
    public User getSender() { return sender; }
    public User getReceiver() { return receiver; }
    public FriendRequestStatus getStatus() { return state.getStatus(); }
}
```

### PostContent (Decorator pattern)

```java
public interface PostContent {
    String render();
}

public class TextContent implements PostContent {
    private final String text;
    public TextContent(String text) { this.text = text; }
    @Override public String render() { return text; }
}

public abstract class PostContentDecorator implements PostContent {
    protected final PostContent wrapped;
    protected PostContentDecorator(PostContent wrapped) { this.wrapped = wrapped; }
}

public class ImageContentDecorator extends PostContentDecorator {
    private final String imageUrl;
    public ImageContentDecorator(PostContent wrapped, String imageUrl) {
        super(wrapped);
        this.imageUrl = imageUrl;
    }
    @Override public String render() { return wrapped.render() + " [Image: " + imageUrl + "]"; }
}

public class VideoContentDecorator extends PostContentDecorator {
    private final String videoUrl;
    public VideoContentDecorator(PostContent wrapped, String videoUrl) {
        super(wrapped);
        this.videoUrl = videoUrl;
    }
    @Override public String render() { return wrapped.render() + " [Video: " + videoUrl + "]"; }
}
```

### PrivacyStrategy (Strategy pattern)

```java
public interface PrivacyStrategy {
    boolean isVisible(Post post, User viewer);
}

public class PublicPrivacy implements PrivacyStrategy {
    @Override public boolean isVisible(Post post, User viewer) { return true; }
}

public class FriendsOnlyPrivacy implements PrivacyStrategy {
    @Override public boolean isVisible(Post post, User viewer) {
        return post.getAuthor().equals(viewer) ||
            SocialGraphService.getInstance().areFriends(post.getAuthor().getId(), viewer.getId());
    }
}

public class PrivateOnlyMePrivacy implements PrivacyStrategy {
    @Override public boolean isVisible(Post post, User viewer) {
        return post.getAuthor().equals(viewer);
    }
}
```

### Comment (Composite pattern)

```java
import java.util.*;

public class Comment {
    private final String id;
    private final User author;
    private final String text;
    private final List<Comment> replies = new ArrayList<>();

    public Comment(String id, User author, String text) {
        this.id = id;
        this.author = author;
        this.text = text;
    }

    public void addReply(Comment reply) { replies.add(reply); }

    public List<Comment> getAllReplies() {
        // flattens the tree if needed
        List<Comment> all = new ArrayList<>(replies);
        for (Comment r : replies) all.addAll(r.getAllReplies());
        return all;
    }

    public User getAuthor() { return author; }
    public String getText() { return text; }
    public List<Comment> getReplies() { return replies; }
}
```

### PostObserver (Observer pattern)

```java
public interface PostObserver {
    void onPostEvent(Post post, PostEventType eventType, User actor);
}

public class NotificationTrigger implements PostObserver {
    private final NotificationService notificationService;
    public NotificationTrigger(NotificationService service) { this.notificationService = service; }

    @Override
    public void onPostEvent(Post post, PostEventType eventType, User actor) {
        if (post.getAuthor().equals(actor)) return; // don't notify self
        NotificationType type = switch (eventType) {
            case LIKE -> NotificationType.LIKE;
            case COMMENT -> NotificationType.COMMENT;
            case TAG -> NotificationType.TAG;
        };
        Notification notification = NotificationFactory.create(type, actor, post);
        notificationService.send(post.getAuthor(), notification);
    }
}

public class FeedCacheUpdater implements PostObserver {
    @Override
    public void onPostEvent(Post post, PostEventType eventType, User actor) {
        // invalidate/update cached feed entries for post.getAuthor()'s friends
        System.out.println("Feed cache invalidated for author: " + post.getAuthor().getName());
    }
}
```

### Notification (Factory pattern)

```java
public abstract class Notification {
    protected final User actor;
    protected final String message;
    protected Notification(User actor, String message) {
        this.actor = actor;
        this.message = message;
    }
    public String getMessage() { return message; }
}

public class LikeNotification extends Notification {
    public LikeNotification(User actor, Post post) {
        super(actor, actor.getName() + " liked your post");
    }
}

public class CommentNotification extends Notification {
    public CommentNotification(User actor, Post post) {
        super(actor, actor.getName() + " commented on your post");
    }
}

public class FriendRequestNotification extends Notification {
    public FriendRequestNotification(User actor) {
        super(actor, actor.getName() + " sent you a friend request");
    }
}

public class TagNotification extends Notification {
    public TagNotification(User actor, Post post) {
        super(actor, actor.getName() + " tagged you in a post");
    }
}

public class NotificationFactory {
    public static Notification create(NotificationType type, User actor, Post post) {
        return switch (type) {
            case LIKE -> new LikeNotification(actor, post);
            case COMMENT -> new CommentNotification(actor, post);
            case FRIEND_REQUEST -> new FriendRequestNotification(actor);
            case TAG -> new TagNotification(actor, post);
        };
    }
}

public class NotificationService {
    public void send(User user, Notification notification) {
        user.receiveNotification(notification);
        System.out.println("[Notify " + user.getName() + "]: " + notification.getMessage());
    }
}
```

### Post (Subject in Observer pattern)

```java
import java.util.*;

public class Post {
    private final String id;
    private final User author;
    private final PostContent content;
    private final PrivacyStrategy privacy;
    private final Set<User> likes = new HashSet<>();
    private final List<Comment> comments = new ArrayList<>();
    private final List<PostObserver> observers = new ArrayList<>();
    private final long timestamp = System.currentTimeMillis();

    public Post(String id, User author, PostContent content, PrivacyStrategy privacy) {
        this.id = id;
        this.author = author;
        this.content = content;
        this.privacy = privacy;
    }

    public void addObserver(PostObserver o) { observers.add(o); }
    private void notifyObservers(PostEventType type, User actor) {
        for (PostObserver o : observers) o.onPostEvent(this, type, actor);
    }

    public void like(User user) {
        likes.add(user);
        notifyObservers(PostEventType.LIKE, user);
    }

    public void addComment(Comment comment) {
        comments.add(comment);
        notifyObservers(PostEventType.COMMENT, comment.getAuthor());
    }

    public boolean isVisibleTo(User viewer) { return privacy.isVisible(this, viewer); }

    public User getAuthor() { return author; }
    public PostContent getContent() { return content; }
    public long getTimestamp() { return timestamp; }
    public int getLikeCount() { return likes.size(); }
    public int getCommentCount() { return comments.size(); }
}
```

### PostBuilder (Builder pattern)

```java
public class PostBuilder {
    private String id;
    private User author;
    private PostContent content;
    private PrivacyStrategy privacy = new PublicPrivacy(); // default

    public PostBuilder setId(String id) { this.id = id; return this; }
    public PostBuilder setAuthor(User author) { this.author = author; return this; }
    public PostBuilder setContent(PostContent content) { this.content = content; return this; }
    public PostBuilder setPrivacy(PrivacyStrategy privacy) { this.privacy = privacy; return this; }

    public Post build() {
        if (author == null || content == null) {
            throw new IllegalStateException("Post requires author and content");
        }
        Post post = new Post(id, author, content, privacy);
        NotificationService notificationService = new NotificationService();
        post.addObserver(new NotificationTrigger(notificationService));
        post.addObserver(new FeedCacheUpdater());
        return post;
    }
}
```

### FeedRankingStrategy (Strategy pattern)

```java
import java.util.*;

public interface FeedRankingStrategy {
    List<Post> rank(List<Post> posts);
}

public class ChronologicalStrategy implements FeedRankingStrategy {
    @Override
    public List<Post> rank(List<Post> posts) {
        List<Post> sorted = new ArrayList<>(posts);
        sorted.sort((a, b) -> Long.compare(b.getTimestamp(), a.getTimestamp()));
        return sorted;
    }
}

public class EngagementStrategy implements FeedRankingStrategy {
    @Override
    public List<Post> rank(List<Post> posts) {
        List<Post> sorted = new ArrayList<>(posts);
        sorted.sort((a, b) -> {
            int scoreA = a.getLikeCount() * 2 + a.getCommentCount() * 3;
            int scoreB = b.getLikeCount() * 2 + b.getCommentCount() * 3;
            return Integer.compare(scoreB, scoreA);
        });
        return sorted;
    }
}
```

### NewsFeedGenerator

```java
import java.util.*;
import java.util.stream.*;

public class NewsFeedGenerator {
    private FeedRankingStrategy rankingStrategy;
    private final Map<String, List<Post>> userPosts; // simplified in-memory post store

    public NewsFeedGenerator(FeedRankingStrategy rankingStrategy, Map<String, List<Post>> userPosts) {
        this.rankingStrategy = rankingStrategy;
        this.userPosts = userPosts;
    }

    public void setRankingStrategy(FeedRankingStrategy strategy) { this.rankingStrategy = strategy; }

    public List<Post> generateFeed(User viewer) {
        Set<String> friendIds = SocialGraphService.getInstance().getFriends(viewer.getId());
        List<Post> candidatePosts = friendIds.stream()
            .flatMap(fid -> userPosts.getOrDefault(fid, Collections.emptyList()).stream())
            .filter(post -> post.isVisibleTo(viewer))
            .collect(Collectors.toList());

        return rankingStrategy.rank(candidatePosts);
    }
}
```

### User

```java
import java.util.*;

public class User {
    private final String id;
    private final String name;
    private final String email;
    private final List<Notification> notifications = new ArrayList<>();
    private final List<FriendRequest> sentRequests = new ArrayList<>();

    public User(String id, String name, String email) {
        this.id = id;
        this.name = name;
        this.email = email;
    }

    public FriendRequest sendFriendRequest(User receiver) {
        FriendRequest request = new FriendRequest(this, receiver);
        sentRequests.add(request);
        NotificationService service = new NotificationService();
        service.send(receiver, NotificationFactory.create(NotificationType.FRIEND_REQUEST, this, null));
        return request;
    }

    public void receiveNotification(Notification n) { notifications.add(n); }

    public String getId() { return id; }
    public String getName() { return name; }
}
```

### Usage

```java
public class Main {
    public static void main(String[] args) {
        User alice = new User("u1", "Alice", "alice@mail.com");
        User bob = new User("u2", "Bob", "bob@mail.com");

        FriendRequest request = alice.sendFriendRequest(bob);
        request.accept(); // now friends via SocialGraphService

        PostContent content = new ImageContentDecorator(new TextContent("Loving this sunset!"), "sunset.jpg");
        Post post = new PostBuilder()
            .setId("p1")
            .setAuthor(alice)
            .setContent(content)
            .setPrivacy(new FriendsOnlyPrivacy())
            .build();

        post.like(bob);
        post.addComment(new Comment("c1", bob, "Beautiful!"));

        Map<String, List<Post>> allPosts = new HashMap<>();
        allPosts.put(alice.getId(), List.of(post));

        NewsFeedGenerator feedGenerator = new NewsFeedGenerator(new EngagementStrategy(), allPosts);
        List<Post> bobFeed = feedGenerator.generateFeed(bob);
        System.out.println("Bob's feed size: " + bobFeed.size());
    }
}
```

---

## 6. Extensibility Notes

- **New content type** (poll, live video, GIF) → new `PostContentDecorator`, no change to `Post`.
- **New privacy rule** (Close Friends, Custom list) → implement `PrivacyStrategy`.
- **New feed ranking** (ML-based relevance score) → implement `FeedRankingStrategy`, swap at runtime per user A/B test.
- **New notification channel** (push, email, SMS) → keep `NotificationService.send()` interface, add `NotificationChannel` strategy internally (Strategy within Factory output).
- **Group/Page support** → `Group`/`Page` can extend a common `Publisher` abstraction that `Post` references instead of just `User`, keeping `Post` decoupled from "who can post."
- **Blocking users** → add a `BlockStrategy` check composed with `PrivacyStrategy` (Chain of Responsibility or simple AND-composition of strategies).
- **Tagging in comments/photos** → reuse `TagNotification` + observer pattern already in place, no core changes needed.

Want me to go deeper on any part — e.g., **real-time notification delivery via WebSocket/pub-sub**, **feed pagination & caching (cursor-based)**, **mutual friends / friend suggestion algorithm (graph BFS)**, or **rate limiting for post creation/spam prevention**?