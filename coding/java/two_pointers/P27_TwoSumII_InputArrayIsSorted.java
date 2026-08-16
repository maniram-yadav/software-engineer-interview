/**
 * LeetCode Top Interview 150 -- #27. Two Sum II - Input Array Is Sorted (Medium)
 *
 * Given a 1-indexed array numbers sorted in non-decreasing order, find two
 * numbers that add up to target and return their (1-indexed) positions.
 * Use O(1) extra space.
 *
 * Example:
 *   Input: numbers = [2,7,11,15], target = 9
 *   Output: [1,2]
 */
public class P27_TwoSumII_InputArrayIsSorted {

    public int[] twoSum(int[] numbers, int target) {
        int left = 0, right = numbers.length - 1;
        while (left < right) {
            int sum = numbers[left] + numbers[right];
            if (sum == target) {
                return new int[]{left + 1, right + 1};
            } else if (sum < target) {
                left++;
            } else {
                right--;
            }
        }
        return new int[]{-1, -1};
    }

    public static void main(String[] args) {
        P27_TwoSumII_InputArrayIsSorted sol = new P27_TwoSumII_InputArrayIsSorted();
        test(sol, new int[]{2, 7, 11, 15}, 9, new int[]{1, 2});
        test(sol, new int[]{2, 3, 4}, 6, new int[]{1, 3});
        test(sol, new int[]{-1, 0}, -1, new int[]{1, 2});
        test(sol, new int[]{1, 2, 3, 4, 4, 9, 56, 90}, 8, new int[]{4, 5});
        System.out.println("All tests passed.");
    }

    private static void test(P27_TwoSumII_InputArrayIsSorted sol, int[] numbers, int target, int[] expected) {
        int[] actual = sol.twoSum(numbers, target);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(numbers) + " target=" + target + " -> " + java.util.Arrays.toString(actual));
    }
}
