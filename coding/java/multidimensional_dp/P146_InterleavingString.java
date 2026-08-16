/**
 * LeetCode Top Interview 150 -- #146. Interleaving String (Medium)
 *
 * Given strings s1, s2, and s3, determine if s3 is formed by an
 * interleaving of s1 and s2 (preserving each string's relative character
 * order).
 *
 * Example:
 *   Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 *   Output: true
 */
public class P146_InterleavingString {

    public boolean isInterleave(String s1, String s2, String s3) {
        int m = s1.length(), n = s2.length();
        if (m + n != s3.length()) return false;

        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[0][0] = true;
        for (int i = 1; i <= m; i++) dp[i][0] = dp[i - 1][0] && s1.charAt(i - 1) == s3.charAt(i - 1);
        for (int j = 1; j <= n; j++) dp[0][j] = dp[0][j - 1] && s2.charAt(j - 1) == s3.charAt(j - 1);

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                dp[i][j] = (dp[i - 1][j] && s1.charAt(i - 1) == s3.charAt(i + j - 1))
                        || (dp[i][j - 1] && s2.charAt(j - 1) == s3.charAt(i + j - 1));
            }
        }
        return dp[m][n];
    }

    public static void main(String[] args) {
        P146_InterleavingString sol = new P146_InterleavingString();
        test(sol, "aabcc", "dbbca", "aadbbcbcac", true);
        test(sol, "aabcc", "dbbca", "aadbbbaccc", false);
        test(sol, "", "", "", true);
        System.out.println("All tests passed.");
    }

    private static void test(P146_InterleavingString sol, String s1, String s2, String s3, boolean expected) {
        boolean actual = sol.isInterleave(s1, s2, s3);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
