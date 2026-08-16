/**
 * LeetCode Top Interview 150 -- #121. Kth Largest Element in an Array (Medium)
 *
 * Given an integer array nums and integer k, return the k-th largest
 * element (k-th largest in sorted order, not the k-th distinct one).
 *
 * Example:
 *   Input: nums = [3,2,1,5,6,4], k = 2
 *   Output: 5
 */
public class P121_KthLargestElementInAnArray {

    public int findKthLargest(int[] nums, int k) {
        java.util.PriorityQueue<Integer> minHeap = new java.util.PriorityQueue<>();
        for (int n : nums) {
            minHeap.add(n);
            if (minHeap.size() > k) minHeap.poll();
        }
        return minHeap.peek();
    }

    public static void main(String[] args) {
        P121_KthLargestElementInAnArray sol = new P121_KthLargestElementInAnArray();
        test(sol, new int[]{3, 2, 1, 5, 6, 4}, 2, 5);
        test(sol, new int[]{3, 2, 3, 1, 2, 4, 5, 5, 6}, 4, 4);
        test(sol, new int[]{1}, 1, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P121_KthLargestElementInAnArray sol, int[] nums, int k, int expected) {
        int actual = sol.findKthLargest(nums, k);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " k=" + k + " -> " + actual);
    }
}
