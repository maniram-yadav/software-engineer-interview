/**
 * Grind 169 -- #621. Task Scheduler (Medium)
 *
 * Given a list of CPU tasks (letters) and a cooldown n between two same
 * tasks, return the minimum number of time units (including idle slots)
 * needed to finish all tasks.
 *
 * Example:
 *   Input: tasks = ["A","A","A","B","B","B"], n = 2
 *   Output: 8   ("A B idle A B idle A B")
 */
public class P621_TaskScheduler {

    public int leastInterval(char[] tasks, int n) {
        int[] counts = new int[26];
        for (char t : tasks) counts[t - 'A']++;
        java.util.Arrays.sort(counts);

        int maxCount = counts[25];
        int idleSlots = (maxCount - 1) * n;
        for (int i = 24; i >= 0 && counts[i] > 0; i--) {
            idleSlots -= Math.min(counts[i], maxCount - 1);
        }
        idleSlots = Math.max(0, idleSlots);
        return tasks.length + idleSlots;
    }

    public static void main(String[] args) {
        P621_TaskScheduler sol = new P621_TaskScheduler();
        test(sol, new char[]{'A', 'A', 'A', 'B', 'B', 'B'}, 2, 8);
        test(sol, new char[]{'A', 'A', 'A', 'B', 'B', 'B'}, 0, 6);
        System.out.println("All tests passed.");
    }

    private static void test(P621_TaskScheduler sol, char[] tasks, int n, int expected) {
        int actual = sol.leastInterval(tasks, n);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: n=" + n + " -> " + actual);
    }
}
