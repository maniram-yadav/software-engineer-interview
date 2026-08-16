/**
 * LeetCode Top Interview 150 -- #28. Container With Most Water (Medium)
 *
 * Given n non-negative integers height[i] representing vertical lines at
 * position i, find two lines that together with the x-axis form a
 * container holding the most water.
 *
 * Example:
 *   Input: height = [1,8,6,2,5,4,8,3,7]
 *   Output: 49
 */
public class P28_ContainerWithMostWater {

    public int maxArea(int[] height) {
        int left = 0, right = height.length - 1, max = 0;
        while (left < right) {
            int h = Math.min(height[left], height[right]);
            max = Math.max(max, h * (right - left));
            if (height[left] < height[right]) {
                left++;
            } else {
                right--;
            }
        }
        return max;
    }

    public static void main(String[] args) {
        P28_ContainerWithMostWater sol = new P28_ContainerWithMostWater();
        test(sol, new int[]{1, 8, 6, 2, 5, 4, 8, 3, 7}, 49);
        test(sol, new int[]{1, 1}, 1);
        test(sol, new int[]{4, 3, 2, 1, 4}, 16);
        test(sol, new int[]{1, 2, 1}, 2);
        System.out.println("All tests passed.");
    }

    private static void test(P28_ContainerWithMostWater sol, int[] height, int expected) {
        int actual = sol.maxArea(height);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(height) + " -> " + actual);
    }
}
