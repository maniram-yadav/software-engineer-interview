/**
 * Grind 169 -- #525. Contiguous Array (Medium)
 *
 * Given a binary array nums, return the maximum length of a contiguous
 * subarray with an equal number of 0s and 1s.
 *
 * Example:
 *   Input: nums = [0,1,0,1]
 *   Output: 4
 */
public class P525_ContiguousArray {

    public int findMaxLength(int[] nums) {
        java.util.Map<Integer, Integer> firstIndex = new java.util.HashMap<>();
        firstIndex.put(0, -1);
        int count = 0, maxLen = 0;
        for (int i = 0; i < nums.length; i++) {
            count += (nums[i] == 1) ? 1 : -1;
            if (firstIndex.containsKey(count)) {
                maxLen = Math.max(maxLen, i - firstIndex.get(count));
            } else {
                firstIndex.put(count, i);
            }
        }
        return maxLen;
    }

    public static void main(String[] args) {
        P525_ContiguousArray sol = new P525_ContiguousArray();
        test(sol, new int[]{0, 1, 0, 1}, 4);
        test(sol, new int[]{0, 1}, 2);
        test(sol, new int[]{0, 0, 1, 0, 0, 0, 1, 1}, 6);
        System.out.println("All tests passed.");
    }

    private static void test(P525_ContiguousArray sol, int[] nums, int expected) {
        int actual = sol.findMaxLength(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
