/**
 * LeetCode Top Interview 150 -- #16. Trapping Rain Water (Hard)
 *
 * Given n non-negative integers representing an elevation map where each
 * bar has width 1, compute how much water it can trap after raining.
 *
 * Example:
 *   Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
 *   Output: 6
 */
public class P16_TrappingRainWater {

    public int trap(int[] height) {
        if (height.length == 0) return 0;
        int left = 0, right = height.length - 1;
        int leftMax = 0, rightMax = 0, water = 0;
        while (left < right) {
            if (height[left] <= height[right]) {
                leftMax = Math.max(leftMax, height[left]);
                water += leftMax - height[left];
                left++;
            } else {
                rightMax = Math.max(rightMax, height[right]);
                water += rightMax - height[right];
                right--;
            }
        }
        return water;
    }

    public static void main(String[] args) {
        P16_TrappingRainWater sol = new P16_TrappingRainWater();
        test(sol, new int[]{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}, 6);
        test(sol, new int[]{4, 2, 0, 3, 2, 5}, 9);
        test(sol, new int[]{}, 0);
        test(sol, new int[]{1, 2}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P16_TrappingRainWater sol, int[] height, int expected) {
        int actual = sol.trap(height);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(height) + " -> " + actual);
    }
}
