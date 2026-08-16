/**
 * LeetCode Top Interview 150 -- #93. Course Schedule (Medium)
 *
 * Given numCourses and prerequisite pairs [a, b] (must take b before a),
 * determine if it's possible to finish all courses (i.e., the prerequisite
 * graph is a DAG).
 *
 * Example:
 *   Input: numCourses = 2, prerequisites = [[1,0]]
 *   Output: true
 */
public class P93_CourseSchedule {

    public boolean canFinish(int numCourses, int[][] prerequisites) {
        java.util.List<java.util.List<Integer>> graph = new java.util.ArrayList<>();
        for (int i = 0; i < numCourses; i++) graph.add(new java.util.ArrayList<>());
        int[] indegree = new int[numCourses];
        for (int[] p : prerequisites) {
            graph.get(p[1]).add(p[0]);
            indegree[p[0]]++;
        }

        java.util.Queue<Integer> queue = new java.util.LinkedList<>();
        for (int i = 0; i < numCourses; i++) {
            if (indegree[i] == 0) queue.add(i);
        }

        int visited = 0;
        while (!queue.isEmpty()) {
            int course = queue.poll();
            visited++;
            for (int next : graph.get(course)) {
                if (--indegree[next] == 0) queue.add(next);
            }
        }
        return visited == numCourses;
    }

    public static void main(String[] args) {
        P93_CourseSchedule sol = new P93_CourseSchedule();
        test(sol, 2, new int[][]{{1, 0}}, true);
        test(sol, 2, new int[][]{{1, 0}, {0, 1}}, false);
        test(sol, 1, new int[][]{}, true);
        System.out.println("All tests passed.");
    }

    private static void test(P93_CourseSchedule sol, int numCourses, int[][] prerequisites, boolean expected) {
        boolean actual = sol.canFinish(numCourses, prerequisites);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
