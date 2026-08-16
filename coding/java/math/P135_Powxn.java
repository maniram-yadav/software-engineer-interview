/**
 * LeetCode Top Interview 150 -- #135. Pow(x, n) (Medium)
 *
 * Implement pow(x, n), computing x raised to the power n, in O(log n) time
 * (fast exponentiation).
 *
 * Example:
 *   Input: x = 2.00000, n = 10
 *   Output: 1024.00000
 */
public class P135_Powxn {

    public double myPow(double x, int n) {
        long N = n;
        if (N < 0) {
            x = 1 / x;
            N = -N;
        }
        double result = 1;
        while (N > 0) {
            if ((N & 1) == 1) result *= x;
            x *= x;
            N >>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        P135_Powxn sol = new P135_Powxn();
        test(sol, 2.0, 10, 1024.0);
        test(sol, 2.1, 3, 9.261);
        test(sol, 2.0, -2, 0.25);
        System.out.println("All tests passed.");
    }

    private static void test(P135_Powxn sol, double x, int n, double expected) {
        double actual = sol.myPow(x, n);
        if (Math.abs(actual - expected) > 1e-6) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + x + "^" + n + " -> " + actual);
    }
}
