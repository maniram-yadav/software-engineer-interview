/**
 * LeetCode Top Interview 150 -- #104. Combination Sum (Medium)
 *
 * Given an array of distinct positive integers candidates and a target,
 * return all unique combinations where the chosen numbers (reusable,
 * unlimited) sum to target.
 *
 * Example:
 *   Input: candidates = [2,3,6,7], target = 7
 *   Output: [[2,2,3],[7]]
 */
public class P104_CombinationSum {

    public java.util.List<java.util.List<Integer>> combinationSum(int[] candidates, int target) {
        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        java.util.Arrays.sort(candidates);
        backtrack(candidates, target, 0, new java.util.ArrayList<>(), result);
        return result;
    }

    private void backtrack(int[] candidates, int remaining, int start, java.util.List<Integer> current, java.util.List<java.util.List<Integer>> result) {
        if (remaining == 0) {
            result.add(new java.util.ArrayList<>(current));
            return;
        }
        for (int i = start; i < candidates.length; i++) {
            if (candidates[i] > remaining) break;
            current.add(candidates[i]);
            backtrack(candidates, remaining - candidates[i], i, current, result);
            current.remove(current.size() - 1);
        }
    }

    public static void main(String[] args) {
        P104_CombinationSum sol = new P104_CombinationSum();
        test(sol, new int[]{2, 3, 6, 7}, 7, "[[2, 2, 3], [7]]");
        test(sol, new int[]{2, 3, 5}, 8, "[[2, 2, 2, 2], [2, 3, 3], [3, 5]]");
        System.out.println("All tests passed.");
    }

    private static void test(P104_CombinationSum sol, int[] candidates, int target, String expected) {
        java.util.List<java.util.List<Integer>> actual = sol.combinationSum(candidates, target);
        if (!actual.toString().equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(candidates) + " target=" + target + " -> " + actual);
    }
}
