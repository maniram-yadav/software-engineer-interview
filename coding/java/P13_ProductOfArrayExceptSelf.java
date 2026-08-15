/**
 * LeetCode Top Interview 150 -- #13. Product of Array Except Self (Medium)
 *
 * Given an array nums, return an array answer where answer[i] is the
 * product of all elements except nums[i], without using division, in O(n).
 *
 * Example:
 *   Input: nums = [1,2,3,4]
 *   Output: [24,12,8,6]
 */
public class P13_ProductOfArrayExceptSelf {

    public int[] productExceptSelf(int[] nums) {
        int n = nums.length;
        int[] answer = new int[n];
        answer[0] = 1;
        for (int i = 1; i < n; i++) {
            answer[i] = answer[i - 1] * nums[i - 1];
        }
        int right = 1;
        for (int i = n - 1; i >= 0; i--) {
            answer[i] *= right;
            right *= nums[i];
        }
        return answer;
    }

    public static void main(String[] args) {
        P13_ProductOfArrayExceptSelf sol = new P13_ProductOfArrayExceptSelf();
        test(sol, new int[]{1, 2, 3, 4}, new int[]{24, 12, 8, 6});
        test(sol, new int[]{-1, 1, 0, -3, 3}, new int[]{0, 0, 9, 0, 0});
        test(sol, new int[]{2, 3}, new int[]{3, 2});
        test(sol, new int[]{4}, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P13_ProductOfArrayExceptSelf sol, int[] nums, int[] expected) {
        int[] actual = sol.productExceptSelf(nums);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + java.util.Arrays.toString(actual));
    }
}
