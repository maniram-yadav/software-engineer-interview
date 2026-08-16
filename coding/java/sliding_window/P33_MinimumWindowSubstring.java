/**
 * LeetCode Top Interview 150 -- #33. Minimum Window Substring (Hard)
 *
 * Given strings s and t, return the smallest substring of s that contains
 * every character of t (with multiplicity). Return "" if no such substring
 * exists.
 *
 * Example:
 *   Input: s = "ADOBECODEBANC", t = "ABC"
 *   Output: "BANC"
 */
public class P33_MinimumWindowSubstring {

    public String minWindow(String s, String t) {
        if (s.isEmpty() || t.isEmpty()) return "";

        java.util.Map<Character, Integer> need = new java.util.HashMap<>();
        for (char c : t.toCharArray()) need.merge(c, 1, Integer::sum);

        java.util.Map<Character, Integer> window = new java.util.HashMap<>();
        int required = need.size();
        int formed = 0;
        int left = 0;
        int bestLen = -1, bestLeft = 0, bestRight = 0;

        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            window.merge(c, 1, Integer::sum);
            if (need.containsKey(c) && window.get(c).intValue() == need.get(c).intValue()) {
                formed++;
            }

            while (formed == required) {
                if (bestLen == -1 || right - left + 1 < bestLen) {
                    bestLen = right - left + 1;
                    bestLeft = left;
                    bestRight = right;
                }
                char leftChar = s.charAt(left);
                window.put(leftChar, window.get(leftChar) - 1);
                if (need.containsKey(leftChar) && window.get(leftChar) < need.get(leftChar)) {
                    formed--;
                }
                left++;
            }
        }

        return bestLen == -1 ? "" : s.substring(bestLeft, bestRight + 1);
    }

    public static void main(String[] args) {
        P33_MinimumWindowSubstring sol = new P33_MinimumWindowSubstring();
        test(sol, "ADOBECODEBANC", "ABC", "BANC");
        test(sol, "a", "a", "a");
        test(sol, "a", "aa", "");
        test(sol, "ab", "b", "b");
        System.out.println("All tests passed.");
    }

    private static void test(P33_MinimumWindowSubstring sol, String s, String t, String expected) {
        String actual = sol.minWindow(s, t);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: s=\"" + s + "\" t=\"" + t + "\" -> \"" + actual + "\"");
    }
}
