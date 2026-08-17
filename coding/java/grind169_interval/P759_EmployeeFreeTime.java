/**
 * Grind 169 -- #759. Employee Free Time (Hard, LeetCode Premium)
 *
 * Given a list of schedules (each employee's list of non-overlapping
 * intervals, sorted), return the list of finite intervals representing
 * common, positive-length free time for all employees.
 *
 * Example:
 *   Input: schedule = [[[1,2],[5,6]],[[1,3]],[[4,10]]]
 *   Output: [[3,4]]
 */
public class P759_EmployeeFreeTime {

    static class Interval {
        int start, end;

        Interval(int start, int end) {
            this.start = start;
            this.end = end;
        }
    }

    public java.util.List<Interval> employeeFreeTime(java.util.List<java.util.List<Interval>> schedule) {
        java.util.List<Interval> all = new java.util.ArrayList<>();
        for (java.util.List<Interval> emp : schedule) all.addAll(emp);
        all.sort((a, b) -> a.start - b.start);

        java.util.List<Interval> result = new java.util.ArrayList<>();
        int end = all.get(0).end;
        for (int i = 1; i < all.size(); i++) {
            Interval cur = all.get(i);
            if (cur.start > end) {
                result.add(new Interval(end, cur.start));
            }
            end = Math.max(end, cur.end);
        }
        return result;
    }

    public static void main(String[] args) {
        P759_EmployeeFreeTime sol = new P759_EmployeeFreeTime();

        java.util.List<java.util.List<Interval>> schedule = java.util.List.of(
                java.util.List.of(new Interval(1, 2), new Interval(5, 6)),
                java.util.List.of(new Interval(1, 3)),
                java.util.List.of(new Interval(4, 10)));

        java.util.List<Interval> result = sol.employeeFreeTime(schedule);
        if (result.size() != 1 || result.get(0).start != 3 || result.get(0).end != 4) {
            throw new AssertionError("Expected [[3,4]] but got " + toString(result));
        }
        System.out.println("PASS: " + toString(result));
        System.out.println("All tests passed.");
    }

    private static String toString(java.util.List<Interval> intervals) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < intervals.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("[").append(intervals.get(i).start).append(",").append(intervals.get(i).end).append("]");
        }
        return sb.append("]").toString();
    }
}
