/**
 * LeetCode Top Interview 150 -- #50. Insert Interval (Medium)
 *
 * Given a sorted, non-overlapping list of intervals and a new interval,
 * insert it and merge as necessary, returning the sorted, non-overlapping
 * result.
 *
 * Example:
 *   Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
 *   Output: [[1,5],[6,9]]
 */
public class P50_InsertInterval {

    public int[][] insert(int[][] intervals, int[] newInterval) {
        java.util.List<int[]> result = new java.util.ArrayList<>();
        int i = 0, n = intervals.length;

        while (i < n && intervals[i][1] < newInterval[0]) {
            result.add(intervals[i]);
            i++;
        }

        int start = newInterval[0], end = newInterval[1];
        while (i < n && intervals[i][0] <= end) {
            start = Math.min(start, intervals[i][0]);
            end = Math.max(end, intervals[i][1]);
            i++;
        }
        result.add(new int[]{start, end});

        while (i < n) {
            result.add(intervals[i]);
            i++;
        }

        return result.toArray(new int[0][]);
    }

    public static void main(String[] args) {
        P50_InsertInterval sol = new P50_InsertInterval();
        test(sol, new int[][]{{1, 3}, {6, 9}}, new int[]{2, 5}, new int[][]{{1, 5}, {6, 9}});
        test(sol, new int[][]{{1, 2}, {3, 5}, {6, 7}, {8, 10}, {12, 16}}, new int[]{4, 8}, new int[][]{{1, 2}, {3, 10}, {12, 16}});
        test(sol, new int[][]{}, new int[]{5, 7}, new int[][]{{5, 7}});
        System.out.println("All tests passed.");
    }

    private static void test(P50_InsertInterval sol, int[][] intervals, int[] newInterval, int[][] expected) {
        int[][] actual = sol.insert(intervals, newInterval);
        if (!java.util.Arrays.deepEquals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.deepToString(expected) + " but got " + java.util.Arrays.deepToString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(actual));
    }
}
