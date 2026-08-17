/**
 * Grind 169 -- #416. Partition Equal Subset Sum (Medium)
 *
 * Given a non-empty array of positive integers, determine if it can be
 * partitioned into two subsets with equal sum.
 *
 * Example:
 *   Input: nums = [1,5,11,5]
 *   Output: true   ([1,5,5] and [11])
 */
public class P416_PartitionEqualSubsetSum {

    public boolean canPartition(int[] nums) {
        int sum = 0;
        for (int n : nums) sum += n;
        if (sum % 2 != 0) return false;

        int target = sum / 2;
        boolean[] dp = new boolean[target + 1];
        dp[0] = true;
        for (int n : nums) {
            for (int j = target; j >= n; j--) {
                dp[j] = dp[j] || dp[j - n];
            }
        }
        return dp[target];
    }

    public static void main(String[] args) {
        P416_PartitionEqualSubsetSum sol = new P416_PartitionEqualSubsetSum();
        test(sol, new int[]{1, 5, 11, 5}, true);
        test(sol, new int[]{1, 2, 3, 5}, false);
        test(sol, new int[]{1, 1}, true);
        System.out.println("All tests passed.");
    }

    private static void test(P416_PartitionEqualSubsetSum sol, int[] nums, boolean expected) {
        boolean actual = sol.canPartition(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
