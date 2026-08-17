/**
 * Grind 169 -- #895. Maximum Frequency Stack (Hard)
 *
 * Design a stack-like data structure FreqStack where pop() removes and
 * returns the most frequent element, breaking ties by most recently
 * pushed.
 *
 * Example:
 *   FreqStack fs = new FreqStack();
 *   fs.push(5); fs.push(7); fs.push(5); fs.push(7); fs.push(4); fs.push(5);
 *   fs.pop(); // 5 (most frequent, tie broken by recency)
 */
public class P895_MaximumFrequencyStack {

    static class FreqStack {
        private final java.util.Map<Integer, Integer> freq = new java.util.HashMap<>();
        private final java.util.Map<Integer, java.util.Deque<Integer>> group = new java.util.HashMap<>();
        private int maxFreq = 0;

        public void push(int val) {
            int f = freq.merge(val, 1, Integer::sum);
            maxFreq = Math.max(maxFreq, f);
            group.computeIfAbsent(f, k -> new java.util.ArrayDeque<>()).push(val);
        }

        public int pop() {
            int val = group.get(maxFreq).pop();
            freq.merge(val, -1, Integer::sum);
            if (group.get(maxFreq).isEmpty()) maxFreq--;
            return val;
        }
    }

    public static void main(String[] args) {
        FreqStack fs = new FreqStack();
        fs.push(5);
        fs.push(7);
        fs.push(5);
        fs.push(7);
        fs.push(4);
        fs.push(5);
        check(fs.pop(), 5, "pop() most frequent");
        check(fs.pop(), 7, "pop() tie broken by recency");
        check(fs.pop(), 5, "pop()");
        check(fs.pop(), 4, "pop()");
        System.out.println("All tests passed.");
    }

    private static void check(int actual, int expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
