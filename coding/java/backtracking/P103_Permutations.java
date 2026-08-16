/**
 * LeetCode Top Interview 150 -- #103. Permutations (Medium)
 *
 * Given an array of distinct integers, return all possible permutations.
 *
 * Example:
 *   Input: nums = [1,2,3]
 *   Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 */
public class P103_Permutations {

    public java.util.List<java.util.List<Integer>> permute(int[] nums) {
        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        backtrack(nums, new java.util.ArrayList<>(), new boolean[nums.length], result);
        return result;
    }

    private void backtrack(int[] nums, java.util.List<Integer> current, boolean[] used, java.util.List<java.util.List<Integer>> result) {
        if (current.size() == nums.length) {
            result.add(new java.util.ArrayList<>(current));
            return;
        }
        for (int i = 0; i < nums.length; i++) {
            if (used[i]) continue;
            used[i] = true;
            current.add(nums[i]);
            backtrack(nums, current, used, result);
            current.remove(current.size() - 1);
            used[i] = false;
        }
    }

    public static void main(String[] args) {
        P103_Permutations sol = new P103_Permutations();
        test(sol, new int[]{1, 2, 3}, "[[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]");
        test(sol, new int[]{0, 1}, "[[0, 1], [1, 0]]");
        System.out.println("All tests passed.");
    }

    private static void test(P103_Permutations sol, int[] nums, String expected) {
        java.util.List<java.util.List<Integer>> actual = sol.permute(nums);
        if (!actual.toString().equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
