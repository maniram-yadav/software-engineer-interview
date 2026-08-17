/**
 * Grind 169 -- #278. First Bad Version (Easy)
 *
 * You have n versions and want to find the first bad one, given an API
 * isBadVersion(version). Minimize the number of calls.
 *
 * Example:
 *   Input: n = 5, bad = 4
 *   Output: 4
 */
public class P278_FirstBadVersion {

    private int badVersion;

    public int firstBadVersion(int n, int bad) {
        this.badVersion = bad;
        int left = 1, right = n;
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (isBadVersion(mid)) right = mid;
            else left = mid + 1;
        }
        return left;
    }

    private boolean isBadVersion(int version) {
        return version >= badVersion;
    }

    public static void main(String[] args) {
        P278_FirstBadVersion sol = new P278_FirstBadVersion();
        test(sol, 5, 4, 4);
        test(sol, 1, 1, 1);
        test(sol, 10, 1, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P278_FirstBadVersion sol, int n, int bad, int expected) {
        int actual = sol.firstBadVersion(n, bad);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: n=" + n + " bad=" + bad + " -> " + actual);
    }
}
