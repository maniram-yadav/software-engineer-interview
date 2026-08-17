/**
 * Grind 169 -- #91. Decode Ways (Medium)
 *
 * A message of digits can be decoded via 'A'->1, ..., 'Z'->26. Given a
 * digit string s, return the number of ways to decode it.
 *
 * Example:
 *   Input: s = "12"
 *   Output: 2   ("AB" or "L")
 */
public class P91_DecodeWays {

    public int numDecodings(String s) {
        if (s.isEmpty() || s.charAt(0) == '0') return 0;
        int n = s.length();
        int prev2 = 1, prev1 = 1;
        for (int i = 1; i < n; i++) {
            int cur = 0;
            if (s.charAt(i) != '0') cur += prev1;
            int twoDigit = Integer.parseInt(s.substring(i - 1, i + 1));
            if (twoDigit >= 10 && twoDigit <= 26) cur += prev2;
            prev2 = prev1;
            prev1 = cur;
        }
        return prev1;
    }

    public static void main(String[] args) {
        P91_DecodeWays sol = new P91_DecodeWays();
        test(sol, "12", 2);
        test(sol, "226", 3);
        test(sol, "06", 0);
        System.out.println("All tests passed.");
    }

    private static void test(P91_DecodeWays sol, String s, int expected) {
        int actual = sol.numDecodings(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
