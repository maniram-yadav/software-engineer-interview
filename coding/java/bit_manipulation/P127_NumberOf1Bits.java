/**
 * LeetCode Top Interview 150 -- #127. Number of 1 Bits (Easy)
 *
 * Given a 32-bit unsigned integer, return the number of set bits (Hamming
 * weight).
 *
 * Example:
 *   Input: n = 00000000000000000000000000001011
 *   Output: 3
 */
public class P127_NumberOf1Bits {

    public int hammingWeight(int n) {
        int count = 0;
        for (int i = 0; i < 32; i++) {
            if (((n >>> i) & 1) == 1) count++;
        }
        return count;
    }

    public static void main(String[] args) {
        P127_NumberOf1Bits sol = new P127_NumberOf1Bits();
        test(sol, 11, 3);
        test(sol, 128, 1);
        test(sol, -3, 31);
        System.out.println("All tests passed.");
    }

    private static void test(P127_NumberOf1Bits sol, int n, int expected) {
        int actual = sol.hammingWeight(n);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + n + " -> " + actual);
    }
}
