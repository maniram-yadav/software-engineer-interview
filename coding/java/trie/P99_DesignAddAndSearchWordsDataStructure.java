/**
 * LeetCode Top Interview 150 -- #99. Design Add and Search Words Data Structure (Medium)
 *
 * Design a data structure supporting addWord(word) and search(word), where
 * search may contain '.' as a wildcard for any single letter.
 *
 * Example:
 *   wd.addWord("bad");
 *   wd.search("b.d"); // true
 *   wd.search("bad");  // true
 *   wd.search("..d");  // true
 */
public class P99_DesignAddAndSearchWordsDataStructure {

    static class WordDictionary {
        static class TrieNode {
            java.util.Map<Character, TrieNode> children = new java.util.HashMap<>();
            boolean isEnd = false;
        }

        private final TrieNode root = new TrieNode();

        public void addWord(String word) {
            TrieNode node = root;
            for (char c : word.toCharArray()) {
                node = node.children.computeIfAbsent(c, k -> new TrieNode());
            }
            node.isEnd = true;
        }

        public boolean search(String word) {
            return dfs(word, 0, root);
        }

        private boolean dfs(String word, int idx, TrieNode node) {
            if (idx == word.length()) return node.isEnd;
            char c = word.charAt(idx);
            if (c == '.') {
                for (TrieNode child : node.children.values()) {
                    if (dfs(word, idx + 1, child)) return true;
                }
                return false;
            } else {
                TrieNode child = node.children.get(c);
                return child != null && dfs(word, idx + 1, child);
            }
        }
    }

    public static void main(String[] args) {
        WordDictionary wd = new WordDictionary();
        wd.addWord("bad");
        wd.addWord("dad");
        wd.addWord("mad");
        check(wd.search("pad"), false, "search(pad)");
        check(wd.search("bad"), true, "search(bad)");
        check(wd.search(".ad"), true, "search(.ad)");
        check(wd.search("b.."), true, "search(b..)");
        check(wd.search("b.d"), true, "search(b.d)");
        check(wd.search("..d"), true, "search(..d)");
        check(wd.search("...."), false, "search(....)");
        System.out.println("All tests passed.");
    }

    private static void check(boolean actual, boolean expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
