/**
 * LeetCode Top Interview 150 -- #94. Course Schedule II (Medium)
 *
 * Same setup as Course Schedule, but return a valid course order to finish
 * all courses, or an empty array if impossible.
 *
 * Example:
 *   Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
 *   Output: [0,1,2,3]
 */
public class P94_CourseScheduleII {

    public int[] findOrder(int numCourses, int[][] prerequisites) {
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

        int[] order = new int[numCourses];
        int idx = 0;
        while (!queue.isEmpty()) {
            int course = queue.poll();
            order[idx++] = course;
            for (int next : graph.get(course)) {
                if (--indegree[next] == 0) queue.add(next);
            }
        }
        return idx == numCourses ? order : new int[0];
    }

    public static void main(String[] args) {
        P94_CourseScheduleII sol = new P94_CourseScheduleII();
        test(sol, 4, new int[][]{{1, 0}, {2, 0}, {3, 1}, {3, 2}}, new int[]{0, 1, 2, 3});
        test(sol, 1, new int[][]{}, new int[]{0});
        test(sol, 2, new int[][]{{1, 0}, {0, 1}}, new int[]{});
        System.out.println("All tests passed.");
    }

    private static void test(P94_CourseScheduleII sol, int numCourses, int[][] prerequisites, int[] expected) {
        int[] actual = sol.findOrder(numCourses, prerequisites);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(actual));
    }
}
