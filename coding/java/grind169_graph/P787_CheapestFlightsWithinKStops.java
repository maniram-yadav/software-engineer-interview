/**
 * Grind 169 -- #787. Cheapest Flights Within K Stops (Medium)
 *
 * Given n cities connected by flights with costs, find the cheapest price
 * from src to dst using at most k stops, or -1 if no such route exists.
 *
 * Example:
 *   Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
 *   Output: 700
 */
public class P787_CheapestFlightsWithinKStops {

    public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
        int[] dist = new int[n];
        java.util.Arrays.fill(dist, Integer.MAX_VALUE);
        dist[src] = 0;

        for (int i = 0; i <= k; i++) {
            int[] temp = dist.clone();
            for (int[] f : flights) {
                int u = f[0], v = f[1], w = f[2];
                if (dist[u] != Integer.MAX_VALUE && dist[u] + w < temp[v]) {
                    temp[v] = dist[u] + w;
                }
            }
            dist = temp;
        }
        return dist[dst] == Integer.MAX_VALUE ? -1 : dist[dst];
    }

    public static void main(String[] args) {
        P787_CheapestFlightsWithinKStops sol = new P787_CheapestFlightsWithinKStops();
        test(sol, 4, new int[][]{{0, 1, 100}, {1, 2, 100}, {2, 0, 100}, {1, 3, 600}, {2, 3, 200}}, 0, 3, 1, 700);
        test(sol, 3, new int[][]{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}}, 0, 2, 1, 200);
        test(sol, 3, new int[][]{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}}, 0, 2, 0, 500);
        System.out.println("All tests passed.");
    }

    private static void test(P787_CheapestFlightsWithinKStops sol, int n, int[][] flights, int src, int dst, int k, int expected) {
        int actual = sol.findCheapestPrice(n, flights, src, dst, k);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: src=" + src + " dst=" + dst + " k=" + k + " -> " + actual);
    }
}
