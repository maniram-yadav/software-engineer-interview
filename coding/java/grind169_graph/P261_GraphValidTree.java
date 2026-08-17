/**
 * Grind 169 -- #261. Graph Valid Tree (Medium)
 *
 * Given n nodes and a list of undirected edges, determine if these edges
 * form a valid tree (connected and acyclic).
 *
 * Example:
 *   Input: n = 5, edges = [[0,1],[0,2],[0,3],[1,4]]
 *   Output: true
 */
public class P261_GraphValidTree {

    public boolean validTree(int n, int[][] edges) {
        if (edges.length != n - 1) return false;

        java.util.List<java.util.List<Integer>> adj = new java.util.ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new java.util.ArrayList<>());
        for (int[] e : edges) {
            adj.get(e[0]).add(e[1]);
            adj.get(e[1]).add(e[0]);
        }

        java.util.Set<Integer> visited = new java.util.HashSet<>();
        java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        stack.push(0);
        visited.add(0);
        while (!stack.isEmpty()) {
            int node = stack.pop();
            for (int neighbor : adj.get(node)) {
                if (visited.add(neighbor)) stack.push(neighbor);
            }
        }
        return visited.size() == n;
    }

    public static void main(String[] args) {
        P261_GraphValidTree sol = new P261_GraphValidTree();
        test(sol, 5, new int[][]{{0, 1}, {0, 2}, {0, 3}, {1, 4}}, true);
        test(sol, 5, new int[][]{{0, 1}, {1, 2}, {2, 3}, {1, 3}, {1, 4}}, false);
        test(sol, 1, new int[][]{}, true);
        System.out.println("All tests passed.");
    }

    private static void test(P261_GraphValidTree sol, int n, int[][] edges, boolean expected) {
        boolean actual = sol.validTree(n, edges);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: n=" + n + " -> " + actual);
    }
}
