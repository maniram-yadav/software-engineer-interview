/**
 * LeetCode Top Interview 150 -- #8. Best Time to Buy and Sell Stock II (Medium)
 *
 * Same setup as Best Time to Buy and Sell Stock, but you may complete as
 * many transactions as you like (buy then sell, one share at a time; you
 * must sell before buying again). Maximize total profit.
 *
 * Example:
 *   Input: prices = [7,1,5,3,6,4]
 *   Output: 7   (buy@1 sell@5 = 4, buy@3 sell@6 = 3)
 */
public class P08_BestTimeToBuyAndSellStockII {

    public int maxProfit(int[] prices) {
        int profit = 0;
        for (int i = 1; i < prices.length; i++) {
            if (prices[i] > prices[i - 1]) {
                profit += prices[i] - prices[i - 1];
            }
        }
        return profit;
    }

    public static void main(String[] args) {
        P08_BestTimeToBuyAndSellStockII sol = new P08_BestTimeToBuyAndSellStockII();
        test(sol, new int[]{7, 1, 5, 3, 6, 4}, 7);
        test(sol, new int[]{1, 2, 3, 4, 5}, 4);
        test(sol, new int[]{7, 6, 4, 3, 1}, 0);
        test(sol, new int[]{1}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P08_BestTimeToBuyAndSellStockII sol, int[] prices, int expected) {
        int actual = sol.maxProfit(prices);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(prices) + " -> " + actual);
    }
}
