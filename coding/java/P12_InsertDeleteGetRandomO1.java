/**
 * LeetCode Top Interview 150 -- #12. Insert Delete GetRandom O(1) (Medium)
 *
 * Design a data structure supporting insert(val), remove(val), and
 * getRandom() (returns a random existing element with equal probability),
 * all in average O(1) time.
 *
 * Example:
 *   RandomizedSet rs = new RandomizedSet();
 *   rs.insert(1);   // true
 *   rs.remove(2);   // false
 *   rs.insert(2);   // true
 *   rs.getRandom(); // returns 1 or 2 randomly
 *   rs.remove(1);   // true
 *   rs.insert(2);   // false (already present)
 */
public class P12_InsertDeleteGetRandomO1 {

    static class RandomizedSet {
        private final java.util.List<Integer> values = new java.util.ArrayList<>();
        private final java.util.Map<Integer, Integer> indexOf = new java.util.HashMap<>();
        private final java.util.Random random = new java.util.Random();

        public boolean insert(int val) {
            if (indexOf.containsKey(val)) return false;
            indexOf.put(val, values.size());
            values.add(val);
            return true;
        }

        public boolean remove(int val) {
            if (!indexOf.containsKey(val)) return false;
            int idx = indexOf.get(val);
            int lastVal = values.get(values.size() - 1);
            values.set(idx, lastVal);
            indexOf.put(lastVal, idx);
            values.remove(values.size() - 1);
            indexOf.remove(val);
            return true;
        }

        public int getRandom() {
            return values.get(random.nextInt(values.size()));
        }
    }

    public static void main(String[] args) {
        RandomizedSet rs = new RandomizedSet();
        test(rs.insert(1), true, "insert(1)");
        test(rs.remove(2), false, "remove(2) absent");
        test(rs.insert(2), true, "insert(2)");
        int r = rs.getRandom();
        if (r != 1 && r != 2) {
            throw new AssertionError("getRandom() out of range: " + r);
        }
        System.out.println("PASS: getRandom() -> " + r);
        test(rs.remove(1), true, "remove(1)");
        test(rs.insert(2), false, "insert(2) duplicate");
        System.out.println("All tests passed.");
    }

    private static void test(boolean actual, boolean expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
