/**
 * LeetCode Top Interview 150 -- #134. Sqrt(x) (Easy)
 *
 * Given a non-negative integer x, return the integer square root of x
 * (truncated), without using built-in power/sqrt functions.
 *
 * Example:
 *   Input: x = 8
 *   Output: 2
 */
public class P134_Sqrtx {

    public int mySqrt(int x) {
        if (x < 2) return x;
        long left = 1, right = x / 2;
        while (left <= right) {
            long mid = left + (right - left) / 2;
            long sq = mid * mid;
            if (sq == x) return (int) mid;
            else if (sq < x) left = mid + 1;
            else right = mid - 1;
        }
        return (int) right;
    }

    public static void main(String[] args) {
        P134_Sqrtx sol = new P134_Sqrtx();
        test(sol, 8, 2);
        test(sol, 4, 2);
        test(sol, 0, 0);
        test(sol, 1, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P134_Sqrtx sol, int x, int expected) {
        int actual = sol.mySqrt(x);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: sqrt(" + x + ") -> " + actual);
    }
}
