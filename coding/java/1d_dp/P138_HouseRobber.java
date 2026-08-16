/**
 * LeetCode Top Interview 150 -- #138. House Robber (Medium)
 *
 * Given an array of non-negative integers representing money in houses
 * along a street, find the max amount you can rob without robbing two
 * adjacent houses.
 *
 * Example:
 *   Input: nums = [1,2,3,1]
 *   Output: 4   (rob house 1 and house 3: 1+3)
 */
public class P138_HouseRobber {

    public int rob(int[] nums) {
        int prev2 = 0, prev1 = 0;
        for (int n : nums) {
            int cur = Math.max(prev1, prev2 + n);
            prev2 = prev1;
            prev1 = cur;
        }
        return prev1;
    }

    public static void main(String[] args) {
        P138_HouseRobber sol = new P138_HouseRobber();
        test(sol, new int[]{1, 2, 3, 1}, 4);
        test(sol, new int[]{2, 7, 9, 3, 1}, 12);
        test(sol, new int[]{}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P138_HouseRobber sol, int[] nums, int expected) {
        int actual = sol.rob(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
