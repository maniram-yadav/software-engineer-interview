/**
 * LeetCode Top Interview 150 -- #39. Ransom Note (Easy)
 *
 * Given strings ransomNote and magazine, return true if ransomNote can be
 * constructed using letters from magazine (each letter used at most once).
 *
 * Example:
 *   Input: ransomNote = "aa", magazine = "aab"
 *   Output: true
 */
public class P39_RansomNote {

    public boolean canConstruct(String ransomNote, String magazine) {
        int[] counts = new int[26];
        for (char c : magazine.toCharArray()) counts[c - 'a']++;
        for (char c : ransomNote.toCharArray()) {
            if (--counts[c - 'a'] < 0) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        P39_RansomNote sol = new P39_RansomNote();
        test(sol, "aa", "aab", true);
        test(sol, "a", "b", false);
        test(sol, "aa", "ab", false);
        test(sol, "", "anything", true);
        System.out.println("All tests passed.");
    }

    private static void test(P39_RansomNote sol, String ransomNote, String magazine, boolean expected) {
        boolean actual = sol.canConstruct(ransomNote, magazine);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: ransomNote=\"" + ransomNote + "\" magazine=\"" + magazine + "\" -> " + actual);
    }
}
