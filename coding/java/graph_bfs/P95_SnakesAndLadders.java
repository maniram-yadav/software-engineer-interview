/**
 * LeetCode Top Interview 150 -- #95. Snakes and Ladders (Medium)
 *
 * Given an n x n boustrophedon-numbered board with some snakes/ladders
 * (board[r][c] != -1 means a jump), return the minimum number of dice
 * rolls (1-6) to reach the last square, or -1.
 *
 * Example:
 *   Input: board = 6x6 grid with some ladder/snake destinations
 *   Output: 4
 */
public class P95_SnakesAndLadders {

    public int snakesAndLadders(int[][] board) {
        int n = board.length;
        int[] flat = new int[n * n + 1];
        int idx = 1;
        boolean leftToRight = true;
        for (int r = n - 1; r >= 0; r--) {
            if (leftToRight) {
                for (int c = 0; c < n; c++) flat[idx++] = board[r][c];
            } else {
                for (int c = n - 1; c >= 0; c--) flat[idx++] = board[r][c];
            }
            leftToRight = !leftToRight;
        }

        int[] dist = new int[n * n + 1];
        java.util.Arrays.fill(dist, -1);
        dist[1] = 0;
        java.util.Queue<Integer> queue = new java.util.LinkedList<>();
        queue.add(1);

        while (!queue.isEmpty()) {
            int cur = queue.poll();
            if (cur == n * n) return dist[cur];
            for (int next = cur + 1; next <= Math.min(cur + 6, n * n); next++) {
                int dest = flat[next] == -1 ? next : flat[next];
                if (dist[dest] == -1) {
                    dist[dest] = dist[cur] + 1;
                    queue.add(dest);
                }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        P95_SnakesAndLadders sol = new P95_SnakesAndLadders();
        test(sol, new int[][]{
                {-1, -1, -1, -1, -1, -1},
                {-1, -1, -1, -1, -1, -1},
                {-1, -1, -1, -1, -1, -1},
                {-1, 35, -1, -1, 13, -1},
                {-1, -1, -1, -1, -1, -1},
                {-1, 15, -1, -1, -1, -1}
        }, 4);
        test(sol, new int[][]{{-1, -1}, {-1, 3}}, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P95_SnakesAndLadders sol, int[][] board, int expected) {
        int actual = sol.snakesAndLadders(board);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
