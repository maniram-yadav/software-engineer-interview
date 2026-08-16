/**
 * LeetCode Top Interview 150 -- #7. Best Time to Buy and Sell Stock (Easy)
 *
 * Given an array prices where prices[i] is the stock price on day i, find
 * the max profit from a single buy followed by a single sell (buy must
 * happen before sell). Return 0 if no profit is possible.
 *
 * Example:
 *   Input: prices = [7,1,5,3,6,4]
 *   Output: 5   (buy at 1, sell at 6)
 */
public class P07_BestTimeToBuyAndSellStock {

    public int maxProfit(int[] prices) {
        int minPrice = Integer.MAX_VALUE;
        int maxProfit = 0;
        for (int price : prices) {
            if (price < minPrice) {
                minPrice = price;
            } else {
                maxProfit = Math.max(maxProfit, price - minPrice);
            }
        }
        return maxProfit;
    }

    public static void main(String[] args) {
        P07_BestTimeToBuyAndSellStock sol = new P07_BestTimeToBuyAndSellStock();
        test(sol, new int[]{7, 1, 5, 3, 6, 4}, 5);
        test(sol, new int[]{7, 6, 4, 3, 1}, 0);
        test(sol, new int[]{1, 2}, 1);
        test(sol, new int[]{2}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P07_BestTimeToBuyAndSellStock sol, int[] prices, int expected) {
        int actual = sol.maxProfit(prices);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(prices) + " -> " + actual);
    }
}
