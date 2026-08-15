/**
 * LeetCode Top Interview 150 -- #18. Integer to Roman (Medium)
 *
 * Convert an integer (1 to 3999) to a Roman numeral string.
 *
 * Example:
 *   Input: num = 1994
 *   Output: "MCMXCIV"
 */
public class P18_IntegerToRoman {

    private static final int[] VALUES = {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
    private static final String[] SYMBOLS = {"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"};

    public String intToRoman(int num) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < VALUES.length; i++) {
            while (num >= VALUES[i]) {
                num -= VALUES[i];
                sb.append(SYMBOLS[i]);
            }
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        P18_IntegerToRoman sol = new P18_IntegerToRoman();
        test(sol, 1994, "MCMXCIV");
        test(sol, 3, "III");
        test(sol, 58, "LVIII");
        test(sol, 9, "IX");
        System.out.println("All tests passed.");
    }

    private static void test(P18_IntegerToRoman sol, int num, String expected) {
        String actual = sol.intToRoman(num);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + num + " -> " + actual);
    }
}
