/**
 * Grind 169 -- #981. Time Based Key-Value Store (Medium)
 *
 * Design a time-based key-value store: set(key, value, timestamp) stores
 * the value, and get(key, timestamp) returns the value set at the largest
 * timestamp <= the given timestamp.
 *
 * Example:
 *   tkv.set("foo","bar",1);
 *   tkv.get("foo",1); // "bar"
 *   tkv.get("foo",3); // "bar"
 *   tkv.set("foo","bar2",4);
 *   tkv.get("foo",4); // "bar2"
 */
public class P981_TimeBasedKeyValueStore {

    static class TimeMap {
        static class Entry {
            int timestamp;
            String value;

            Entry(int timestamp, String value) {
                this.timestamp = timestamp;
                this.value = value;
            }
        }

        private final java.util.Map<String, java.util.List<Entry>> store = new java.util.HashMap<>();

        public void set(String key, String value, int timestamp) {
            store.computeIfAbsent(key, k -> new java.util.ArrayList<>()).add(new Entry(timestamp, value));
        }

        public String get(String key, int timestamp) {
            java.util.List<Entry> entries = store.get(key);
            if (entries == null) return "";
            int left = 0, right = entries.size() - 1, result = -1;
            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (entries.get(mid).timestamp <= timestamp) {
                    result = mid;
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            return result == -1 ? "" : entries.get(result).value;
        }
    }

    public static void main(String[] args) {
        TimeMap tkv = new TimeMap();
        tkv.set("foo", "bar", 1);
        check(tkv.get("foo", 1), "bar", "get(foo,1)");
        check(tkv.get("foo", 3), "bar", "get(foo,3)");
        tkv.set("foo", "bar2", 4);
        check(tkv.get("foo", 4), "bar2", "get(foo,4)");
        check(tkv.get("foo", 5), "bar2", "get(foo,5)");
        check(tkv.get("bar", 1), "", "get(missing key)");
        System.out.println("All tests passed.");
    }

    private static void check(String actual, String expected, String label) {
        if (!actual.equals(expected)) {
            throw new AssertionError(label + ": expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: " + label + " -> \"" + actual + "\"");
    }
}
