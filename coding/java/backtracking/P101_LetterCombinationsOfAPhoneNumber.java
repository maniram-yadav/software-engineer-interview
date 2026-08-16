/**
 * LeetCode Top Interview 150 -- #101. Letter Combinations of a Phone Number (Medium)
 *
 * Given a string of digits 2-9, return all possible letter combinations
 * the number could represent (standard phone keypad mapping).
 *
 * Example:
 *   Input: digits = "23"
 *   Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 */
public class P101_LetterCombinationsOfAPhoneNumber {

    private static final String[] MAPPING = {"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};

    public java.util.List<String> letterCombinations(String digits) {
        java.util.List<String> result = new java.util.ArrayList<>();
        if (digits.isEmpty()) return result;
        backtrack(digits, 0, new StringBuilder(), result);
        return result;
    }

    private void backtrack(String digits, int idx, StringBuilder current, java.util.List<String> result) {
        if (idx == digits.length()) {
            result.add(current.toString());
            return;
        }
        String letters = MAPPING[digits.charAt(idx) - '0'];
        for (char c : letters.toCharArray()) {
            current.append(c);
            backtrack(digits, idx + 1, current, result);
            current.deleteCharAt(current.length() - 1);
        }
    }

    public static void main(String[] args) {
        P101_LetterCombinationsOfAPhoneNumber sol = new P101_LetterCombinationsOfAPhoneNumber();
        test(sol, "23", new String[]{"ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"});
        test(sol, "", new String[]{});
        test(sol, "2", new String[]{"a", "b", "c"});
        System.out.println("All tests passed.");
    }

    private static void test(P101_LetterCombinationsOfAPhoneNumber sol, String digits, String[] expected) {
        java.util.List<String> actual = sol.letterCombinations(digits);
        java.util.List<String> expectedList = java.util.Arrays.asList(expected);
        if (!actual.equals(expectedList)) {
            throw new AssertionError("Expected " + expectedList + " but got " + actual);
        }
        System.out.println("PASS: \"" + digits + "\" -> " + actual);
    }
}
