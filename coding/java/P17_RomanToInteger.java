/**
 * LeetCode Top Interview 150 -- #17. Roman to Integer (Easy)
 *
 * Convert a Roman numeral string to an integer.
 *
 * Example:
 *   Input: s = "MCMXCIV"
 *   Output: 1994
 */
public class P17_RomanToInteger {

    public int romanToInt(String s) {
        java.util.Map<Character, Integer> values = new java.util.HashMap<>();
        values.put('I', 1);
        values.put('V', 5);
        values.put('X', 10);
        values.put('L', 50);
        values.put('C', 100);
        values.put('D', 500);
        values.put('M', 1000);

        int total = 0;
        for (int i = 0; i < s.length(); i++) {
            int cur = values.get(s.charAt(i));
            if (i + 1 < s.length() && cur < values.get(s.charAt(i + 1))) {
                total -= cur;
            } else {
                total += cur;
            }
        }
        return total;
    }

    public static void main(String[] args) {
        P17_RomanToInteger sol = new P17_RomanToInteger();
        test(sol, "MCMXCIV", 1994);
        test(sol, "III", 3);
        test(sol, "LVIII", 58);
        test(sol, "IX", 9);
        System.out.println("All tests passed.");
    }

    private static void test(P17_RomanToInteger sol, String s, int expected) {
        int actual = sol.romanToInt(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + s + " -> " + actual);
    }
}
