/**
 * LeetCode Top Interview 150 -- #24. Text Justification (Hard)
 *
 * Given an array of words and a width maxWidth, format the text so each
 * line has exactly maxWidth characters, fully justified (extra spaces
 * distributed as evenly as possible, left-heavy); the last line is
 * left-justified with single spaces.
 *
 * Example:
 *   Input: words = ["This","is","an","example","of","text","justification."], maxWidth = 16
 *   Output:
 *   [
 *     "This    is    an",
 *     "example  of text",
 *     "justification.  "
 *   ]
 */
public class P24_TextJustification {

    public java.util.List<String> fullJustify(String[] words, int maxWidth) {
        java.util.List<String> result = new java.util.ArrayList<>();
        int n = words.length;
        int i = 0;

        while (i < n) {
            int j = i;
            int lineLength = 0;
            while (j < n && lineLength + words[j].length() + (j - i) <= maxWidth) {
                lineLength += words[j].length();
                j++;
            }

            int numWords = j - i;
            int numSpaces = maxWidth - lineLength;
            StringBuilder line = new StringBuilder();

            if (j == n || numWords == 1) {
                for (int k = i; k < j; k++) {
                    line.append(words[k]);
                    if (k < j - 1) line.append(' ');
                }
                while (line.length() < maxWidth) line.append(' ');
            } else {
                int gaps = numWords - 1;
                int spacesPerGap = numSpaces / gaps;
                int extra = numSpaces % gaps;
                for (int k = i; k < j - 1; k++) {
                    line.append(words[k]);
                    int spaces = spacesPerGap + (k - i < extra ? 1 : 0);
                    for (int s = 0; s < spaces; s++) line.append(' ');
                }
                line.append(words[j - 1]);
            }

            result.add(line.toString());
            i = j;
        }

        return result;
    }

    public static void main(String[] args) {
        P24_TextJustification sol = new P24_TextJustification();
        test(sol,
                new String[]{"This", "is", "an", "example", "of", "text", "justification."}, 16,
                new String[]{"This    is    an", "example  of text", "justification.  "});
        test(sol,
                new String[]{"What", "must", "be", "acknowledgment", "shall", "be"}, 16,
                new String[]{"What   must   be", "acknowledgment  ", "shall be        "});
        test(sol,
                new String[]{"Listen", "to", "many,", "speak", "to", "a", "few."}, 6,
                new String[]{"Listen", "to    ", "many, ", "speak ", "to   a", "few.  "});
        System.out.println("All tests passed.");
    }

    private static void test(P24_TextJustification sol, String[] words, int maxWidth, String[] expected) {
        java.util.List<String> actual = sol.fullJustify(words, maxWidth);
        java.util.List<String> expectedList = java.util.Arrays.asList(expected);
        if (!actual.equals(expectedList)) {
            throw new AssertionError("Expected " + expectedList + " but got " + actual);
        }
        System.out.println("PASS: " + actual);
    }
}
