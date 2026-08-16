/**
 * LeetCode Top Interview 150 -- #67. LRU Cache (Medium)
 *
 * Design a Least Recently Used (LRU) cache with get(key) and put(key,
 * value), both O(1), evicting the least recently used entry when capacity
 * is exceeded.
 *
 * Example:
 *   LRUCache cache = new LRUCache(2);
 *   cache.put(1,1); cache.put(2,2);
 *   cache.get(1);       // 1
 *   cache.put(3,3);     // evicts key 2
 *   cache.get(2);       // -1 (not found)
 */
public class P67_LRUCache {

    static class LRUCache {
        private final int capacity;
        private final java.util.LinkedHashMap<Integer, Integer> map;

        public LRUCache(int capacity) {
            this.capacity = capacity;
            this.map = new java.util.LinkedHashMap<>(capacity, 0.75f, true) {
                @Override
                protected boolean removeEldestEntry(java.util.Map.Entry<Integer, Integer> eldest) {
                    return size() > LRUCache.this.capacity;
                }
            };
        }

        public int get(int key) {
            return map.getOrDefault(key, -1);
        }

        public void put(int key, int value) {
            map.put(key, value);
        }
    }

    public static void main(String[] args) {
        LRUCache cache = new LRUCache(2);
        cache.put(1, 1);
        cache.put(2, 2);
        check(cache.get(1), 1, "get(1)");
        cache.put(3, 3);
        check(cache.get(2), -1, "get(2) after eviction");
        cache.put(4, 4);
        check(cache.get(1), -1, "get(1) after eviction");
        check(cache.get(3), 3, "get(3)");
        check(cache.get(4), 4, "get(4)");
        System.out.println("All tests passed.");
    }

    private static void check(int actual, int expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
