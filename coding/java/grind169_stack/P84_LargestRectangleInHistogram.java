/**
 * Grind 169 -- #84. Largest Rectangle in Histogram (Hard)
 *
 * Given an array of bar heights of a histogram (width 1 each), return the
 * area of the largest rectangle that fits within it.
 *
 * Example:
 *   Input: heights = [2,1,5,6,2,3]
 *   Output: 10
 */
public class P84_LargestRectangleInHistogram {

    public int largestRectangleArea(int[] heights) {
        java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        int maxArea = 0;
        int n = heights.length;
        for (int i = 0; i <= n; i++) {
            int h = (i == n) ? 0 : heights[i];
            while (!stack.isEmpty() && heights[stack.peek()] > h) {
                int height = heights[stack.pop()];
                int width = stack.isEmpty() ? i : i - stack.peek() - 1;
                maxArea = Math.max(maxArea, height * width);
            }
            stack.push(i);
        }
        return maxArea;
    }

    public static void main(String[] args) {
        P84_LargestRectangleInHistogram sol = new P84_LargestRectangleInHistogram();
        test(sol, new int[]{2, 1, 5, 6, 2, 3}, 10);
        test(sol, new int[]{2, 4}, 4);
        System.out.println("All tests passed.");
    }

    private static void test(P84_LargestRectangleInHistogram sol, int[] heights, int expected) {
        int actual = sol.largestRectangleArea(heights);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(heights) + " -> " + actual);
    }
}
