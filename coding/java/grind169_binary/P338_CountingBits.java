/**
 * Grind 169 -- #338. Counting Bits (Easy)
 *
 * Given an integer n, return an array ans of length n+1 where ans[i] is
 * the number of 1's in the binary representation of i.
 *
 * Example:
 *   Input: n = 5
 *   Output: [0,1,1,2,1,2]
 */
public class P338_CountingBits {

    public int[] countBits(int n) {
        int[] ans = new int[n + 1];
        for (int i = 1; i <= n; i++) {
            ans[i] = ans[i >> 1] + (i & 1);
        }
        return ans;
    }

    public static void main(String[] args) {
        P338_CountingBits sol = new P338_CountingBits();
        test(sol, 5, new int[]{0, 1, 1, 2, 1, 2});
        test(sol, 2, new int[]{0, 1, 1});
        System.out.println("All tests passed.");
    }

    private static void test(P338_CountingBits sol, int n, int[] expected) {
        int[] actual = sol.countBits(n);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: n=" + n + " -> " + java.util.Arrays.toString(actual));
    }
}
