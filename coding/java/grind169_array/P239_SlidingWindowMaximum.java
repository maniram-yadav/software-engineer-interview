/**
 * Grind 169 -- #239. Sliding Window Maximum (Hard)
 *
 * Given an array nums and a sliding window of size k moving from left to
 * right, return the max value in the window at each position.
 *
 * Example:
 *   Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
 *   Output: [3,3,5,5,6,7]
 */
public class P239_SlidingWindowMaximum {

    public int[] maxSlidingWindow(int[] nums, int k) {
        java.util.Deque<Integer> deque = new java.util.ArrayDeque<>();
        int[] result = new int[nums.length - k + 1];
        for (int i = 0; i < nums.length; i++) {
            while (!deque.isEmpty() && deque.peekFirst() <= i - k) deque.pollFirst();
            while (!deque.isEmpty() && nums[deque.peekLast()] < nums[i]) deque.pollLast();
            deque.offerLast(i);
            if (i >= k - 1) result[i - k + 1] = nums[deque.peekFirst()];
        }
        return result;
    }

    public static void main(String[] args) {
        P239_SlidingWindowMaximum sol = new P239_SlidingWindowMaximum();
        test(sol, new int[]{1, 3, -1, -3, 5, 3, 6, 7}, 3, new int[]{3, 3, 5, 5, 6, 7});
        test(sol, new int[]{1}, 1, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P239_SlidingWindowMaximum sol, int[] nums, int k, int[] expected) {
        int[] actual = sol.maxSlidingWindow(nums, k);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(actual));
    }
}
