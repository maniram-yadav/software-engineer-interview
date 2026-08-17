/**
 * Grind 169 -- #252. Meeting Rooms (Easy, LeetCode Premium)
 *
 * Given an array of meeting time intervals, determine if a person could
 * attend all meetings (no overlaps).
 *
 * Example:
 *   Input: intervals = [[0,30],[5,10],[15,20]]
 *   Output: false
 */
public class P252_MeetingRooms {

    public boolean canAttendMeetings(int[][] intervals) {
        java.util.Arrays.sort(intervals, (a, b) -> a[0] - b[0]);
        for (int i = 1; i < intervals.length; i++) {
            if (intervals[i][0] < intervals[i - 1][1]) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        P252_MeetingRooms sol = new P252_MeetingRooms();
        test(sol, new int[][]{{0, 30}, {5, 10}, {15, 20}}, false);
        test(sol, new int[][]{{7, 10}, {2, 4}}, true);
        System.out.println("All tests passed.");
    }

    private static void test(P252_MeetingRooms sol, int[][] intervals, boolean expected) {
        boolean actual = sol.canAttendMeetings(intervals);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
