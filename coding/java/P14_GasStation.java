/**
 * LeetCode Top Interview 150 -- #14. Gas Station (Medium)
 *
 * There are n gas stations in a circle. gas[i] is the fuel available at
 * station i, cost[i] is the fuel to travel from station i to i+1. Return
 * the starting station index from which you can complete the circuit, or
 * -1 if impossible (the answer is unique if it exists).
 *
 * Example:
 *   Input: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
 *   Output: 3
 */
public class P14_GasStation {

    public int canCompleteCircuit(int[] gas, int[] cost) {
        int total = 0, tank = 0, start = 0;
        for (int i = 0; i < gas.length; i++) {
            int diff = gas[i] - cost[i];
            total += diff;
            tank += diff;
            if (tank < 0) {
                start = i + 1;
                tank = 0;
            }
        }
        return total >= 0 ? start : -1;
    }

    public static void main(String[] args) {
        P14_GasStation sol = new P14_GasStation();
        test(sol, new int[]{1, 2, 3, 4, 5}, new int[]{3, 4, 5, 1, 2}, 3);
        test(sol, new int[]{2, 3, 4}, new int[]{3, 4, 3}, -1);
        test(sol, new int[]{5}, new int[]{4}, 0);
        test(sol, new int[]{3, 3, 4}, new int[]{3, 4, 4}, -1);
        System.out.println("All tests passed.");
    }

    private static void test(P14_GasStation sol, int[] gas, int[] cost, int expected) {
        int actual = sol.canCompleteCircuit(gas, cost);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: gas=" + java.util.Arrays.toString(gas) + " cost=" + java.util.Arrays.toString(cost) + " -> " + actual);
    }
}
