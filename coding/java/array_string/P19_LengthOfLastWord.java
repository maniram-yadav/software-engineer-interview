/**
 * LeetCode Top Interview 150 -- #19. Length of Last Word (Easy)
 *
 * Given a string s of words separated by spaces, return the length of the
 * last word.
 *
 * Example:
 *   Input: s = "Hello World"
 *   Output: 5
 */
public class P19_LengthOfLastWord {

    public int lengthOfLastWord(String s) {
        int i = s.length() - 1;
        while (i >= 0 && s.charAt(i) == ' ') i--;
        int length = 0;
        while (i >= 0 && s.charAt(i) != ' ') {
            length++;
            i--;
        }
        return length;
    }

    public static void main(String[] args) {
        P19_LengthOfLastWord sol = new P19_LengthOfLastWord();
        test(sol, "Hello World", 5);
        test(sol, "   fly me   to   the moon  ", 4);
        test(sol, "luffy is still joyboy", 6);
        test(sol, "a", 1);
        System.out.println("All tests passed.");
    }

    private static void test(P19_LengthOfLastWord sol, String s, int expected) {
        int actual = sol.lengthOfLastWord(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
