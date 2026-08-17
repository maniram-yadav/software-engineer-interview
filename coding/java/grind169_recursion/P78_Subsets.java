/**
 * Grind 169 -- #78. Subsets (Medium)
 *
 * Given an integer array of unique elements, return all possible subsets
 * (the power set).
 *
 * Example:
 *   Input: nums = [1,2,3]
 *   Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 */
public class P78_Subsets {

    public java.util.List<java.util.List<Integer>> subsets(int[] nums) {
        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        backtrack(nums, 0, new java.util.ArrayList<>(), result);
        return result;
    }

    private void backtrack(int[] nums, int start, java.util.List<Integer> current, java.util.List<java.util.List<Integer>> result) {
        result.add(new java.util.ArrayList<>(current));
        for (int i = start; i < nums.length; i++) {
            current.add(nums[i]);
            backtrack(nums, i + 1, current, result);
            current.remove(current.size() - 1);
        }
    }

    public static void main(String[] args) {
        P78_Subsets sol = new P78_Subsets();
        test(sol, new int[]{1, 2, 3}, 8);
        test(sol, new int[]{0}, 2);
        System.out.println("All tests passed.");
    }

    private static void test(P78_Subsets sol, int[] nums, int expectedCount) {
        java.util.List<java.util.List<Integer>> actual = sol.subsets(nums);
        if (actual.size() != expectedCount) {
            throw new AssertionError("Expected " + expectedCount + " subsets but got " + actual.size());
        }
        java.util.Set<java.util.Set<Integer>> distinct = new java.util.HashSet<>();
        for (java.util.List<Integer> s : actual) distinct.add(new java.util.HashSet<>(s));
        if (distinct.size() != expectedCount) {
            throw new AssertionError("Subsets are not all distinct: " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
