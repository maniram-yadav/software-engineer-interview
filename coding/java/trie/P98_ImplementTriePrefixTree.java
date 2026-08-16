/**
 * LeetCode Top Interview 150 -- #98. Implement Trie (Prefix Tree) (Medium)
 *
 * Implement a trie with insert(word), search(word) (exact match), and
 * startsWith(prefix).
 *
 * Example:
 *   Trie trie = new Trie();
 *   trie.insert("apple");
 *   trie.search("apple");   // true
 *   trie.search("app");     // false
 *   trie.startsWith("app"); // true
 */
public class P98_ImplementTriePrefixTree {

    static class Trie {
        private final java.util.Map<Character, Trie> children = new java.util.HashMap<>();
        private boolean isEnd = false;

        public void insert(String word) {
            Trie node = this;
            for (char c : word.toCharArray()) {
                node = node.children.computeIfAbsent(c, k -> new Trie());
            }
            node.isEnd = true;
        }

        public boolean search(String word) {
            Trie node = find(word);
            return node != null && node.isEnd;
        }

        public boolean startsWith(String prefix) {
            return find(prefix) != null;
        }

        private Trie find(String s) {
            Trie node = this;
            for (char c : s.toCharArray()) {
                node = node.children.get(c);
                if (node == null) return null;
            }
            return node;
        }
    }

    public static void main(String[] args) {
        Trie trie = new Trie();
        trie.insert("apple");
        check(trie.search("apple"), true, "search(apple)");
        check(trie.search("app"), false, "search(app)");
        check(trie.startsWith("app"), true, "startsWith(app)");
        trie.insert("app");
        check(trie.search("app"), true, "search(app) after insert");
        check(trie.search("appl"), false, "search(appl)");
        check(trie.startsWith("b"), false, "startsWith(b)");
        System.out.println("All tests passed.");
    }

    private static void check(boolean actual, boolean expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
