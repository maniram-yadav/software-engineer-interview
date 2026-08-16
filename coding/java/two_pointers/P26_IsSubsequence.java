/**
 * LeetCode Top Interview 150 -- #26. Is Subsequence (Easy)
 *
 * Given strings s and t, determine if s is a subsequence of t (characters
 * of s appear in t in order, not necessarily contiguous).
 *
 * Example:
 *   Input: s = "abc", t = "ahbgdc"
 *   Output: true
 */
public class P26_IsSubsequence {

    public boolean isSubsequence(String s, String t) {
        int i = 0, j = 0;
        while (i < s.length() && j < t.length()) {
            if (s.charAt(i) == t.charAt(j)) i++;
            j++;
        }
        return i == s.length();
    }

    public static void main(String[] args) {
        P26_IsSubsequence sol = new P26_IsSubsequence();
        test(sol, "abc", "ahbgdc", true);
        test(sol, "axc", "ahbgdc", false);
        test(sol, "", "abc", true);
        test(sol, "abc", "abc", true);
        System.out.println("All tests passed.");
    }

    private static void test(P26_IsSubsequence sol, String s, String t, boolean expected) {
        boolean actual = sol.isSubsequence(s, t);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: s=\"" + s + "\" t=\"" + t + "\" -> " + actual);
    }
}
