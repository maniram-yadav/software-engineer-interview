/**
 * LeetCode Top Interview 150 -- #125. Add Binary (Easy)
 *
 * Given two binary strings a and b, return their sum as a binary string.
 *
 * Example:
 *   Input: a = "11", b = "1"
 *   Output: "100"
 */
public class P125_AddBinary {

    public String addBinary(String a, String b) {
        StringBuilder sb = new StringBuilder();
        int i = a.length() - 1, j = b.length() - 1, carry = 0;
        while (i >= 0 || j >= 0 || carry != 0) {
            int sum = carry;
            if (i >= 0) sum += a.charAt(i--) - '0';
            if (j >= 0) sum += b.charAt(j--) - '0';
            sb.append(sum % 2);
            carry = sum / 2;
        }
        return sb.reverse().toString();
    }

    public static void main(String[] args) {
        P125_AddBinary sol = new P125_AddBinary();
        test(sol, "11", "1", "100");
        test(sol, "1010", "1011", "10101");
        test(sol, "0", "0", "0");
        System.out.println("All tests passed.");
    }

    private static void test(P125_AddBinary sol, String a, String b, String expected) {
        String actual = sol.addBinary(a, b);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: " + a + " + " + b + " -> " + actual);
    }
}
