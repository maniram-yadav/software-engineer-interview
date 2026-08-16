/**
 * LeetCode Top Interview 150 -- #53. Simplify Path (Medium)
 *
 * Given an absolute Unix-style file path, simplify it to its canonical
 * form (resolve ".", "..", and redundant slashes).
 *
 * Example:
 *   Input: path = "/a/./b/../../c/"
 *   Output: "/c"
 */
public class P53_SimplifyPath {

    public String simplifyPath(String path) {
        java.util.Deque<String> stack = new java.util.ArrayDeque<>();
        for (String part : path.split("/")) {
            if (part.isEmpty() || part.equals(".")) continue;
            if (part.equals("..")) {
                if (!stack.isEmpty()) stack.pop();
            } else {
                stack.push(part);
            }
        }

        StringBuilder sb = new StringBuilder();
        for (java.util.Iterator<String> it = stack.descendingIterator(); it.hasNext(); ) {
            sb.append('/').append(it.next());
        }
        return sb.length() == 0 ? "/" : sb.toString();
    }

    public static void main(String[] args) {
        P53_SimplifyPath sol = new P53_SimplifyPath();
        test(sol, "/a/./b/../../c/", "/c");
        test(sol, "/../", "/");
        test(sol, "/home//foo/", "/home/foo");
        test(sol, "/a/../../b/../c//.//", "/c");
        System.out.println("All tests passed.");
    }

    private static void test(P53_SimplifyPath sol, String path, String expected) {
        String actual = sol.simplifyPath(path);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: \"" + path + "\" -> \"" + actual + "\"");
    }
}
