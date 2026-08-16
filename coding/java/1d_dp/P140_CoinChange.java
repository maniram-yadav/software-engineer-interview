/**
 * LeetCode Top Interview 150 -- #140. Coin Change (Medium)
 *
 * Given coin denominations and an amount, return the fewest number of
 * coins needed to make up that amount, or -1 if impossible.
 *
 * Example:
 *   Input: coins = [1,2,5], amount = 11
 *   Output: 3   (5+5+1)
 */
public class P140_CoinChange {

    public int coinChange(int[] coins, int amount) {
        int[] dp = new int[amount + 1];
        java.util.Arrays.fill(dp, amount + 1);
        dp[0] = 0;
        for (int i = 1; i <= amount; i++) {
            for (int coin : coins) {
                if (coin <= i) {
                    dp[i] = Math.min(dp[i], dp[i - coin] + 1);
                }
            }
        }
        return dp[amount] > amount ? -1 : dp[amount];
    }

    public static void main(String[] args) {
        P140_CoinChange sol = new P140_CoinChange();
        test(sol, new int[]{1, 2, 5}, 11, 3);
        test(sol, new int[]{2}, 3, -1);
        test(sol, new int[]{1}, 0, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P140_CoinChange sol, int[] coins, int amount, int expected) {
        int actual = sol.coinChange(coins, amount);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(coins) + " amount=" + amount + " -> " + actual);
    }
}
