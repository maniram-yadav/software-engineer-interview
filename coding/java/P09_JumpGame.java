/**
 * LeetCode Top Interview 150 -- #9. Jump Game (Medium)
 *
 * Given an array nums where nums[i] is the max jump length from index i,
 * starting at index 0, return true if you can reach the last index.
 *
 * Example:
 *   Input: nums = [2,3,1,1,4]
 *   Output: true
 */
public class P09_JumpGame {

    public boolean canJump(int[] nums) {
        int reach = 0;
        for (int i = 0; i < nums.length; i++) {
            if (i > reach) return false;
            reach = Math.max(reach, i + nums[i]);
        }
        return true;
    }

    public static void main(String[] args) {
        P09_JumpGame sol = new P09_JumpGame();
        test(sol, new int[]{2, 3, 1, 1, 4}, true);
        test(sol, new int[]{3, 2, 1, 0, 4}, false);
        test(sol, new int[]{0}, true);
        test(sol, new int[]{1, 0, 1, 0}, false);
        System.out.println("All tests passed.");
    }

    private static void test(P09_JumpGame sol, int[] nums, boolean expected) {
        boolean actual = sol.canJump(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
