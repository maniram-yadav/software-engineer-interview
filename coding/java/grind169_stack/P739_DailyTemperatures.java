/**
 * Grind 169 -- #739. Daily Temperatures (Medium)
 *
 * Given an array of daily temperatures, return an array answer where
 * answer[i] is the number of days you'd have to wait for a warmer
 * temperature; 0 if none.
 *
 * Example:
 *   Input: temperatures = [73,74,75,71,69,72,76,73]
 *   Output: [1,1,4,2,1,1,0,0]
 */
public class P739_DailyTemperatures {

    public int[] dailyTemperatures(int[] temperatures) {
        int n = temperatures.length;
        int[] answer = new int[n];
        java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        for (int i = 0; i < n; i++) {
            while (!stack.isEmpty() && temperatures[i] > temperatures[stack.peek()]) {
                int idx = stack.pop();
                answer[idx] = i - idx;
            }
            stack.push(i);
        }
        return answer;
    }

    public static void main(String[] args) {
        P739_DailyTemperatures sol = new P739_DailyTemperatures();
        test(sol, new int[]{73, 74, 75, 71, 69, 72, 76, 73}, new int[]{1, 1, 4, 2, 1, 1, 0, 0});
        test(sol, new int[]{30, 40, 50, 60}, new int[]{1, 1, 1, 0});
        test(sol, new int[]{30, 60, 90}, new int[]{1, 1, 0});
        System.out.println("All tests passed.");
    }

    private static void test(P739_DailyTemperatures sol, int[] temperatures, int[] expected) {
        int[] actual = sol.dailyTemperatures(temperatures);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(actual));
    }
}
