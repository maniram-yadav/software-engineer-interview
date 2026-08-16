/**
 * LeetCode Top Interview 150 -- #120. Median of Two Sorted Arrays (Hard)
 *
 * Given two sorted arrays nums1 and nums2 of size m and n, return the
 * median of the combined sorted array in O(log(m+n)) time.
 *
 * Example:
 *   Input: nums1 = [1,3], nums2 = [2]
 *   Output: 2.0
 */
public class P120_MedianOfTwoSortedArrays {

    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        if (nums1.length > nums2.length) return findMedianSortedArrays(nums2, nums1);

        int m = nums1.length, n = nums2.length;
        int low = 0, high = m;
        while (low <= high) {
            int partitionX = (low + high) / 2;
            int partitionY = (m + n + 1) / 2 - partitionX;

            int maxX = (partitionX == 0) ? Integer.MIN_VALUE : nums1[partitionX - 1];
            int minX = (partitionX == m) ? Integer.MAX_VALUE : nums1[partitionX];
            int maxY = (partitionY == 0) ? Integer.MIN_VALUE : nums2[partitionY - 1];
            int minY = (partitionY == n) ? Integer.MAX_VALUE : nums2[partitionY];

            if (maxX <= minY && maxY <= minX) {
                if ((m + n) % 2 == 0) {
                    return (Math.max(maxX, maxY) + Math.min(minX, minY)) / 2.0;
                } else {
                    return Math.max(maxX, maxY);
                }
            } else if (maxX > minY) {
                high = partitionX - 1;
            } else {
                low = partitionX + 1;
            }
        }
        throw new IllegalArgumentException("Input arrays are not sorted");
    }

    public static void main(String[] args) {
        P120_MedianOfTwoSortedArrays sol = new P120_MedianOfTwoSortedArrays();
        test(sol, new int[]{1, 3}, new int[]{2}, 2.0);
        test(sol, new int[]{1, 2}, new int[]{3, 4}, 2.5);
        test(sol, new int[]{}, new int[]{1}, 1.0);
        System.out.println("All tests passed.");
    }

    private static void test(P120_MedianOfTwoSortedArrays sol, int[] nums1, int[] nums2, double expected) {
        double actual = sol.findMedianSortedArrays(nums1, nums2);
        if (Math.abs(actual - expected) > 1e-9) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums1) + " " + java.util.Arrays.toString(nums2) + " -> " + actual);
    }
}
