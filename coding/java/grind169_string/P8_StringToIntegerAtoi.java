/**
 * Grind 169 -- #8. String to Integer (atoi) (Medium)
 *
 * Implement atoi to convert a string to a 32-bit signed integer, following
 * specific whitespace/sign/overflow rules.
 *
 * Example:
 *   Input: s = "   -42"
 *   Output: -42
 */
public class P8_StringToIntegerAtoi {

    public int myAtoi(String s) {
        int i = 0, n = s.length();
        while (i < n && s.charAt(i) == ' ') i++;
        if (i == n) return 0;

        int sign = 1;
        if (s.charAt(i) == '+' || s.charAt(i) == '-') {
            sign = (s.charAt(i) == '-') ? -1 : 1;
            i++;
        }

        long result = 0;
        while (i < n && Character.isDigit(s.charAt(i))) {
            result = result * 10 + (s.charAt(i) - '0');
            if (result * sign > Integer.MAX_VALUE) return Integer.MAX_VALUE;
            if (result * sign < Integer.MIN_VALUE) return Integer.MIN_VALUE;
            i++;
        }
        return (int) (result * sign);
    }

    public static void main(String[] args) {
        P8_StringToIntegerAtoi sol = new P8_StringToIntegerAtoi();
        test(sol, "   -42", -42);
        test(sol, "4193 with words", 4193);
        test(sol, "words and 987", 0);
        System.out.println("All tests passed.");
    }

    private static void test(P8_StringToIntegerAtoi sol, String s, int expected) {
        int actual = sol.myAtoi(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
