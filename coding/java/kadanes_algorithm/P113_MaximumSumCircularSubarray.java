/**
 * LeetCode Top Interview 150 -- #113. Maximum Sum Circular Subarray (Medium)
 *
 * Given a circular integer array nums (end connects to start), find the
 * maximum possible sum of a non-empty contiguous subarray.
 *
 * Example:
 *   Input: nums = [5,-3,5]
 *   Output: 10   (wrap-around subarray [5,5])
 */
public class P113_MaximumSumCircularSubarray {

    public int maxSubarraySumCircular(int[] nums) {
        int total = 0;
        int maxSum = nums[0], curMax = 0;
        int minSum = nums[0], curMin = 0;

        for (int n : nums) {
            curMax = Math.max(curMax + n, n);
            maxSum = Math.max(maxSum, curMax);
            curMin = Math.min(curMin + n, n);
            minSum = Math.min(minSum, curMin);
            total += n;
        }

        if (maxSum < 0) return maxSum;
        return Math.max(maxSum, total - minSum);
    }

    public static void main(String[] args) {
        P113_MaximumSumCircularSubarray sol = new P113_MaximumSumCircularSubarray();
        test(sol, new int[]{1, -2, 3, -2}, 3);
        test(sol, new int[]{5, -3, 5}, 10);
        test(sol, new int[]{-3, -2, -3}, -2);
        System.out.println("All tests passed.");
    }

    private static void test(P113_MaximumSumCircularSubarray sol, int[] nums, int expected) {
        int actual = sol.maxSubarraySumCircular(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
