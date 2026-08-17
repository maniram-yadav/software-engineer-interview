/**
 * Grind 169 -- #435. Non-overlapping Intervals (Medium)
 *
 * Given an array of intervals, return the minimum number of intervals to
 * remove so the rest are non-overlapping.
 *
 * Example:
 *   Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
 *   Output: 1
 */
public class P435_NonOverlappingIntervals {

    public int eraseOverlapIntervals(int[][] intervals) {
        if (intervals.length == 0) return 0;
        java.util.Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));

        int count = 0;
        int end = intervals[0][1];
        for (int i = 1; i < intervals.length; i++) {
            if (intervals[i][0] < end) {
                count++;
            } else {
                end = intervals[i][1];
            }
        }
        return count;
    }

    public static void main(String[] args) {
        P435_NonOverlappingIntervals sol = new P435_NonOverlappingIntervals();
        test(sol, new int[][]{{1, 2}, {2, 3}, {3, 4}, {1, 3}}, 1);
        test(sol, new int[][]{{1, 2}, {1, 2}, {1, 2}}, 2);
        test(sol, new int[][]{{1, 2}, {2, 3}}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P435_NonOverlappingIntervals sol, int[][] intervals, int expected) {
        int actual = sol.eraseOverlapIntervals(intervals);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
