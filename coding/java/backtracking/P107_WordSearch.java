/**
 * LeetCode Top Interview 150 -- #107. Word Search (Medium)
 *
 * Given an m x n grid of characters and a word, return true if the word
 * can be constructed from letters of sequentially adjacent cells (no cell
 * reused).
 *
 * Example:
 *   Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
 *   Output: true
 */
public class P107_WordSearch {

    public boolean exist(char[][] board, String word) {
        int rows = board.length, cols = board[0].length;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (dfs(board, r, c, word, 0)) return true;
            }
        }
        return false;
    }

    private boolean dfs(char[][] board, int r, int c, String word, int idx) {
        if (idx == word.length()) return true;
        if (r < 0 || r >= board.length || c < 0 || c >= board[0].length || board[r][c] != word.charAt(idx)) {
            return false;
        }
        char temp = board[r][c];
        board[r][c] = '#';
        boolean found = dfs(board, r + 1, c, word, idx + 1)
                || dfs(board, r - 1, c, word, idx + 1)
                || dfs(board, r, c + 1, word, idx + 1)
                || dfs(board, r, c - 1, word, idx + 1);
        board[r][c] = temp;
        return found;
    }

    public static void main(String[] args) {
        P107_WordSearch sol = new P107_WordSearch();
        char[][] board = {
                {'A', 'B', 'C', 'E'},
                {'S', 'F', 'C', 'S'},
                {'A', 'D', 'E', 'E'}
        };
        test(sol, board, "ABCCED", true);
        test(sol, board, "SEE", true);
        test(sol, board, "ABCB", false);
        System.out.println("All tests passed.");
    }

    private static void test(P107_WordSearch sol, char[][] board, String word, boolean expected) {
        boolean actual = sol.exist(board, word);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + word + "\" -> " + actual);
    }
}
