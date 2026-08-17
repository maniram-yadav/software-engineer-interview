/**
 * Grind 169 -- #253. Meeting Rooms II (Medium, LeetCode Premium)
 *
 * Given an array of meeting time intervals, return the minimum number of
 * conference rooms required.
 *
 * Example:
 *   Input: intervals = [[0,30],[5,10],[15,20]]
 *   Output: 2
 */
public class P253_MeetingRoomsII {

    public int minMeetingRooms(int[][] intervals) {
        int n = intervals.length;
        int[] starts = new int[n], ends = new int[n];
        for (int i = 0; i < n; i++) {
            starts[i] = intervals[i][0];
            ends[i] = intervals[i][1];
        }
        java.util.Arrays.sort(starts);
        java.util.Arrays.sort(ends);

        int rooms = 0, maxRooms = 0, endPtr = 0;
        for (int i = 0; i < n; i++) {
            while (endPtr < n && ends[endPtr] <= starts[i]) {
                rooms--;
                endPtr++;
            }
            rooms++;
            maxRooms = Math.max(maxRooms, rooms);
        }
        return maxRooms;
    }

    public static void main(String[] args) {
        P253_MeetingRoomsII sol = new P253_MeetingRoomsII();
        test(sol, new int[][]{{0, 30}, {5, 10}, {15, 20}}, 2);
        test(sol, new int[][]{{7, 10}, {2, 4}}, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P253_MeetingRoomsII sol, int[][] intervals, int expected) {
        int actual = sol.minMeetingRooms(intervals);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
