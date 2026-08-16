/**
 * LeetCode Top Interview 150 -- #51. Minimum Number of Arrows to Burst Balloons (Medium)
 *
 * Balloons are represented as horizontal diameter intervals [xstart, xend].
 * An arrow shot straight up at x bursts every balloon whose interval
 * contains x. Return the minimum number of arrows needed to burst all
 * balloons.
 *
 * Example:
 *   Input: points = [[10,16],[2,8],[1,6],[7,12]]
 *   Output: 2
 */
public class P51_MinimumNumberOfArrowsToBurstBalloons {

    public int findMinArrowShots(int[][] points) {
        if (points.length == 0) return 0;
        java.util.Arrays.sort(points, (a, b) -> Long.compare(a[1], b[1]));

        int arrows = 1;
        long end = points[0][1];
        for (int[] p : points) {
            if (p[0] > end) {
                arrows++;
                end = p[1];
            }
        }
        return arrows;
    }

    public static void main(String[] args) {
        P51_MinimumNumberOfArrowsToBurstBalloons sol = new P51_MinimumNumberOfArrowsToBurstBalloons();
        test(sol, new int[][]{{10, 16}, {2, 8}, {1, 6}, {7, 12}}, 2);
        test(sol, new int[][]{{1, 2}, {3, 4}, {5, 6}, {7, 8}}, 4);
        test(sol, new int[][]{{1, 2}, {2, 3}, {3, 4}, {4, 5}}, 2);
        System.out.println("All tests passed.");
    }

    private static void test(P51_MinimumNumberOfArrowsToBurstBalloons sol, int[][] points, int expected) {
        int actual = sol.findMinArrowShots(points);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(points) + " -> " + actual);
    }
}
