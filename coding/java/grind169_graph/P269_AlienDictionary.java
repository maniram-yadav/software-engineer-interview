/**
 * Grind 169 -- #269. Alien Dictionary (Hard, LeetCode Premium)
 *
 * Given a list of words sorted lexicographically according to an unknown
 * alien language's rules, derive a valid character ordering of that alien
 * alphabet (topological sort).
 *
 * Example:
 *   Input: words = ["wrt","wrf","er","ett","rftt"]
 *   Output: "wertf"
 */
public class P269_AlienDictionary {

    public String alienOrder(String[] words) {
        java.util.Map<Character, java.util.Set<Character>> graph = new java.util.HashMap<>();
        java.util.Map<Character, Integer> indegree = new java.util.HashMap<>();
        for (String w : words) {
            for (char c : w.toCharArray()) {
                graph.putIfAbsent(c, new java.util.HashSet<>());
                indegree.putIfAbsent(c, 0);
            }
        }

        for (int i = 0; i < words.length - 1; i++) {
            String w1 = words[i], w2 = words[i + 1];
            int minLen = Math.min(w1.length(), w2.length());
            if (w1.length() > w2.length() && w1.startsWith(w2)) return "";
            for (int j = 0; j < minLen; j++) {
                char c1 = w1.charAt(j), c2 = w2.charAt(j);
                if (c1 != c2) {
                    if (graph.get(c1).add(c2)) indegree.merge(c2, 1, Integer::sum);
                    break;
                }
            }
        }

        java.util.Queue<Character> queue = new java.util.LinkedList<>();
        for (char c : indegree.keySet()) {
            if (indegree.get(c) == 0) queue.add(c);
        }

        StringBuilder sb = new StringBuilder();
        while (!queue.isEmpty()) {
            char c = queue.poll();
            sb.append(c);
            for (char next : graph.get(c)) {
                indegree.merge(next, -1, Integer::sum);
                if (indegree.get(next) == 0) queue.add(next);
            }
        }
        return sb.length() == indegree.size() ? sb.toString() : "";
    }

    public static void main(String[] args) {
        P269_AlienDictionary sol = new P269_AlienDictionary();

        String result1 = sol.alienOrder(new String[]{"wrt", "wrf", "er", "ett", "rftt"});
        checkValidOrder(new String[]{"wrt", "wrf", "er", "ett", "rftt"}, result1, 5);
        System.out.println("PASS: \"wertf\" chain -> \"" + result1 + "\"");

        String result2 = sol.alienOrder(new String[]{"z", "x"});
        checkValidOrder(new String[]{"z", "x"}, result2, 2);
        System.out.println("PASS: simple order -> \"" + result2 + "\"");

        String result3 = sol.alienOrder(new String[]{"z", "x", "z"});
        if (!result3.isEmpty()) {
            throw new AssertionError("Expected \"\" for cyclic constraint but got \"" + result3 + "\"");
        }
        System.out.println("PASS: cyclic constraint -> \"\"");

        System.out.println("All tests passed.");
    }

    private static void checkValidOrder(String[] words, String order, int expectedLength) {
        if (order.length() != expectedLength) {
            throw new AssertionError("Expected order length " + expectedLength + " but got \"" + order + "\"");
        }
        java.util.Map<Character, Integer> position = new java.util.HashMap<>();
        for (int i = 0; i < order.length(); i++) position.put(order.charAt(i), i);

        for (int i = 0; i < words.length - 1; i++) {
            String w1 = words[i], w2 = words[i + 1];
            int minLen = Math.min(w1.length(), w2.length());
            for (int j = 0; j < minLen; j++) {
                char c1 = w1.charAt(j), c2 = w2.charAt(j);
                if (c1 != c2) {
                    if (position.get(c1) >= position.get(c2)) {
                        throw new AssertionError("Order \"" + order + "\" violates constraint " + c1 + " < " + c2);
                    }
                    break;
                }
            }
        }
    }
}
