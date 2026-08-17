/**
 * Grind 169 -- #310. Minimum Height Trees (Medium)
 *
 * Given a tree (connected, undirected, acyclic graph) with n nodes, return
 * all roots that produce minimum-height trees (the "centroids").
 *
 * Example:
 *   Input: n = 4, edges = [[1,0],[1,2],[1,3]]
 *   Output: [1]
 */
public class P310_MinimumHeightTrees {

    public java.util.List<Integer> findMinHeightTrees(int n, int[][] edges) {
        if (n == 1) return new java.util.ArrayList<>(java.util.List.of(0));

        java.util.List<java.util.Set<Integer>> adj = new java.util.ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new java.util.HashSet<>());
        for (int[] e : edges) {
            adj.get(e[0]).add(e[1]);
            adj.get(e[1]).add(e[0]);
        }

        java.util.List<Integer> leaves = new java.util.ArrayList<>();
        for (int i = 0; i < n; i++) {
            if (adj.get(i).size() == 1) leaves.add(i);
        }

        int remaining = n;
        while (remaining > 2) {
            remaining -= leaves.size();
            java.util.List<Integer> newLeaves = new java.util.ArrayList<>();
            for (int leaf : leaves) {
                int neighbor = adj.get(leaf).iterator().next();
                adj.get(neighbor).remove(leaf);
                if (adj.get(neighbor).size() == 1) newLeaves.add(neighbor);
            }
            leaves = newLeaves;
        }
        return leaves;
    }

    public static void main(String[] args) {
        P310_MinimumHeightTrees sol = new P310_MinimumHeightTrees();
        test(sol, 4, new int[][]{{1, 0}, {1, 2}, {1, 3}}, new int[]{1});
        test(sol, 6, new int[][]{{3, 0}, {3, 1}, {3, 2}, {3, 4}, {5, 4}}, new int[]{3, 4});
        test(sol, 1, new int[][]{}, new int[]{0});
        System.out.println("All tests passed.");
    }

    private static void test(P310_MinimumHeightTrees sol, int n, int[][] edges, int[] expected) {
        java.util.List<Integer> actual = sol.findMinHeightTrees(n, edges);
        java.util.List<Integer> sortedActual = new java.util.ArrayList<>(actual);
        java.util.Collections.sort(sortedActual);
        java.util.List<Integer> expectedList = new java.util.ArrayList<>();
        for (int e : expected) expectedList.add(e);
        if (!sortedActual.equals(expectedList)) {
            throw new AssertionError("Expected " + expectedList + " but got " + sortedActual);
        }
        System.out.println("PASS: n=" + n + " -> " + sortedActual);
    }
}
