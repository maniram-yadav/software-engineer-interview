/**
 * LeetCode Top Interview 150 -- #133. Factorial Trailing Zeroes (Medium)
 *
 * Given an integer n, return the number of trailing zeroes in n!, in
 * logarithmic time.
 *
 * Example:
 *   Input: n = 5
 *   Output: 1   (5! = 120)
 */
public class P133_FactorialTrailingZeroes {

    public int trailingZeroes(int n) {
        int count = 0;
        for (long p = 5; p <= n; p *= 5) {
            count += n / p;
        }
        return count;
    }

    public static void main(String[] args) {
        P133_FactorialTrailingZeroes sol = new P133_FactorialTrailingZeroes();
        test(sol, 5, 1);
        test(sol, 0, 0);
        test(sol, 25, 6);
        System.out.println("All tests passed.");
    }

    private static void test(P133_FactorialTrailingZeroes sol, int n, int expected) {
        int actual = sol.trailingZeroes(n);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + n + "! -> " + actual + " trailing zeroes");
    }
}
