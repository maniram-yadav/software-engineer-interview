/**
 * Grind 169 -- #362. Design Hit Counter (Medium, LeetCode Premium)
 *
 * Design a hit counter that counts hits received in the past 5 minutes,
 * supporting hit(timestamp) and getHits(timestamp), with timestamps
 * monotonically increasing.
 *
 * Example:
 *   hc.hit(1); hc.hit(2); hc.hit(3);
 *   hc.getHits(4); // 3
 *   hc.hit(300);
 *   hc.getHits(300); // 4
 *   hc.getHits(301); // 3
 */
public class P362_DesignHitCounter {

    static class HitCounter {
        private final java.util.Deque<Integer> hits = new java.util.ArrayDeque<>();

        public void hit(int timestamp) {
            hits.addLast(timestamp);
        }

        public int getHits(int timestamp) {
            while (!hits.isEmpty() && hits.peekFirst() <= timestamp - 300) hits.pollFirst();
            return hits.size();
        }
    }

    public static void main(String[] args) {
        HitCounter hc = new HitCounter();
        hc.hit(1);
        hc.hit(2);
        hc.hit(3);
        check(hc.getHits(4), 3, "getHits(4)");
        hc.hit(300);
        check(hc.getHits(300), 4, "getHits(300)");
        check(hc.getHits(301), 3, "getHits(301)");
        System.out.println("All tests passed.");
    }

    private static void check(int actual, int expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
