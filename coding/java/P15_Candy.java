/**
 * LeetCode Top Interview 150 -- #15. Candy (Hard)
 *
 * Children stand in a line, each with a rating. Give each child at least
 * one candy; any child with a higher rating than a neighbor must get more
 * candy than that neighbor. Return the minimum total candies needed.
 *
 * Example:
 *   Input: ratings = [1,0,2]
 *   Output: 5   (candies = [2,1,2])
 */
public class P15_Candy {

    public int candy(int[] ratings) {
        int n = ratings.length;
        int[] candies = new int[n];
        java.util.Arrays.fill(candies, 1);
        for (int i = 1; i < n; i++) {
            if (ratings[i] > ratings[i - 1]) {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for (int i = n - 2; i >= 0; i--) {
            if (ratings[i] > ratings[i + 1]) {
                candies[i] = Math.max(candies[i], candies[i + 1] + 1);
            }
        }
        int total = 0;
        for (int c : candies) total += c;
        return total;
    }

    public static void main(String[] args) {
        P15_Candy sol = new P15_Candy();
        test(sol, new int[]{1, 0, 2}, 5);
        test(sol, new int[]{1, 2, 2}, 4);
        test(sol, new int[]{1, 3, 2, 2, 1}, 7);
        test(sol, new int[]{5}, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P15_Candy sol, int[] ratings, int expected) {
        int actual = sol.candy(ratings);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(ratings) + " -> " + actual);
    }
}
