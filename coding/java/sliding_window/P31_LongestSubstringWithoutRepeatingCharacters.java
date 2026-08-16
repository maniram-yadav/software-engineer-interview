/**
 * LeetCode Top Interview 150 -- #31. Longest Substring Without Repeating Characters (Medium)
 *
 * Given a string s, find the length of the longest substring without
 * repeating characters.
 *
 * Example:
 *   Input: s = "abcabcbb"
 *   Output: 3   ("abc")
 */
public class P31_LongestSubstringWithoutRepeatingCharacters {

    public int lengthOfLongestSubstring(String s) {
        java.util.Map<Character, Integer> lastIndex = new java.util.HashMap<>();
        int left = 0, maxLen = 0;
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            if (lastIndex.containsKey(c) && lastIndex.get(c) >= left) {
                left = lastIndex.get(c) + 1;
            }
            lastIndex.put(c, right);
            maxLen = Math.max(maxLen, right - left + 1);
        }
        return maxLen;
    }

    public static void main(String[] args) {
        P31_LongestSubstringWithoutRepeatingCharacters sol = new P31_LongestSubstringWithoutRepeatingCharacters();
        test(sol, "abcabcbb", 3);
        test(sol, "bbbbb", 1);
        test(sol, "pwwkew", 3);
        test(sol, "", 0);
        System.out.println("All tests passed.");
    }

    private static void test(P31_LongestSubstringWithoutRepeatingCharacters sol, String s, int expected) {
        int actual = sol.lengthOfLongestSubstring(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
