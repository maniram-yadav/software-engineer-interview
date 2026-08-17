/**
 * Grind 169 -- #528. Random Pick with Weight (Medium)
 *
 * Given an array w of positive weights, design a structure that picks an
 * index i with probability proportional to w[i].
 *
 * Example:
 *   Input: w = [1,3]
 *   Output: pickIndex() returns 0 with probability 1/4, 1 with probability 3/4
 */
public class P528_RandomPickWithWeight {

    static class Solution {
        private final int[] prefixSums;
        private final int total;
        private final java.util.Random random = new java.util.Random();

        public Solution(int[] w) {
            prefixSums = new int[w.length];
            int sum = 0;
            for (int i = 0; i < w.length; i++) {
                sum += w[i];
                prefixSums[i] = sum;
            }
            total = sum;
        }

        public int pickIndex() {
            int target = random.nextInt(total) + 1;
            int left = 0, right = prefixSums.length - 1;
            while (left < right) {
                int mid = left + (right - left) / 2;
                if (prefixSums[mid] < target) left = mid + 1;
                else right = mid;
            }
            return left;
        }
    }

    public static void main(String[] args) {
        Solution single = new Solution(new int[]{5});
        for (int i = 0; i < 100; i++) {
            if (single.pickIndex() != 0) {
                throw new AssertionError("Single-weight solution should always return index 0");
            }
        }
        System.out.println("PASS: single weight always returns index 0");

        Solution weighted = new Solution(new int[]{1, 3});
        int[] counts = new int[2];
        int trials = 100000;
        for (int i = 0; i < trials; i++) counts[weighted.pickIndex()]++;
        double ratio = (double) counts[1] / trials;
        if (ratio < 0.65 || ratio > 0.85) {
            throw new AssertionError("Expected index 1 ratio near 0.75 but got " + ratio);
        }
        System.out.println("PASS: weights [1,3] -> index 1 picked " + ratio + " of the time (expected ~0.75)");

        System.out.println("All tests passed.");
    }
}
