/**
 * LeetCode Top Interview 150 -- #136. Max Points on a Line (Hard)
 *
 * Given an array of points on the X-Y plane, return the maximum number of
 * points that lie on the same straight line.
 *
 * Example:
 *   Input: points = [[1,1],[2,2],[3,3]]
 *   Output: 3
 */
public class P136_MaxPointsOnALine {

    public int maxPoints(int[][] points) {
        int n = points.length;
        if (n <= 2) return n;

        int maxCount = 1;
        for (int i = 0; i < n; i++) {
            java.util.Map<String, Integer> slopeCount = new java.util.HashMap<>();
            for (int j = 0; j < n; j++) {
                if (i == j) continue;
                int dx = points[j][0] - points[i][0];
                int dy = points[j][1] - points[i][1];
                int g = gcd(dx, dy);
                if (g != 0) {
                    dx /= g;
                    dy /= g;
                }
                if (dx < 0 || (dx == 0 && dy < 0)) {
                    dx = -dx;
                    dy = -dy;
                }
                String key = dx + "," + dy;
                slopeCount.merge(key, 1, Integer::sum);
                maxCount = Math.max(maxCount, slopeCount.get(key) + 1);
            }
        }
        return maxCount;
    }

    private int gcd(int a, int b) {
        a = Math.abs(a);
        b = Math.abs(b);
        while (b != 0) {
            int t = b;
            b = a % b;
            a = t;
        }
        return a;
    }

    public static void main(String[] args) {
        P136_MaxPointsOnALine sol = new P136_MaxPointsOnALine();
        test(sol, new int[][]{{1, 1}, {2, 2}, {3, 3}}, 3);
        test(sol, new int[][]{{1, 1}, {3, 2}, {5, 3}, {4, 1}, {2, 3}, {1, 4}}, 4);
        test(sol, new int[][]{{0, 0}}, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P136_MaxPointsOnALine sol, int[][] points, int expected) {
        int actual = sol.maxPoints(points);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(points) + " -> " + actual);
    }
}
