/**
 * LeetCode Top Interview 150 -- #148. Best Time to Buy and Sell Stock III (Hard)
 *
 * Given stock prices, find the max profit with at most two transactions
 * (must sell before buying again).
 *
 * Example:
 *   Input: prices = [3,3,5,0,0,3,1,4]
 *   Output: 6   (buy@0 sell@3=3, buy@1 sell@4=3)
 */
public class P148_BestTimeToBuyAndSellStockIII {

    public int maxProfit(int[] prices) {
        int buy1 = Integer.MIN_VALUE, sell1 = 0, buy2 = Integer.MIN_VALUE, sell2 = 0;
        for (int p : prices) {
            buy1 = Math.max(buy1, -p);
            sell1 = Math.max(sell1, buy1 + p);
            buy2 = Math.max(buy2, sell1 - p);
            sell2 = Math.max(sell2, buy2 + p);
        }
        return sell2;
    }

    public static void main(String[] args) {
        P148_BestTimeToBuyAndSellStockIII sol = new P148_BestTimeToBuyAndSellStockIII();
        test(sol, new int[]{3, 3, 5, 0, 0, 3, 1, 4}, 6);
        test(sol, new int[]{1, 2, 3, 4, 5}, 4);
        test(sol, new int[]{7, 6, 4, 3, 1}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P148_BestTimeToBuyAndSellStockIII sol, int[] prices, int expected) {
        int actual = sol.maxProfit(prices);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(prices) + " -> " + actual);
    }
}
