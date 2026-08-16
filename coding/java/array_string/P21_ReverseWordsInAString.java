/**
 * LeetCode Top Interview 150 -- #21. Reverse Words in a String (Medium)
 *
 * Given a string s, reverse the order of the words (words are separated by
 * one or more spaces); collapse extra spaces and trim leading/trailing
 * spaces.
 *
 * Example:
 *   Input: s = "  the sky is blue  "
 *   Output: "blue is sky the"
 */
public class P21_ReverseWordsInAString {

    public String reverseWords(String s) {
        String[] words = s.trim().split("\\s+");
        StringBuilder sb = new StringBuilder();
        for (int i = words.length - 1; i >= 0; i--) {
            sb.append(words[i]);
            if (i > 0) sb.append(' ');
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        P21_ReverseWordsInAString sol = new P21_ReverseWordsInAString();
        test(sol, "  the sky is blue  ", "blue is sky the");
        test(sol, "hello world", "world hello");
        test(sol, "a good   example", "example good a");
        test(sol, "  Bob    Loves  Alice   ", "Alice Loves Bob");
        System.out.println("All tests passed.");
    }

    private static void test(P21_ReverseWordsInAString sol, String s, String expected) {
        String actual = sol.reverseWords(s);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: \"" + s + "\" -> \"" + actual + "\"");
    }
}
