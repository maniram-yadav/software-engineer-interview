/**
 * LeetCode Top Interview 150 -- #43. Group Anagrams (Medium)
 *
 * Given an array of strings, group the anagrams together (any order).
 *
 * Example:
 *   Input: strs = ["eat","tea","tan","ate","nat","bat"]
 *   Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 */
public class P43_GroupAnagrams {

    public java.util.List<java.util.List<String>> groupAnagrams(String[] strs) {
        java.util.Map<String, java.util.List<String>> groups = new java.util.HashMap<>();
        for (String s : strs) {
            char[] chars = s.toCharArray();
            java.util.Arrays.sort(chars);
            String key = new String(chars);
            groups.computeIfAbsent(key, k -> new java.util.ArrayList<>()).add(s);
        }
        return new java.util.ArrayList<>(groups.values());
    }

    public static void main(String[] args) {
        P43_GroupAnagrams sol = new P43_GroupAnagrams();
        test(sol, new String[]{"eat", "tea", "tan", "ate", "nat", "bat"},
                new String[][]{{"bat"}, {"nat", "tan"}, {"ate", "eat", "tea"}});
        test(sol, new String[]{""}, new String[][]{{""}});
        test(sol, new String[]{"a"}, new String[][]{{"a"}});
        System.out.println("All tests passed.");
    }

    private static void test(P43_GroupAnagrams sol, String[] strs, String[][] expected) {
        java.util.List<java.util.List<String>> actual = sol.groupAnagrams(strs);
        java.util.List<java.util.List<String>> normalizedActual = normalize(actual);
        java.util.List<java.util.List<String>> normalizedExpected = normalize(
                java.util.Arrays.stream(expected).map(java.util.Arrays::asList).collect(java.util.stream.Collectors.toList()));
        if (!normalizedActual.equals(normalizedExpected)) {
            throw new AssertionError("Expected " + normalizedExpected + " but got " + normalizedActual);
        }
        System.out.println("PASS: " + normalizedActual);
    }

    private static java.util.List<java.util.List<String>> normalize(java.util.List<java.util.List<String>> groups) {
        java.util.List<java.util.List<String>> sorted = new java.util.ArrayList<>();
        for (java.util.List<String> g : groups) {
            java.util.List<String> copy = new java.util.ArrayList<>(g);
            java.util.Collections.sort(copy);
            sorted.add(copy);
        }
        sorted.sort((a, b) -> a.toString().compareTo(b.toString()));
        return sorted;
    }
}
