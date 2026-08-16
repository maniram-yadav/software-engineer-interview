/**
 * LeetCode Top Interview 150 -- #1. Merge Sorted Array (Easy)
 *
 * You're given two sorted integer arrays nums1 and nums2, where nums1 has
 * extra trailing space (length m + n) to hold all elements. Merge nums2
 * into nums1 in place so the result is a single sorted array.
 *
 * Example:
 *   Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
 *   Output: [1,2,2,3,5,6]
 */
public class P01_MergeSortedArray {

    public void merge(int[] nums1, int m, int[] nums2, int n) {
        int i = m - 1, j = n - 1, k = m + n - 1;
        while (j >= 0) {
            if (i >= 0 && nums1[i] > nums2[j]) {
                nums1[k--] = nums1[i--];
            } else {
                nums1[k--] = nums2[j--];
            }
        }
    }

    public static void main(String[] args) {
        P01_MergeSortedArray sol = new P01_MergeSortedArray();
        test(sol, new int[]{1, 2, 3, 0, 0, 0}, 3, new int[]{2, 5, 6}, 3, new int[]{1, 2, 2, 3, 5, 6});
        test(sol, new int[]{1}, 1, new int[]{}, 0, new int[]{1});
        test(sol, new int[]{0}, 0, new int[]{1}, 1, new int[]{1});
        test(sol, new int[]{4, 5, 6, 0, 0, 0}, 3, new int[]{1, 2, 3}, 3, new int[]{1, 2, 3, 4, 5, 6});
        System.out.println("All tests passed.");
    }

    private static void test(P01_MergeSortedArray sol, int[] nums1, int m, int[] nums2, int n, int[] expected) {
        sol.merge(nums1, m, nums2, n);
        if (!java.util.Arrays.equals(nums1, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(nums1));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(expected));
    }
}
