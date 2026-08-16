/**
 * LeetCode Top Interview 150 -- #132. Plus One (Easy)
 *
 * Given a non-empty array of digits representing a non-negative integer,
 * increment the integer by one and return the resulting digit array.
 *
 * Example:
 *   Input: digits = [1,2,3]
 *   Output: [1,2,4]
 */
public class P132_PlusOne {

    public int[] plusOne(int[] digits) {
        for (int i = digits.length - 1; i >= 0; i--) {
            if (digits[i] < 9) {
                digits[i]++;
                return digits;
            }
            digits[i] = 0;
        }
        int[] result = new int[digits.length + 1];
        result[0] = 1;
        return result;
    }

    public static void main(String[] args) {
        P132_PlusOne sol = new P132_PlusOne();
        test(sol, new int[]{1, 2, 3}, new int[]{1, 2, 4});
        test(sol, new int[]{9, 9}, new int[]{1, 0, 0});
        test(sol, new int[]{0}, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P132_PlusOne sol, int[] digits, int[] expected) {
        int[] actual = sol.plusOne(digits);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: -> " + java.util.Arrays.toString(actual));
    }
}
