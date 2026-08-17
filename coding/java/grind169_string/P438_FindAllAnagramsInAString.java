/**
 * Grind 169 -- #438. Find All Anagrams in a String (Medium)
 *
 * Given strings s and p, return all start indices of p's anagrams in s.
 *
 * Example:
 *   Input: s = "cbaebabacd", p = "abc"
 *   Output: [0,6]
 */
public class P438_FindAllAnagramsInAString {

    public java.util.List<Integer> findAnagrams(String s, String p) {
        java.util.List<Integer> result = new java.util.ArrayList<>();
        if (s.length() < p.length()) return result;

        int[] need = new int[26], window = new int[26];
        for (char c : p.toCharArray()) need[c - 'a']++;

        for (int i = 0; i < s.length(); i++) {
            window[s.charAt(i) - 'a']++;
            if (i >= p.length()) window[s.charAt(i - p.length()) - 'a']--;
            if (i >= p.length() - 1 && java.util.Arrays.equals(need, window)) {
                result.add(i - p.length() + 1);
            }
        }
        return result;
    }

    public static void main(String[] args) {
        P438_FindAllAnagramsInAString sol = new P438_FindAllAnagramsInAString();
        test(sol, "cbaebabacd", "abc", new int[]{0, 6});
        test(sol, "abab", "ab", new int[]{0, 1, 2});
        System.out.println("All tests passed.");
    }

    private static void test(P438_FindAllAnagramsInAString sol, String s, String p, int[] expected) {
        java.util.List<Integer> actual = sol.findAnagrams(s, p);
        int[] actualArr = actual.stream().mapToInt(Integer::intValue).toArray();
        if (!java.util.Arrays.equals(actualArr, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + actual);
        }
        System.out.println("PASS: s=\"" + s + "\" p=\"" + p + "\" -> " + actual);
    }
}
