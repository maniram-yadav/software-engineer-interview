/**
 * LeetCode Top Interview 150 -- #30. Minimum Size Subarray Sum (Medium)
 *
 * Given a positive integer array nums and a target target, find the
 * minimal length of a contiguous subarray whose sum is >= target. Return 0
 * if none exists.
 *
 * Example:
 *   Input: target = 7, nums = [2,3,1,2,4,3]
 *   Output: 2   (subarray [4,3])
 */
public class P30_MinimumSizeSubarraySum {

    public int minSubArrayLen(int target, int[] nums) {
        int left = 0, sum = 0, minLen = Integer.MAX_VALUE;
        for (int right = 0; right < nums.length; right++) {
            sum += nums[right];
            while (sum >= target) {
                minLen = Math.min(minLen, right - left + 1);
                sum -= nums[left++];
            }
        }
        return minLen == Integer.MAX_VALUE ? 0 : minLen;
    }

    public static void main(String[] args) {
        P30_MinimumSizeSubarraySum sol = new P30_MinimumSizeSubarraySum();
        test(sol, 7, new int[]{2, 3, 1, 2, 4, 3}, 2);
        test(sol, 4, new int[]{1, 4, 4}, 1);
        test(sol, 11, new int[]{1, 1, 1, 1, 1, 1, 1, 1}, 0);
        test(sol, 15, new int[]{1, 2, 3, 4, 5}, 5);
        System.out.println("All tests passed.");
    }

    private static void test(P30_MinimumSizeSubarraySum sol, int target, int[] nums, int expected) {
        int actual = sol.minSubArrayLen(target, nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: target=" + target + " " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
