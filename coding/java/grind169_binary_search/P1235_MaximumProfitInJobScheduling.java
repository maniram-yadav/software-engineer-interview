/**
 * Grind 169 -- #1235. Maximum Profit in Job Scheduling (Hard)
 *
 * Given startTime, endTime, and profit arrays for jobs, find the maximum
 * profit achievable by scheduling non-overlapping jobs.
 *
 * Example:
 *   Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
 *   Output: 120
 */
public class P1235_MaximumProfitInJobScheduling {

    public int jobScheduling(int[] startTime, int[] endTime, int[] profit) {
        int n = startTime.length;
        Integer[] indices = new Integer[n];
        for (int i = 0; i < n; i++) indices[i] = i;
        java.util.Arrays.sort(indices, (a, b) -> endTime[a] - endTime[b]);

        int[] sortedEnd = new int[n];
        long[] dp = new long[n + 1];
        for (int i = 0; i < n; i++) sortedEnd[i] = endTime[indices[i]];

        for (int i = 1; i <= n; i++) {
            int idx = indices[i - 1];
            int j = upperBound(sortedEnd, i - 1, startTime[idx]);
            dp[i] = Math.max(dp[i - 1], dp[j] + profit[idx]);
        }
        return (int) dp[n];
    }

    private int upperBound(int[] sortedEnd, int len, int target) {
        int left = 0, right = len;
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (sortedEnd[mid] <= target) left = mid + 1;
            else right = mid;
        }
        return left;
    }

    public static void main(String[] args) {
        P1235_MaximumProfitInJobScheduling sol = new P1235_MaximumProfitInJobScheduling();
        test(sol, new int[]{1, 2, 3, 3}, new int[]{3, 4, 5, 6}, new int[]{50, 10, 40, 70}, 120);
        test(sol, new int[]{1, 2, 3, 4, 6}, new int[]{3, 5, 10, 6, 9}, new int[]{20, 20, 100, 70, 60}, 150);
        System.out.println("All tests passed.");
    }

    private static void test(P1235_MaximumProfitInJobScheduling sol, int[] startTime, int[] endTime, int[] profit, int expected) {
        int actual = sol.jobScheduling(startTime, endTime, profit);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
