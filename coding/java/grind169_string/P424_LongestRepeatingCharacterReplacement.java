/**
 * Grind 169 -- #424. Longest Repeating Character Replacement (Medium)
 *
 * Given a string s and an integer k, you can replace up to k characters
 * with any other uppercase letter. Return the length of the longest
 * substring containing the same letter after such replacements.
 *
 * Example:
 *   Input: s = "ABAB", k = 2
 *   Output: 4
 */
public class P424_LongestRepeatingCharacterReplacement {

    public int characterReplacement(String s, int k) {
        int[] counts = new int[26];
        int left = 0, maxCount = 0, result = 0;
        for (int right = 0; right < s.length(); right++) {
            counts[s.charAt(right) - 'A']++;
            maxCount = Math.max(maxCount, counts[s.charAt(right) - 'A']);
            while (right - left + 1 - maxCount > k) {
                counts[s.charAt(left) - 'A']--;
                left++;
            }
            result = Math.max(result, right - left + 1);
        }
        return result;
    }

    public static void main(String[] args) {
        P424_LongestRepeatingCharacterReplacement sol = new P424_LongestRepeatingCharacterReplacement();
        test(sol, "ABAB", 2, 4);
        test(sol, "AABABBA", 1, 4);
        System.out.println("All tests passed.");
    }

    private static void test(P424_LongestRepeatingCharacterReplacement sol, String s, int k, int expected) {
        int actual = sol.characterReplacement(s, k);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" k=" + k + " -> " + actual);
    }
}
