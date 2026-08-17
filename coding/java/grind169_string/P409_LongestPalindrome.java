/**
 * Grind 169 -- #409. Longest Palindrome (Easy)
 *
 * Given a string of lowercase/uppercase letters, return the length of the
 * longest palindrome that can be built from those letters (case-sensitive,
 * rearrangement allowed).
 *
 * Example:
 *   Input: s = "abccccdd"
 *   Output: 7   (e.g. "dccaccd")
 */
public class P409_LongestPalindrome {

    public int longestPalindrome(String s) {
        int[] counts = new int[128];
        for (char c : s.toCharArray()) counts[c]++;
        int length = 0;
        boolean hasOdd = false;
        for (int count : counts) {
            length += (count / 2) * 2;
            if (count % 2 == 1) hasOdd = true;
        }
        return hasOdd ? length + 1 : length;
    }

    public static void main(String[] args) {
        P409_LongestPalindrome sol = new P409_LongestPalindrome();
        test(sol, "abccccdd", 7);
        test(sol, "a", 1);
        test(sol, "bb", 2);
        System.out.println("All tests passed.");
    }

    private static void test(P409_LongestPalindrome sol, String s, int expected) {
        int actual = sol.longestPalindrome(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
