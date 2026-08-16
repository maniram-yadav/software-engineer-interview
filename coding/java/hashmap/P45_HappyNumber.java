/**
 * LeetCode Top Interview 150 -- #45. Happy Number (Easy)
 *
 * A happy number is defined by repeatedly replacing it with the sum of the
 * squares of its digits, eventually reaching 1 (unhappy numbers loop
 * forever). Determine if n is happy.
 *
 * Example:
 *   Input: n = 19
 *   Output: true   (1^2+9^2=82 -> 8^2+2^2=68 -> 6^2+8^2=100 -> 1^2+0^2+0^2=1)
 */
public class P45_HappyNumber {

    public boolean isHappy(int n) {
        java.util.Set<Integer> seen = new java.util.HashSet<>();
        while (n != 1 && seen.add(n)) {
            n = sumOfSquares(n);
        }
        return n == 1;
    }

    private int sumOfSquares(int n) {
        int sum = 0;
        while (n > 0) {
            int digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        return sum;
    }

    public static void main(String[] args) {
        P45_HappyNumber sol = new P45_HappyNumber();
        test(sol, 19, true);
        test(sol, 2, false);
        test(sol, 1, true);
        test(sol, 7, true);
        System.out.println("All tests passed.");
    }

    private static void test(P45_HappyNumber sol, int n, boolean expected) {
        boolean actual = sol.isHappy(n);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + n + " -> " + actual);
    }
}
