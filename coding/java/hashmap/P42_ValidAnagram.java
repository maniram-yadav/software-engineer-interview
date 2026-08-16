/**
 * LeetCode Top Interview 150 -- #42. Valid Anagram (Easy)
 *
 * Given strings s and t, return true if t is an anagram of s.
 *
 * Example:
 *   Input: s = "anagram", t = "nagaram"
 *   Output: true
 */
public class P42_ValidAnagram {

    public boolean isAnagram(String s, String t) {
        if (s.length() != t.length()) return false;
        int[] counts = new int[26];
        for (char c : s.toCharArray()) counts[c - 'a']++;
        for (char c : t.toCharArray()) {
            if (--counts[c - 'a'] < 0) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        P42_ValidAnagram sol = new P42_ValidAnagram();
        test(sol, "anagram", "nagaram", true);
        test(sol, "rat", "car", false);
        test(sol, "a", "ab", false);
        test(sol, "", "", true);
        System.out.println("All tests passed.");
    }

    private static void test(P42_ValidAnagram sol, String s, String t, boolean expected) {
        boolean actual = sol.isAnagram(s, t);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: s=\"" + s + "\" t=\"" + t + "\" -> " + actual);
    }
}
