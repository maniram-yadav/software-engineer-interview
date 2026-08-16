/**
 * LeetCode Top Interview 150 -- #41. Word Pattern (Easy)
 *
 * Given a pattern and a string s, find if s follows the same pattern -- a
 * full bijection between letters in pattern and words in s.
 *
 * Example:
 *   Input: pattern = "abba", s = "dog cat cat dog"
 *   Output: true
 */
public class P41_WordPattern {

    public boolean wordPattern(String pattern, String s) {
        String[] words = s.split(" ");
        if (pattern.length() != words.length) return false;

        java.util.Map<Character, String> charToWord = new java.util.HashMap<>();
        java.util.Map<String, Character> wordToChar = new java.util.HashMap<>();

        for (int i = 0; i < pattern.length(); i++) {
            char c = pattern.charAt(i);
            String w = words[i];
            if (charToWord.containsKey(c) && !charToWord.get(c).equals(w)) return false;
            if (wordToChar.containsKey(w) && wordToChar.get(w) != c) return false;
            charToWord.put(c, w);
            wordToChar.put(w, c);
        }
        return true;
    }

    public static void main(String[] args) {
        P41_WordPattern sol = new P41_WordPattern();
        test(sol, "abba", "dog cat cat dog", true);
        test(sol, "abba", "dog cat cat fish", false);
        test(sol, "aaaa", "dog cat cat dog", false);
        test(sol, "abba", "dog dog dog dog", false);
        System.out.println("All tests passed.");
    }

    private static void test(P41_WordPattern sol, String pattern, String s, boolean expected) {
        boolean actual = sol.wordPattern(pattern, s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: pattern=\"" + pattern + "\" s=\"" + s + "\" -> " + actual);
    }
}
