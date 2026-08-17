/**
 * Grind 169 -- #377. Combination Sum IV (Medium)
 *
 * Given an array of distinct positive integers and a target, return the
 * number of possible combinations (order matters, elements reusable) that
 * add up to target.
 *
 * Example:
 *   Input: nums = [1,2,3], target = 4
 *   Output: 7
 */
public class P377_CombinationSumIV {

    public int combinationSum4(int[] nums, int target) {
        int[] dp = new int[target + 1];
        dp[0] = 1;
        for (int i = 1; i <= target; i++) {
            for (int n : nums) {
                if (n <= i) dp[i] += dp[i - n];
            }
        }
        return dp[target];
    }

    public static void main(String[] args) {
        P377_CombinationSumIV sol = new P377_CombinationSumIV();
        test(sol, new int[]{1, 2, 3}, 4, 7);
        test(sol, new int[]{9}, 3, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P377_CombinationSumIV sol, int[] nums, int target, int expected) {
        int actual = sol.combinationSum4(nums, target);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " target=" + target + " -> " + actual);
    }
}
