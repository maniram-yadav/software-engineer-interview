/**
 * LeetCode Top Interview 150 -- #2. Remove Element (Easy)
 *
 * Given an array nums and a value val, remove all occurrences of val in
 * place and return the new length. Order doesn't matter, and elements
 * beyond the returned length are ignored.
 *
 * Example:
 *   Input: nums = [3,2,2,3], val = 3
 *   Output: 2, nums = [2,2,_,_]
 */
public class P02_RemoveElement {

    public int removeElement(int[] nums, int val) {
        int k = 0;
        for (int num : nums) {
            if (num != val) {
                nums[k++] = num;
            }
        }
        return k;
    }

    public static void main(String[] args) {
        P02_RemoveElement sol = new P02_RemoveElement();
        test(sol, new int[]{3, 2, 2, 3}, 3, 2, new int[]{2, 2});
        test(sol, new int[]{0, 1, 2, 2, 3, 0, 4, 2}, 2, 5, new int[]{0, 0, 1, 3, 4});
        test(sol, new int[]{}, 1, 0, new int[]{});
        test(sol, new int[]{1, 1, 1}, 1, 0, new int[]{});
        System.out.println("All tests passed.");
    }

    private static void test(P02_RemoveElement sol, int[] nums, int val, int expectedLen, int[] expectedElementsSorted) {
        int k = sol.removeElement(nums, val);
        if (k != expectedLen) {
            throw new AssertionError("Expected length " + expectedLen + " but got " + k);
        }
        int[] actual = java.util.Arrays.copyOf(nums, k);
        java.util.Arrays.sort(actual);
        if (!java.util.Arrays.equals(actual, expectedElementsSorted)) {
            throw new AssertionError("Expected elements " + java.util.Arrays.toString(expectedElementsSorted) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: length=" + k + " nums=" + java.util.Arrays.toString(actual));
    }
}
