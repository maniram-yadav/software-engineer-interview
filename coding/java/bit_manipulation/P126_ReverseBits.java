/**
 * LeetCode Top Interview 150 -- #126. Reverse Bits (Easy)
 *
 * Reverse the bits of a given 32-bit unsigned integer.
 *
 * Example:
 *   Input: n = 00000010100101000001111010011100
 *   Output: 964176192 (00111001011110000010100101000000)
 */
public class P126_ReverseBits {

    public int reverseBits(int n) {
        int result = 0;
        for (int i = 0; i < 32; i++) {
            result = (result << 1) | (n & 1);
            n >>>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        P126_ReverseBits sol = new P126_ReverseBits();
        test(sol, 43261596, 964176192);
        test(sol, 0, 0);
        test(sol, -1, -1);
        System.out.println("All tests passed.");
    }

    private static void test(P126_ReverseBits sol, int n, int expected) {
        int actual = sol.reverseBits(n);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + n + " -> " + actual);
    }
}
