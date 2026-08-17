/**
 * Grind 169 -- #973. K Closest Points to Origin (Medium)
 *
 * Given an array of points on the X-Y plane and an integer k, return the k
 * closest points to the origin (any order).
 *
 * Example:
 *   Input: points = [[1,3],[-2,2]], k = 1
 *   Output: [[-2,2]]
 */
public class P973_KClosestPointsToOrigin {

    public int[][] kClosest(int[][] points, int k) {
        java.util.PriorityQueue<int[]> maxHeap = new java.util.PriorityQueue<>(
                (a, b) -> (b[0] * b[0] + b[1] * b[1]) - (a[0] * a[0] + a[1] * a[1]));
        for (int[] p : points) {
            maxHeap.add(p);
            if (maxHeap.size() > k) maxHeap.poll();
        }
        return maxHeap.toArray(new int[0][]);
    }

    public static void main(String[] args) {
        P973_KClosestPointsToOrigin sol = new P973_KClosestPointsToOrigin();
        test(sol, new int[][]{{1, 3}, {-2, 2}}, 1, new int[][]{{-2, 2}});
        test(sol, new int[][]{{3, 3}, {5, -1}, {-2, 4}}, 2, new int[][]{{3, 3}, {-2, 4}});
        System.out.println("All tests passed.");
    }

    private static void test(P973_KClosestPointsToOrigin sol, int[][] points, int k, int[][] expected) {
        int[][] actual = sol.kClosest(points, k);
        java.util.Set<java.util.List<Integer>> actualSet = new java.util.HashSet<>();
        for (int[] p : actual) actualSet.add(java.util.List.of(p[0], p[1]));
        java.util.Set<java.util.List<Integer>> expectedSet = new java.util.HashSet<>();
        for (int[] p : expected) expectedSet.add(java.util.List.of(p[0], p[1]));
        if (!actualSet.equals(expectedSet)) {
            throw new AssertionError("Expected " + expectedSet + " but got " + actualSet);
        }
        System.out.println("PASS: k=" + k + " -> " + actualSet);
    }
}
