/**
 * LeetCode Top Interview 150 -- #25. Valid Palindrome (Easy)
 *
 * Given a string s, considering only alphanumeric characters and ignoring
 * case, determine if it reads the same forward and backward.
 *
 * Example:
 *   Input: s = "A man, a plan, a canal: Panama"
 *   Output: true
 */
public class P25_ValidPalindrome {

    public boolean isPalindrome(String s) {
        int left = 0, right = s.length() - 1;
        while (left < right) {
            while (left < right && !Character.isLetterOrDigit(s.charAt(left))) left++;
            while (left < right && !Character.isLetterOrDigit(s.charAt(right))) right--;
            if (Character.toLowerCase(s.charAt(left)) != Character.toLowerCase(s.charAt(right))) {
                return false;
            }
            left++;
            right--;
        }
        return true;
    }

    public static void main(String[] args) {
        P25_ValidPalindrome sol = new P25_ValidPalindrome();
        test(sol, "A man, a plan, a canal: Panama", true);
        test(sol, "race a car", false);
        test(sol, " ", true);
        test(sol, "0P", false);
        System.out.println("All tests passed.");
    }

    private static void test(P25_ValidPalindrome sol, String s, boolean expected) {
        boolean actual = sol.isPalindrome(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
