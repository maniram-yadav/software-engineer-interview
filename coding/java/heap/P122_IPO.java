/**
 * LeetCode Top Interview 150 -- #122. IPO (Hard)
 *
 * Given k projects (with profits and required capital), starting capital
 * w, choose at most k projects to maximize final capital (each finished
 * project's profit is added to capital, enabling more projects).
 *
 * Example:
 *   Input: k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
 *   Output: 4
 */
public class P122_IPO {

    public int findMaximizedCapital(int k, int w, int[] profits, int[] capital) {
        int n = profits.length;
        int[][] projects = new int[n][2];
        for (int i = 0; i < n; i++) {
            projects[i][0] = capital[i];
            projects[i][1] = profits[i];
        }
        java.util.Arrays.sort(projects, (a, b) -> a[0] - b[0]);

        java.util.PriorityQueue<Integer> maxHeap = new java.util.PriorityQueue<>(java.util.Collections.reverseOrder());
        int idx = 0;
        for (int i = 0; i < k; i++) {
            while (idx < n && projects[idx][0] <= w) {
                maxHeap.add(projects[idx][1]);
                idx++;
            }
            if (maxHeap.isEmpty()) break;
            w += maxHeap.poll();
        }
        return w;
    }

    public static void main(String[] args) {
        P122_IPO sol = new P122_IPO();
        test(sol, 2, 0, new int[]{1, 2, 3}, new int[]{0, 1, 1}, 4);
        test(sol, 3, 0, new int[]{1, 2, 3}, new int[]{0, 1, 2}, 6);
        test(sol, 1, 0, new int[]{1, 2, 3}, new int[]{1, 1, 2}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P122_IPO sol, int k, int w, int[] profits, int[] capital, int expected) {
        int actual = sol.findMaximizedCapital(k, w, profits, capital);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: k=" + k + " w=" + w + " -> " + actual);
    }
}
