/**
 * LeetCode Top Interview 150 -- #147. Edit Distance (Medium)
 *
 * Given two strings word1 and word2, return the minimum number of
 * operations (insert, delete, replace) to convert word1 into word2.
 *
 * Example:
 *   Input: word1 = "horse", word2 = "ros"
 *   Output: 3
 */
public class P147_EditDistance {

    public int minDistance(String word1, String word2) {
        int m = word1.length(), n = word2.length();
        int[][] dp = new int[m + 1][n + 1];
        for (int i = 0; i <= m; i++) dp[i][0] = i;
        for (int j = 0; j <= n; j++) dp[0][j] = j;

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (word1.charAt(i - 1) == word2.charAt(j - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + Math.min(dp[i - 1][j - 1], Math.min(dp[i - 1][j], dp[i][j - 1]));
                }
            }
        }
        return dp[m][n];
    }

    public static void main(String[] args) {
        P147_EditDistance sol = new P147_EditDistance();
        test(sol, "horse", "ros", 3);
        test(sol, "intention", "execution", 5);
        test(sol, "", "abc", 3);
        System.out.println("All tests passed.");
    }

    private static void test(P147_EditDistance sol, String word1, String word2, int expected) {
        int actual = sol.minDistance(word1, word2);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + word1 + "\" -> \"" + word2 + "\" = " + actual);
    }
}
