/**
 * LeetCode Top Interview 150 -- #92. Evaluate Division (Medium)
 *
 * Given equations like a / b = 2.0 and query pairs, evaluate each query
 * using the equations as a weighted graph. Return -1.0 if undeterminable.
 *
 * Example:
 *   Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"]]
 *   Output: [6.0,0.5,-1.0]
 */
public class P92_EvaluateDivision {

    public double[] calcEquation(java.util.List<java.util.List<String>> equations, double[] values, java.util.List<java.util.List<String>> queries) {
        java.util.Map<String, java.util.Map<String, Double>> graph = new java.util.HashMap<>();
        for (int i = 0; i < equations.size(); i++) {
            String a = equations.get(i).get(0), b = equations.get(i).get(1);
            graph.computeIfAbsent(a, k -> new java.util.HashMap<>()).put(b, values[i]);
            graph.computeIfAbsent(b, k -> new java.util.HashMap<>()).put(a, 1.0 / values[i]);
        }

        double[] result = new double[queries.size()];
        for (int i = 0; i < queries.size(); i++) {
            String c = queries.get(i).get(0), d = queries.get(i).get(1);
            if (!graph.containsKey(c) || !graph.containsKey(d)) {
                result[i] = -1.0;
            } else {
                result[i] = dfs(graph, c, d, new java.util.HashSet<>());
            }
        }
        return result;
    }

    private double dfs(java.util.Map<String, java.util.Map<String, Double>> graph, String cur, String target, java.util.Set<String> visited) {
        if (cur.equals(target)) return 1.0;
        visited.add(cur);
        for (java.util.Map.Entry<String, Double> e : graph.get(cur).entrySet()) {
            if (!visited.contains(e.getKey())) {
                double sub = dfs(graph, e.getKey(), target, visited);
                if (sub != -1.0) return sub * e.getValue();
            }
        }
        return -1.0;
    }

    public static void main(String[] args) {
        P92_EvaluateDivision sol = new P92_EvaluateDivision();

        java.util.List<java.util.List<String>> equations = java.util.List.of(
                java.util.List.of("a", "b"), java.util.List.of("b", "c"));
        double[] values = {2.0, 3.0};
        java.util.List<java.util.List<String>> queries = java.util.List.of(
                java.util.List.of("a", "c"), java.util.List.of("b", "a"), java.util.List.of("a", "e"));
        test(sol, equations, values, queries, new double[]{6.0, 0.5, -1.0});

        java.util.List<java.util.List<String>> equations2 = java.util.List.of(java.util.List.of("x", "y"));
        double[] values2 = {5.0};
        java.util.List<java.util.List<String>> queries2 = java.util.List.of(
                java.util.List.of("x", "y"), java.util.List.of("y", "x"), java.util.List.of("x", "x"));
        test(sol, equations2, values2, queries2, new double[]{5.0, 0.2, 1.0});

        System.out.println("All tests passed.");
    }

    private static void test(P92_EvaluateDivision sol, java.util.List<java.util.List<String>> equations, double[] values,
                              java.util.List<java.util.List<String>> queries, double[] expected) {
        double[] actual = sol.calcEquation(equations, values, queries);
        for (int i = 0; i < expected.length; i++) {
            if (Math.abs(actual[i] - expected[i]) > 1e-9) {
                throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
            }
        }
        System.out.println("PASS: " + java.util.Arrays.toString(actual));
    }
}
