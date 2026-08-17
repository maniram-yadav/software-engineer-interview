/**
 * Grind 169 -- #815. Bus Routes (Hard)
 *
 * Given bus routes (each a list of stops) and a source/target stop, return
 * the minimum number of buses needed to travel from source to target, or
 * -1.
 *
 * Example:
 *   Input: routes = [[1,2,7],[3,6,7]], source = 1, target = 6
 *   Output: 2
 */
public class P815_BusRoutes {

    public int numBusesToDestination(int[][] routes, int source, int target) {
        if (source == target) return 0;

        java.util.Map<Integer, java.util.List<Integer>> stopToRoutes = new java.util.HashMap<>();
        for (int i = 0; i < routes.length; i++) {
            for (int stop : routes[i]) {
                stopToRoutes.computeIfAbsent(stop, k -> new java.util.ArrayList<>()).add(i);
            }
        }

        java.util.Queue<Integer> queue = new java.util.LinkedList<>();
        java.util.Set<Integer> visitedStops = new java.util.HashSet<>();
        java.util.Set<Integer> visitedRoutes = new java.util.HashSet<>();
        queue.add(source);
        visitedStops.add(source);

        int buses = 0;
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                int stop = queue.poll();
                if (stop == target) return buses;
                for (int routeIdx : stopToRoutes.getOrDefault(stop, java.util.List.of())) {
                    if (visitedRoutes.contains(routeIdx)) continue;
                    visitedRoutes.add(routeIdx);
                    for (int nextStop : routes[routeIdx]) {
                        if (visitedStops.add(nextStop)) queue.add(nextStop);
                    }
                }
            }
            buses++;
        }
        return -1;
    }

    public static void main(String[] args) {
        P815_BusRoutes sol = new P815_BusRoutes();
        test(sol, new int[][]{{1, 2, 7}, {3, 6, 7}}, 1, 6, 2);
        test(sol, new int[][]{{7, 12}, {4, 5, 15}, {6}, {15, 19}, {9, 12, 13}}, 15, 12, -1);
        test(sol, new int[][]{{1, 2, 7}}, 1, 1, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P815_BusRoutes sol, int[][] routes, int source, int target, int expected) {
        int actual = sol.numBusesToDestination(routes, source, target);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: source=" + source + " target=" + target + " -> " + actual);
    }
}
