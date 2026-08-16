/**
 * LeetCode Top Interview 150 -- #11. H-Index (Medium)
 *
 * Given an array citations where citations[i] is the number of citations
 * for the researcher's i-th paper, return the researcher's h-index -- the
 * max h such that at least h papers have >= h citations each.
 *
 * Example:
 *   Input: citations = [3,0,6,1,5]
 *   Output: 3   (3 papers have >= 3 citations each)
 */
public class P11_HIndex {

    public int hIndex(int[] citations) {
        int n = citations.length;
        int[] buckets = new int[n + 1];
        for (int c : citations) {
            buckets[Math.min(c, n)]++;
        }
        int total = 0;
        for (int h = n; h >= 0; h--) {
            total += buckets[h];
            if (total >= h) return h;
        }
        return 0;
    }

    public static void main(String[] args) {
        P11_HIndex sol = new P11_HIndex();
        test(sol, new int[]{3, 0, 6, 1, 5}, 3);
        test(sol, new int[]{1, 3, 1}, 1);
        test(sol, new int[]{0}, 0);
        test(sol, new int[]{100}, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P11_HIndex sol, int[] citations, int expected) {
        int actual = sol.hIndex(citations);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(citations) + " -> " + actual);
    }
}
