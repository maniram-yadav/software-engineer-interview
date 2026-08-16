/**
 * LeetCode Top Interview 150 -- #32. Substring with Concatenation of All Words (Hard)
 *
 * Given a string s and an array of same-length words, return the starting
 * indices of all substrings in s that are a concatenation of every word in
 * words exactly once, in any order.
 *
 * Example:
 *   Input: s = "barfoothefoobarman", words = ["foo","bar"]
 *   Output: [0,9]
 */
public class P32_SubstringWithConcatenationOfAllWords {

    public java.util.List<Integer> findSubstring(String s, String[] words) {
        java.util.List<Integer> result = new java.util.ArrayList<>();
        if (words.length == 0) return result;

        int wordLen = words[0].length();
        int numWords = words.length;
        int totalLen = wordLen * numWords;
        if (s.length() < totalLen) return result;

        java.util.Map<String, Integer> wordCount = new java.util.HashMap<>();
        for (String w : words) wordCount.merge(w, 1, Integer::sum);

        for (int i = 0; i + totalLen <= s.length(); i++) {
            java.util.Map<String, Integer> seen = new java.util.HashMap<>();
            int j = 0;
            for (; j < numWords; j++) {
                int start = i + j * wordLen;
                String word = s.substring(start, start + wordLen);
                if (!wordCount.containsKey(word)) break;
                seen.merge(word, 1, Integer::sum);
                if (seen.get(word) > wordCount.get(word)) break;
            }
            if (j == numWords) result.add(i);
        }
        return result;
    }

    public static void main(String[] args) {
        P32_SubstringWithConcatenationOfAllWords sol = new P32_SubstringWithConcatenationOfAllWords();
        test(sol, "barfoothefoobarman", new String[]{"foo", "bar"}, new int[]{0, 9});
        test(sol, "wordgoodgoodgoodbestword", new String[]{"word", "good", "best", "word"}, new int[]{});
        test(sol, "barfoofoobarthefoobarman", new String[]{"bar", "foo", "the"}, new int[]{6, 9, 12});
        System.out.println("All tests passed.");
    }

    private static void test(P32_SubstringWithConcatenationOfAllWords sol, String s, String[] words, int[] expected) {
        java.util.List<Integer> actual = sol.findSubstring(s, words);
        int[] actualArr = actual.stream().mapToInt(Integer::intValue).toArray();
        if (!java.util.Arrays.equals(actualArr, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" " + java.util.Arrays.toString(words) + " -> " + actual);
    }
}
