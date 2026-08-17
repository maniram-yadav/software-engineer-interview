/**
 * Grind 169 -- #733. Flood Fill (Easy)
 *
 * Given an image (2D grid of pixel values), a starting pixel (sr, sc), and
 * a new color, perform a flood fill: recolor the starting pixel and all
 * 4-directionally connected pixels of the same original color.
 *
 * Example:
 *   Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
 *   Output: [[2,2,2],[2,2,0],[2,0,1]]
 */
public class P733_FloodFill {

    public int[][] floodFill(int[][] image, int sr, int sc, int color) {
        int oldColor = image[sr][sc];
        if (oldColor != color) fill(image, sr, sc, oldColor, color);
        return image;
    }

    private void fill(int[][] image, int r, int c, int oldColor, int color) {
        if (r < 0 || r >= image.length || c < 0 || c >= image[0].length || image[r][c] != oldColor) return;
        image[r][c] = color;
        fill(image, r + 1, c, oldColor, color);
        fill(image, r - 1, c, oldColor, color);
        fill(image, r, c + 1, oldColor, color);
        fill(image, r, c - 1, oldColor, color);
    }

    public static void main(String[] args) {
        P733_FloodFill sol = new P733_FloodFill();
        test(sol, new int[][]{{1, 1, 1}, {1, 1, 0}, {1, 0, 1}}, 1, 1, 2,
                new int[][]{{2, 2, 2}, {2, 2, 0}, {2, 0, 1}});
        test(sol, new int[][]{{0, 0, 0}, {0, 0, 0}}, 0, 0, 0,
                new int[][]{{0, 0, 0}, {0, 0, 0}});
        System.out.println("All tests passed.");
    }

    private static void test(P733_FloodFill sol, int[][] image, int sr, int sc, int color, int[][] expected) {
        int[][] actual = sol.floodFill(image, sr, sc, color);
        if (!java.util.Arrays.deepEquals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.deepToString(expected) + " but got " + java.util.Arrays.deepToString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(actual));
    }
}
