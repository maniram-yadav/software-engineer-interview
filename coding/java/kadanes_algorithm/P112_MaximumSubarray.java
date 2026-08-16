/**
 * LeetCode Top Interview 150 -- #112. Maximum Subarray (Medium)
 *
 * Given an integer array nums, find the contiguous subarray (at least one
 * element) with the largest sum, and return that sum.
 *
 * Example:
 *   Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 *   Output: 6   (subarray [4,-1,2,1])
 */
public class P112_MaximumSubarray {

    public int maxSubArray(int[] nums) {
        int maxSum = nums[0], curSum = nums[0];
        for (int i = 1; i < nums.length; i++) {
            curSum = Math.max(nums[i], curSum + nums[i]);
            maxSum = Math.max(maxSum, curSum);
        }
        return maxSum;
    }

    public static void main(String[] args) {
        P112_MaximumSubarray sol = new P112_MaximumSubarray();
        test(sol, new int[]{-2, 1, -3, 4, -1, 2, 1, -5, 4}, 6);
        test(sol, new int[]{1}, 1);
        test(sol, new int[]{5, 4, -1, 7, 8}, 23);
        System.out.println("All tests passed.");
    }

    private static void test(P112_MaximumSubarray sol, int[] nums, int expected) {
        int actual = sol.maxSubArray(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
