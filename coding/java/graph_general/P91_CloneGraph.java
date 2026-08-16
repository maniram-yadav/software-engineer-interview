/**
 * LeetCode Top Interview 150 -- #91. Clone Graph (Medium)
 *
 * Given a reference to a node in a connected undirected graph, return a
 * deep copy of the graph.
 *
 * Example:
 *   Input: adjList = [[2,4],[1,3],[2,4],[1,3]]
 *   Output: deep-cloned graph with identical adjacency
 */
public class P91_CloneGraph {

    static class Node {
        int val;
        java.util.List<Node> neighbors;

        Node(int val) {
            this.val = val;
            neighbors = new java.util.ArrayList<>();
        }
    }

    public Node cloneGraph(Node node) {
        if (node == null) return null;
        return clone(node, new java.util.HashMap<>());
    }

    private Node clone(Node node, java.util.Map<Node, Node> map) {
        if (map.containsKey(node)) return map.get(node);
        Node copy = new Node(node.val);
        map.put(node, copy);
        for (Node neighbor : node.neighbors) {
            copy.neighbors.add(clone(neighbor, map));
        }
        return copy;
    }

    public static void main(String[] args) {
        P91_CloneGraph sol = new P91_CloneGraph();

        // Build graph 1-2-3-4-1 (a 4-cycle): adjList = [[2,4],[1,3],[2,4],[1,3]]
        Node[] nodes = new Node[5]; // 1-indexed
        for (int i = 1; i <= 4; i++) nodes[i] = new Node(i);
        nodes[1].neighbors.add(nodes[2]);
        nodes[1].neighbors.add(nodes[4]);
        nodes[2].neighbors.add(nodes[1]);
        nodes[2].neighbors.add(nodes[3]);
        nodes[3].neighbors.add(nodes[2]);
        nodes[3].neighbors.add(nodes[4]);
        nodes[4].neighbors.add(nodes[1]);
        nodes[4].neighbors.add(nodes[3]);

        Node clone = sol.cloneGraph(nodes[1]);
        test(nodes[1], clone);

        Node single = new Node(1);
        Node cloneSingle = sol.cloneGraph(single);
        if (cloneSingle == single || cloneSingle.val != 1 || !cloneSingle.neighbors.isEmpty()) {
            throw new AssertionError("Single node clone failed");
        }
        System.out.println("PASS: single node clone");

        if (sol.cloneGraph(null) != null) {
            throw new AssertionError("null clone should be null");
        }
        System.out.println("PASS: null clone");

        System.out.println("All tests passed.");
    }

    private static void test(Node original, Node clone) {
        java.util.Map<Integer, java.util.List<Integer>> origAdj = adjacency(original);
        java.util.Map<Integer, java.util.List<Integer>> cloneAdj = adjacency(clone);
        if (!origAdj.equals(cloneAdj)) {
            throw new AssertionError("Adjacency mismatch: expected " + origAdj + " but got " + cloneAdj);
        }
        java.util.Set<Node> origNodes = java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<>());
        collectNodes(original, origNodes, new java.util.HashSet<>());
        java.util.Set<Node> cloneNodes = java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<>());
        collectNodes(clone, cloneNodes, new java.util.HashSet<>());
        for (Node n : cloneNodes) {
            if (origNodes.contains(n)) {
                throw new AssertionError("Clone reused an original node instance");
            }
        }
        System.out.println("PASS: adjacency=" + origAdj);
    }

    private static java.util.Map<Integer, java.util.List<Integer>> adjacency(Node start) {
        java.util.Map<Integer, java.util.List<Integer>> map = new java.util.TreeMap<>();
        java.util.Set<Integer> visited = new java.util.HashSet<>();
        java.util.Deque<Node> stack = new java.util.ArrayDeque<>();
        stack.push(start);
        while (!stack.isEmpty()) {
            Node node = stack.pop();
            if (!visited.add(node.val)) continue;
            java.util.List<Integer> vals = new java.util.ArrayList<>();
            for (Node n : node.neighbors) vals.add(n.val);
            java.util.Collections.sort(vals);
            map.put(node.val, vals);
            for (Node n : node.neighbors) stack.push(n);
        }
        return map;
    }

    private static void collectNodes(Node node, java.util.Set<Node> nodes, java.util.Set<Integer> visitedVals) {
        if (!visitedVals.add(node.val)) return;
        nodes.add(node);
        for (Node n : node.neighbors) collectNodes(n, nodes, visitedVals);
    }
}
