/**
 * Grind 169 -- #1197. Minimum Knight Moves (Medium)
 *
 * On an infinite chessboard, a knight starts at (0,0). Return the minimum
 * number of moves to reach (x, y).
 *
 * Example:
 *   Input: x = 2, y = 1
 *   Output: 1
 */
public class P1197_MinimumKnightMoves {

    private static final int[][] MOVES = {
            {1, 2}, {2, 1}, {-1, 2}, {-2, 1}, {1, -2}, {2, -1}, {-1, -2}, {-2, -1}
    };

    public int minKnightMoves(int x, int y) {
        x = Math.abs(x);
        y = Math.abs(y);

        java.util.Queue<int[]> queue = new java.util.LinkedList<>();
        java.util.Set<Long> visited = new java.util.HashSet<>();
        queue.add(new int[]{0, 0});
        visited.add(0L);

        int steps = 0;
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                int[] cell = queue.poll();
                if (cell[0] == x && cell[1] == y) return steps;
                for (int[] m : MOVES) {
                    int nx = cell[0] + m[0], ny = cell[1] + m[1];
                    if (nx >= -2 && ny >= -2 && nx <= x + 2 && ny <= y + 2) {
                        long key = (long) (nx + 1000) * 10000 + (ny + 1000);
                        if (visited.add(key)) queue.add(new int[]{nx, ny});
                    }
                }
            }
            steps++;
        }
        return -1;
    }

    public static void main(String[] args) {
        P1197_MinimumKnightMoves sol = new P1197_MinimumKnightMoves();
        test(sol, 2, 1, 1);
        test(sol, 5, 5, 4);
        test(sol, 0, 0, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P1197_MinimumKnightMoves sol, int x, int y, int expected) {
        int actual = sol.minKnightMoves(x, y);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: (" + x + "," + y + ") -> " + actual);
    }
}
