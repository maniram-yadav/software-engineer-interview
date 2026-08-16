/**
 * LeetCode Top Interview 150 -- #49. Merge Intervals (Medium)
 *
 * Given an array of intervals, merge all overlapping intervals and return
 * the non-overlapping intervals covering all input intervals.
 *
 * Example:
 *   Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
 *   Output: [[1,6],[8,10],[15,18]]
 */
public class P49_MergeIntervals {

    public int[][] merge(int[][] intervals) {
        java.util.Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
        java.util.List<int[]> merged = new java.util.ArrayList<>();
        for (int[] interval : intervals) {
            if (merged.isEmpty() || merged.get(merged.size() - 1)[1] < interval[0]) {
                merged.add(interval);
            } else {
                merged.get(merged.size() - 1)[1] = Math.max(merged.get(merged.size() - 1)[1], interval[1]);
            }
        }
        return merged.toArray(new int[0][]);
    }

    public static void main(String[] args) {
        P49_MergeIntervals sol = new P49_MergeIntervals();
        test(sol, new int[][]{{1, 3}, {2, 6}, {8, 10}, {15, 18}}, new int[][]{{1, 6}, {8, 10}, {15, 18}});
        test(sol, new int[][]{{1, 4}, {4, 5}}, new int[][]{{1, 5}});
        test(sol, new int[][]{{1, 4}, {0, 4}}, new int[][]{{0, 4}});
        System.out.println("All tests passed.");
    }

    private static void test(P49_MergeIntervals sol, int[][] intervals, int[][] expected) {
        int[][] actual = sol.merge(intervals);
        if (!java.util.Arrays.deepEquals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.deepToString(expected) + " but got " + java.util.Arrays.deepToString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(actual));
    }
}
