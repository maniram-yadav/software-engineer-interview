/**
 * Grind 169 -- #735. Asteroid Collision (Medium)
 *
 * Given an array of asteroids (sign = direction, magnitude = size) moving
 * in a row, simulate collisions (larger survives, equal both explode) and
 * return the state after all collisions.
 *
 * Example:
 *   Input: asteroids = [5,10,-5]
 *   Output: [5,10]
 */
public class P735_AsteroidCollision {

    public int[] asteroidCollision(int[] asteroids) {
        java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        for (int a : asteroids) {
            boolean alive = true;
            while (alive && a < 0 && !stack.isEmpty() && stack.peek() > 0) {
                int top = stack.peek();
                if (top < -a) {
                    stack.pop();
                } else if (top == -a) {
                    stack.pop();
                    alive = false;
                } else {
                    alive = false;
                }
            }
            if (alive) stack.push(a);
        }

        int[] result = new int[stack.size()];
        for (int i = result.length - 1; i >= 0; i--) result[i] = stack.pop();
        return result;
    }

    public static void main(String[] args) {
        P735_AsteroidCollision sol = new P735_AsteroidCollision();
        test(sol, new int[]{5, 10, -5}, new int[]{5, 10});
        test(sol, new int[]{8, -8}, new int[]{});
        test(sol, new int[]{10, 2, -5}, new int[]{10});
        System.out.println("All tests passed.");
    }

    private static void test(P735_AsteroidCollision sol, int[] asteroids, int[] expected) {
        int[] actual = sol.asteroidCollision(asteroids);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(actual));
    }
}
