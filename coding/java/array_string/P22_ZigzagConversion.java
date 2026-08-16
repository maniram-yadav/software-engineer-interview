/**
 * LeetCode Top Interview 150 -- #22. Zigzag Conversion (Medium)
 *
 * Write a string in a zigzag pattern on a given number of rows, then read
 * it row by row.
 *
 * Example:
 *   Input: s = "PAYPALISHIRING", numRows = 3
 *   Output: "PAHNAPLSIIGYIR"
 */
public class P22_ZigzagConversion {

    public String convert(String s, int numRows) {
        if (numRows == 1 || numRows >= s.length()) return s;

        StringBuilder[] rows = new StringBuilder[numRows];
        for (int i = 0; i < numRows; i++) rows[i] = new StringBuilder();

        int curRow = 0;
        boolean goingDown = false;
        for (char c : s.toCharArray()) {
            rows[curRow].append(c);
            if (curRow == 0 || curRow == numRows - 1) goingDown = !goingDown;
            curRow += goingDown ? 1 : -1;
        }

        StringBuilder result = new StringBuilder();
        for (StringBuilder row : rows) result.append(row);
        return result.toString();
    }

    public static void main(String[] args) {
        P22_ZigzagConversion sol = new P22_ZigzagConversion();
        test(sol, "PAYPALISHIRING", 3, "PAHNAPLSIIGYIR");
        test(sol, "PAYPALISHIRING", 4, "PINALSIGYAHRPI");
        test(sol, "A", 1, "A");
        test(sol, "AB", 1, "AB");
        System.out.println("All tests passed.");
    }

    private static void test(P22_ZigzagConversion sol, String s, int numRows, String expected) {
        String actual = sol.convert(s, numRows);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: \"" + s + "\" rows=" + numRows + " -> \"" + actual + "\"");
    }
}
