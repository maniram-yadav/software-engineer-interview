/**
 * Grind 169 -- #336. Palindrome Pairs (Hard)
 *
 * Given a list of unique words, return all pairs of indices (i, j) such
 * that concatenating words[i] + words[j] forms a palindrome.
 *
 * Example:
 *   Input: words = ["abcd","dcba","lls","s","sssll"]
 *   Output: [[0,1],[1,0],[3,2],[2,4]]
 */
public class P336_PalindromePairs {

    public java.util.List<java.util.List<Integer>> palindromePairs(String[] words) {
        java.util.Map<String, Integer> wordToIndex = new java.util.HashMap<>();
        for (int i = 0; i < words.length; i++) wordToIndex.put(words[i], i);

        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        for (int i = 0; i < words.length; i++) {
            String w = words[i];
            for (int j = 0; j <= w.length(); j++) {
                String prefix = w.substring(0, j);
                String suffix = w.substring(j);

                if (isPalindrome(prefix)) {
                    String revSuffix = new StringBuilder(suffix).reverse().toString();
                    Integer k = wordToIndex.get(revSuffix);
                    if (k != null && k != i) result.add(java.util.List.of(k, i));
                }
                if (j != w.length() && isPalindrome(suffix)) {
                    String revPrefix = new StringBuilder(prefix).reverse().toString();
                    Integer k = wordToIndex.get(revPrefix);
                    if (k != null && k != i) result.add(java.util.List.of(i, k));
                }
            }
        }
        return result;
    }

    private boolean isPalindrome(String s) {
        int left = 0, right = s.length() - 1;
        while (left < right) {
            if (s.charAt(left++) != s.charAt(right--)) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        P336_PalindromePairs sol = new P336_PalindromePairs();

        String[] words = {"abcd", "dcba", "lls", "s", "sssll"};
        java.util.List<java.util.List<Integer>> actual = sol.palindromePairs(words);
        java.util.Set<java.util.List<Integer>> actualSet = new java.util.HashSet<>(actual);
        java.util.Set<java.util.List<Integer>> expectedSet = java.util.Set.of(
                java.util.List.of(0, 1), java.util.List.of(1, 0), java.util.List.of(3, 2), java.util.List.of(2, 4));
        if (!actualSet.equals(expectedSet)) {
            throw new AssertionError("Expected " + expectedSet + " but got " + actualSet);
        }
        System.out.println("PASS: " + actual);

        String[] words2 = {"a", ""};
        java.util.List<java.util.List<Integer>> actual2 = sol.palindromePairs(words2);
        java.util.Set<java.util.List<Integer>> actualSet2 = new java.util.HashSet<>(actual2);
        java.util.Set<java.util.List<Integer>> expectedSet2 = java.util.Set.of(java.util.List.of(0, 1), java.util.List.of(1, 0));
        if (!actualSet2.equals(expectedSet2)) {
            throw new AssertionError("Expected " + expectedSet2 + " but got " + actualSet2);
        }
        System.out.println("PASS: " + actual2);

        System.out.println("All tests passed.");
    }
}
