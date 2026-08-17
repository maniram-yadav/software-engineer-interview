/**
 * Grind 169 -- #323. Number of Connected Components in an Undirected Graph (Medium)
 *
 * Given n nodes and a list of undirected edges, return the number of
 * connected components.
 *
 * Example:
 *   Input: n = 5, edges = [[0,1],[1,2],[3,4]]
 *   Output: 2
 */
public class P323_NumberOfConnectedComponentsInAnUndirectedGraph {

    public int countComponents(int n, int[][] edges) {
        int[] parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        for (int[] e : edges) union(parent, e[0], e[1]);

        java.util.Set<Integer> roots = new java.util.HashSet<>();
        for (int i = 0; i < n; i++) roots.add(find(parent, i));
        return roots.size();
    }

    private int find(int[] parent, int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

    private void union(int[] parent, int a, int b) {
        int rootA = find(parent, a), rootB = find(parent, b);
        if (rootA != rootB) parent[rootA] = rootB;
    }

    public static void main(String[] args) {
        P323_NumberOfConnectedComponentsInAnUndirectedGraph sol = new P323_NumberOfConnectedComponentsInAnUndirectedGraph();
        test(sol, 5, new int[][]{{0, 1}, {1, 2}, {3, 4}}, 2);
        test(sol, 5, new int[][]{{0, 1}, {1, 2}, {2, 3}, {3, 4}}, 1);
        test(sol, 3, new int[][]{}, 3);
        System.out.println("All tests passed.");
    }

    private static void test(P323_NumberOfConnectedComponentsInAnUndirectedGraph sol, int n, int[][] edges, int expected) {
        int actual = sol.countComponents(n, edges);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: n=" + n + " -> " + actual);
    }
}
