/**
 * Grind 169 -- #7. Reverse Integer (Medium)
 *
 * Given a 32-bit signed integer x, return x with its digits reversed;
 * return 0 if the reversed value overflows a 32-bit signed integer.
 *
 * Example:
 *   Input: x = 123
 *   Output: 321
 */
public class P7_ReverseInteger {

    public int reverse(int x) {
        long result = 0;
        while (x != 0) {
            result = result * 10 + x % 10;
            x /= 10;
            if (result > Integer.MAX_VALUE || result < Integer.MIN_VALUE) return 0;
        }
        return (int) result;
    }

    public static void main(String[] args) {
        P7_ReverseInteger sol = new P7_ReverseInteger();
        test(sol, 123, 321);
        test(sol, -123, -321);
        test(sol, 120, 21);
        test(sol, 1534236469, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P7_ReverseInteger sol, int x, int expected) {
        int actual = sol.reverse(x);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + x + " -> " + actual);
    }
}
