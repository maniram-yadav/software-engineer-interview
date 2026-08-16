/**
 * LeetCode Top Interview 150 -- #102. Combinations (Medium)
 *
 * Given two integers n and k, return all possible combinations of k
 * numbers chosen from 1..n.
 *
 * Example:
 *   Input: n = 4, k = 2
 *   Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
 */
public class P102_Combinations {

    public java.util.List<java.util.List<Integer>> combine(int n, int k) {
        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        backtrack(n, k, 1, new java.util.ArrayList<>(), result);
        return result;
    }

    private void backtrack(int n, int k, int start, java.util.List<Integer> current, java.util.List<java.util.List<Integer>> result) {
        if (current.size() == k) {
            result.add(new java.util.ArrayList<>(current));
            return;
        }
        for (int i = start; i <= n; i++) {
            current.add(i);
            backtrack(n, k, i + 1, current, result);
            current.remove(current.size() - 1);
        }
    }

    public static void main(String[] args) {
        P102_Combinations sol = new P102_Combinations();
        test(sol, 4, 2, "[[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]");
        test(sol, 1, 1, "[[1]]");
        System.out.println("All tests passed.");
    }

    private static void test(P102_Combinations sol, int n, int k, String expected) {
        java.util.List<java.util.List<Integer>> actual = sol.combine(n, k);
        if (!actual.toString().equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: n=" + n + " k=" + k + " -> " + actual);
    }
}
