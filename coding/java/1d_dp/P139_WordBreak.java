/**
 * LeetCode Top Interview 150 -- #139. Word Break (Medium)
 *
 * Given a string s and a dictionary of words wordDict, return true if s
 * can be segmented into a space-separated sequence of one or more
 * dictionary words.
 *
 * Example:
 *   Input: s = "leetcode", wordDict = ["leet","code"]
 *   Output: true
 */
public class P139_WordBreak {

    public boolean wordBreak(String s, java.util.List<String> wordDict) {
        java.util.Set<String> dict = new java.util.HashSet<>(wordDict);
        boolean[] dp = new boolean[s.length() + 1];
        dp[0] = true;
        for (int i = 1; i <= s.length(); i++) {
            for (int j = 0; j < i; j++) {
                if (dp[j] && dict.contains(s.substring(j, i))) {
                    dp[i] = true;
                    break;
                }
            }
        }
        return dp[s.length()];
    }

    public static void main(String[] args) {
        P139_WordBreak sol = new P139_WordBreak();
        test(sol, "leetcode", java.util.List.of("leet", "code"), true);
        test(sol, "applepenapple", java.util.List.of("apple", "pen"), true);
        test(sol, "catsandog", java.util.List.of("cats", "dog", "sand", "and", "cat"), false);
        System.out.println("All tests passed.");
    }

    private static void test(P139_WordBreak sol, String s, java.util.List<String> wordDict, boolean expected) {
        boolean actual = sol.wordBreak(s, wordDict);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
