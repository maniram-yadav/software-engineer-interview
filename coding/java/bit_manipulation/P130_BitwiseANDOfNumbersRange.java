/**
 * LeetCode Top Interview 150 -- #130. Bitwise AND of Numbers Range (Medium)
 *
 * Given two integers left and right, return the bitwise AND of all numbers
 * in the range [left, right] inclusive.
 *
 * Example:
 *   Input: left = 5, right = 7
 *   Output: 4
 */
public class P130_BitwiseANDOfNumbersRange {

    public int rangeBitwiseAnd(int left, int right) {
        int shift = 0;
        while (left < right) {
            left >>= 1;
            right >>= 1;
            shift++;
        }
        return left << shift;
    }

    public static void main(String[] args) {
        P130_BitwiseANDOfNumbersRange sol = new P130_BitwiseANDOfNumbersRange();
        test(sol, 5, 7, 4);
        test(sol, 0, 0, 0);
        test(sol, 1, 2147483647, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P130_BitwiseANDOfNumbersRange sol, int left, int right, int expected) {
        int actual = sol.rangeBitwiseAnd(left, right);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: [" + left + "," + right + "] -> " + actual);
    }
}
