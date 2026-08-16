/**
 * LeetCode Top Interview 150 -- #123. Find K Pairs with Smallest Sums (Medium)
 *
 * Given two sorted integer arrays and integer k, return the k pairs (u, v)
 * with u from nums1 and v from nums2 that have the smallest sums.
 *
 * Example:
 *   Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
 *   Output: [[1,2],[1,4],[1,6]]
 */
public class P123_FindKPairsWithSmallestSums {

    public java.util.List<java.util.List<Integer>> kSmallestPairs(int[] nums1, int[] nums2, int k) {
        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        if (nums1.length == 0 || nums2.length == 0 || k == 0) return result;

        java.util.PriorityQueue<int[]> heap = new java.util.PriorityQueue<>((a, b) -> (a[0] + a[1]) - (b[0] + b[1]));
        for (int i = 0; i < Math.min(nums1.length, k); i++) {
            heap.add(new int[]{nums1[i], nums2[0], 0});
        }

        while (k-- > 0 && !heap.isEmpty()) {
            int[] cur = heap.poll();
            result.add(java.util.List.of(cur[0], cur[1]));
            int nextJ = cur[2] + 1;
            if (nextJ < nums2.length) {
                heap.add(new int[]{cur[0], nums2[nextJ], nextJ});
            }
        }
        return result;
    }

    public static void main(String[] args) {
        P123_FindKPairsWithSmallestSums sol = new P123_FindKPairsWithSmallestSums();
        test(sol, new int[]{1, 7, 11}, new int[]{2, 4, 6}, 3, "[[1, 2], [1, 4], [1, 6]]");
        test(sol, new int[]{1, 1, 2}, new int[]{1, 2, 3}, 2, "[[1, 1], [1, 1]]");
        System.out.println("All tests passed.");
    }

    private static void test(P123_FindKPairsWithSmallestSums sol, int[] nums1, int[] nums2, int k, String expected) {
        java.util.List<java.util.List<Integer>> actual = sol.kSmallestPairs(nums1, nums2, k);
        if (!actual.toString().equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: k=" + k + " -> " + actual);
    }
}
