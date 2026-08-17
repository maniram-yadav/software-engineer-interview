/**
 * Grind 169 -- #394. Decode String (Medium)
 *
 * Given an encoded string with the pattern k[encoded_string] (repeat
 * encoded_string k times), return the fully decoded string.
 *
 * Example:
 *   Input: s = "3[a]2[bc]"
 *   Output: "aaabcbc"
 */
public class P394_DecodeString {

    public String decodeString(String s) {
        java.util.Deque<Integer> countStack = new java.util.ArrayDeque<>();
        java.util.Deque<StringBuilder> stringStack = new java.util.ArrayDeque<>();
        StringBuilder current = new StringBuilder();
        int k = 0;

        for (char c : s.toCharArray()) {
            if (Character.isDigit(c)) {
                k = k * 10 + (c - '0');
            } else if (c == '[') {
                countStack.push(k);
                stringStack.push(current);
                current = new StringBuilder();
                k = 0;
            } else if (c == ']') {
                StringBuilder decoded = stringStack.pop();
                int count = countStack.pop();
                for (int i = 0; i < count; i++) decoded.append(current);
                current = decoded;
            } else {
                current.append(c);
            }
        }
        return current.toString();
    }

    public static void main(String[] args) {
        P394_DecodeString sol = new P394_DecodeString();
        test(sol, "3[a]2[bc]", "aaabcbc");
        test(sol, "3[a2[c]]", "accaccacc");
        test(sol, "2[abc]3[cd]ef", "abcabccdcdcdef");
        System.out.println("All tests passed.");
    }

    private static void test(P394_DecodeString sol, String s, String expected) {
        String actual = sol.decodeString(s);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: \"" + s + "\" -> \"" + actual + "\"");
    }
}
