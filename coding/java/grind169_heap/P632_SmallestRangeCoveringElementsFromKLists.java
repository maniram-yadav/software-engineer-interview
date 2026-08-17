/**
 * Grind 169 -- #632. Smallest Range Covering Elements from K Lists (Hard)
 *
 * Given k sorted integer lists, find the smallest range [a, b] that
 * includes at least one number from each list.
 *
 * Example:
 *   Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
 *   Output: [20,24]
 */
public class P632_SmallestRangeCoveringElementsFromKLists {

    public int[] smallestRange(java.util.List<java.util.List<Integer>> nums) {
        java.util.PriorityQueue<int[]> heap = new java.util.PriorityQueue<>((a, b) -> a[0] - b[0]);
        int maxVal = Integer.MIN_VALUE;
        for (int i = 0; i < nums.size(); i++) {
            int val = nums.get(i).get(0);
            heap.add(new int[]{val, i, 0});
            maxVal = Math.max(maxVal, val);
        }

        int[] best = {heap.peek()[0], maxVal};
        while (heap.size() == nums.size()) {
            int[] cur = heap.poll();
            int listIdx = cur[1], elemIdx = cur[2];
            if (maxVal - cur[0] < best[1] - best[0]) {
                best = new int[]{cur[0], maxVal};
            }
            if (elemIdx + 1 < nums.get(listIdx).size()) {
                int nextVal = nums.get(listIdx).get(elemIdx + 1);
                heap.add(new int[]{nextVal, listIdx, elemIdx + 1});
                maxVal = Math.max(maxVal, nextVal);
            }
        }
        return best;
    }

    public static void main(String[] args) {
        P632_SmallestRangeCoveringElementsFromKLists sol = new P632_SmallestRangeCoveringElementsFromKLists();
        test(sol, java.util.List.of(
                java.util.List.of(4, 10, 15, 24, 26),
                java.util.List.of(0, 9, 12, 20),
                java.util.List.of(5, 18, 22, 30)), new int[]{20, 24});
        test(sol, java.util.List.of(
                java.util.List.of(1, 2, 3),
                java.util.List.of(1, 2, 3),
                java.util.List.of(1, 2, 3)), new int[]{1, 1});
        System.out.println("All tests passed.");
    }

    private static void test(P632_SmallestRangeCoveringElementsFromKLists sol, java.util.List<java.util.List<Integer>> nums, int[] expected) {
        int[] actual = sol.smallestRange(nums);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(actual));
    }
}
