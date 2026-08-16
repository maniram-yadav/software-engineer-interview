/**
 * LeetCode Top Interview 150 -- #124. Find Median from Data Stream (Hard)
 *
 * Design a data structure that supports adding integers from a stream and
 * finding the median of all elements so far efficiently (typically via two
 * heaps).
 *
 * Example:
 *   addNum(1); addNum(2);
 *   findMedian(); // 1.5
 *   addNum(3);
 *   findMedian(); // 2.0
 */
public class P124_FindMedianFromDataStream {

    static class MedianFinder {
        private final java.util.PriorityQueue<Integer> low = new java.util.PriorityQueue<>(java.util.Collections.reverseOrder());
        private final java.util.PriorityQueue<Integer> high = new java.util.PriorityQueue<>();

        public void addNum(int num) {
            low.add(num);
            high.add(low.poll());
            if (high.size() > low.size()) {
                low.add(high.poll());
            }
        }

        public double findMedian() {
            if (low.size() > high.size()) return low.peek();
            return (low.peek() + high.peek()) / 2.0;
        }
    }

    public static void main(String[] args) {
        MedianFinder mf = new MedianFinder();
        mf.addNum(1);
        mf.addNum(2);
        check(mf.findMedian(), 1.5, "findMedian after [1,2]");
        mf.addNum(3);
        check(mf.findMedian(), 2.0, "findMedian after [1,2,3]");
        mf.addNum(4);
        check(mf.findMedian(), 2.5, "findMedian after [1,2,3,4]");
        System.out.println("All tests passed.");
    }

    private static void check(double actual, double expected, String label) {
        if (Math.abs(actual - expected) > 1e-9) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
