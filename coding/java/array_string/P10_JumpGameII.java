/**
 * LeetCode Top Interview 150 -- #10. Jump Game II (Medium)
 *
 * Same setup as Jump Game, but return the minimum number of jumps needed
 * to reach the last index (a valid path is guaranteed).
 *
 * Example:
 *   Input: nums = [2,3,1,1,4]
 *   Output: 2   (jump 1 step from index 0 to 1, then 3 steps to the last index)
 */
public class P10_JumpGameII {

    public int jump(int[] nums) {
        int jumps = 0, curEnd = 0, farthest = 0;
        for (int i = 0; i < nums.length - 1; i++) {
            farthest = Math.max(farthest, i + nums[i]);
            if (i == curEnd) {
                jumps++;
                curEnd = farthest;
            }
        }
        return jumps;
    }

    public static void main(String[] args) {
        P10_JumpGameII sol = new P10_JumpGameII();
        test(sol, new int[]{2, 3, 1, 1, 4}, 2);
        test(sol, new int[]{2, 3, 0, 1, 4}, 2);
        test(sol, new int[]{1}, 0);
        test(sol, new int[]{1, 2, 3}, 2);
        System.out.println("All tests passed.");
    }

    private static void test(P10_JumpGameII sol, int[] nums, int expected) {
        int actual = sol.jump(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
