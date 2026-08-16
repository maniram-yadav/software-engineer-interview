/**
 * LeetCode Top Interview 150 -- #145. Longest Palindromic Substring (Medium)
 *
 * Given a string s, return the longest palindromic substring.
 *
 * Example:
 *   Input: s = "babad"
 *   Output: "bab"  (or "aba")
 */
public class P145_LongestPalindromicSubstring {

    public String longestPalindrome(String s) {
        if (s.length() < 2) return s;
        int start = 0, maxLen = 1;
        for (int i = 0; i < s.length(); i++) {
            int len1 = expand(s, i, i);
            int len2 = expand(s, i, i + 1);
            int len = Math.max(len1, len2);
            if (len > maxLen) {
                maxLen = len;
                start = i - (len - 1) / 2;
            }
        }
        return s.substring(start, start + maxLen);
    }

    private int expand(String s, int left, int right) {
        while (left >= 0 && right < s.length() && s.charAt(left) == s.charAt(right)) {
            left--;
            right++;
        }
        return right - left - 1;
    }

    public static void main(String[] args) {
        P145_LongestPalindromicSubstring sol = new P145_LongestPalindromicSubstring();
        test(sol, "babad", 3);
        test(sol, "cbbd", 2);
        test(sol, "a", 1);
        System.out.println("All tests passed.");
    }

    private static void test(P145_LongestPalindromicSubstring sol, String s, int expectedLen) {
        String actual = sol.longestPalindrome(s);
        if (actual.length() != expectedLen) {
            throw new AssertionError("Expected length " + expectedLen + " but got \"" + actual + "\" (length " + actual.length() + ")");
        }
        if (!s.contains(actual)) {
            throw new AssertionError("\"" + actual + "\" is not a substring of \"" + s + "\"");
        }
        if (!isPalindrome(actual)) {
            throw new AssertionError("\"" + actual + "\" is not a palindrome");
        }
        System.out.println("PASS: \"" + s + "\" -> \"" + actual + "\"");
    }

    private static boolean isPalindrome(String s) {
        int left = 0, right = s.length() - 1;
        while (left < right) {
            if (s.charAt(left++) != s.charAt(right--)) return false;
        }
        return true;
    }
}
