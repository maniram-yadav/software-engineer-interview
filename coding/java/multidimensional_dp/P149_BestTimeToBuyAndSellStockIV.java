/**
 * LeetCode Top Interview 150 -- #149. Best Time to Buy and Sell Stock IV (Hard)
 *
 * Same as Best Time to Buy and Sell Stock III, generalized to at most k
 * transactions.
 *
 * Example:
 *   Input: k = 2, prices = [2,4,1]
 *   Output: 2
 */
public class P149_BestTimeToBuyAndSellStockIV {

    public int maxProfit(int k, int[] prices) {
        if (prices.length == 0 || k == 0) return 0;

        int[] buy = new int[k + 1];
        int[] sell = new int[k + 1];
        java.util.Arrays.fill(buy, Integer.MIN_VALUE);

        for (int p : prices) {
            for (int i = 1; i <= k; i++) {
                buy[i] = Math.max(buy[i], sell[i - 1] - p);
                sell[i] = Math.max(sell[i], buy[i] + p);
            }
        }
        return sell[k];
    }

    public static void main(String[] args) {
        P149_BestTimeToBuyAndSellStockIV sol = new P149_BestTimeToBuyAndSellStockIV();
        test(sol, 2, new int[]{2, 4, 1}, 2);
        test(sol, 2, new int[]{3, 2, 6, 5, 0, 3}, 7);
        test(sol, 0, new int[]{1, 2, 3}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P149_BestTimeToBuyAndSellStockIV sol, int k, int[] prices, int expected) {
        int actual = sol.maxProfit(k, prices);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: k=" + k + " " + java.util.Arrays.toString(prices) + " -> " + actual);
    }
}
