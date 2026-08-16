/**
 * LeetCode Top Interview 150 -- #131. Palindrome Number (Easy)
 *
 * Given an integer x, return true if it reads the same backward as
 * forward (ideally without converting to a string).
 *
 * Example:
 *   Input: x = 121
 *   Output: true
 */
public class P131_PalindromeNumber {

    public boolean isPalindrome(int x) {
        if (x < 0 || (x % 10 == 0 && x != 0)) return false;
        int reverted = 0;
        while (x > reverted) {
            reverted = reverted * 10 + x % 10;
            x /= 10;
        }
        return x == reverted || x == reverted / 10;
    }

    public static void main(String[] args) {
        P131_PalindromeNumber sol = new P131_PalindromeNumber();
        test(sol, 121, true);
        test(sol, -121, false);
        test(sol, 10, false);
        test(sol, 12321, true);
        System.out.println("All tests passed.");
    }

    private static void test(P131_PalindromeNumber sol, int x, boolean expected) {
        boolean actual = sol.isPalindrome(x);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + x + " -> " + actual);
    }
}
