/**
 * LeetCode Top Interview 150 -- #100. Word Search II (Hard)
 *
 * Given an m x n board of characters and a list of words, return all words
 * from the list that can be formed by a path of adjacent cells (no cell
 * reused within a single word).
 *
 * Example:
 *   Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
 *   Output: ["eat","oath"]
 */
public class P100_WordSearchII {

    static class TrieNode {
        java.util.Map<Character, TrieNode> children = new java.util.HashMap<>();
        String word = null;
    }

    public java.util.List<String> findWords(char[][] board, String[] words) {
        TrieNode root = new TrieNode();
        for (String w : words) {
            TrieNode node = root;
            for (char c : w.toCharArray()) {
                node = node.children.computeIfAbsent(c, k -> new TrieNode());
            }
            node.word = w;
        }

        java.util.List<String> result = new java.util.ArrayList<>();
        int rows = board.length, cols = board[0].length;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                dfs(board, r, c, root, result);
            }
        }
        return result;
    }

    private void dfs(char[][] board, int r, int c, TrieNode node, java.util.List<String> result) {
        if (r < 0 || r >= board.length || c < 0 || c >= board[0].length) return;
        char ch = board[r][c];
        if (ch == '#' || !node.children.containsKey(ch)) return;

        TrieNode next = node.children.get(ch);
        if (next.word != null) {
            result.add(next.word);
            next.word = null;
        }

        board[r][c] = '#';
        dfs(board, r + 1, c, next, result);
        dfs(board, r - 1, c, next, result);
        dfs(board, r, c + 1, next, result);
        dfs(board, r, c - 1, next, result);
        board[r][c] = ch;
    }

    public static void main(String[] args) {
        P100_WordSearchII sol = new P100_WordSearchII();

        char[][] board1 = {
                {'o', 'a', 'a', 'n'},
                {'e', 't', 'a', 'e'},
                {'i', 'h', 'k', 'r'},
                {'i', 'f', 'l', 'v'}
        };
        test(sol, board1, new String[]{"oath", "pea", "eat", "rain"}, new String[]{"eat", "oath"});

        char[][] board2 = {{'a', 'b'}, {'c', 'd'}};
        test(sol, board2, new String[]{"abcb"}, new String[]{});

        System.out.println("All tests passed.");
    }

    private static void test(P100_WordSearchII sol, char[][] board, String[] words, String[] expected) {
        java.util.List<String> actual = sol.findWords(board, words);
        java.util.List<String> sortedActual = new java.util.ArrayList<>(actual);
        java.util.Collections.sort(sortedActual);
        java.util.List<String> sortedExpected = new java.util.ArrayList<>(java.util.Arrays.asList(expected));
        java.util.Collections.sort(sortedExpected);
        if (!sortedActual.equals(sortedExpected)) {
            throw new AssertionError("Expected " + sortedExpected + " but got " + sortedActual);
        }
        System.out.println("PASS: " + sortedActual);
    }
}
