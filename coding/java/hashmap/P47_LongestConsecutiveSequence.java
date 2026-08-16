/**
 * LeetCode Top Interview 150 -- #47. Longest Consecutive Sequence (Medium)
 *
 * Given an unsorted array of integers, return the length of the longest
 * run of consecutive integers, in O(n) time.
 *
 * Example:
 *   Input: nums = [100,4,200,1,3,2]
 *   Output: 4   (the sequence [1,2,3,4])
 */
public class P47_LongestConsecutiveSequence {

    public int longestConsecutive(int[] nums) {
        java.util.Set<Integer> set = new java.util.HashSet<>();
        for (int n : nums) set.add(n);

        int longest = 0;
        for (int n : set) {
            if (!set.contains(n - 1)) {
                int length = 1;
                while (set.contains(n + length)) length++;
                longest = Math.max(longest, length);
            }
        }
        return longest;
    }

    public static void main(String[] args) {
        P47_LongestConsecutiveSequence sol = new P47_LongestConsecutiveSequence();
        test(sol, new int[]{100, 4, 200, 1, 3, 2}, 4);
        test(sol, new int[]{0, 3, 7, 2, 5, 8, 4, 6, 0, 1}, 9);
        test(sol, new int[]{}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P47_LongestConsecutiveSequence sol, int[] nums, int expected) {
        int actual = sol.longestConsecutive(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
