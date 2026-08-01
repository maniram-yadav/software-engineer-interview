# DP Solutions — Part 6: Digit DP (Java)
### 3 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

---

## 1. Non-negative Integers without Consecutive Ones

**Problem:** Given a positive integer `n`, return the number of integers in the range `[0, n]` whose binary representation does NOT contain two consecutive 1's.

**Example:**
```
Input: n = 5
Output: 5
Explanation: In binary: 0="0", 1="1", 2="10", 3="11", 4="100", 5="101".
Excluding 3 ("11", which has consecutive ones), the valid ones are 
0, 1, 2, 4, 5 — a count of 5.
```

**Brute force:** check every integer from 0 to n, scan its binary representation for consecutive ones → O(n log n).
**Optimized:** binary digit DP from the most significant bit down, using a precomputed Fibonacci-like table `f[i]` = count of valid i-bit binary strings — when we encounter a 1-bit in `n`, we can freely fill everything below it with any valid (no-consecutive-ones) pattern, EXCEPT we must also handle the "both this bit and the previous placed bit are 1" case which invalidates further counting.
```java
class NonNegativeIntegersWithoutConsecutiveOnes {
    public int findIntegers(int n) {
        int[] f = new int[32];      // f[i] = count of i-bit valid (no-consecutive-1) binary strings
        f[0] = 1;
        f[1] = 2;
        for (int i = 2; i < 32; i++) f[i] = f[i - 1] + f[i - 2];

        int ans = 0, prevBit = 0;
        for (int i = 30; i >= 0; i--) {
            if (((n >> i) & 1) == 1) {
                ans += f[i]; // all numbers with a 0 here and anything valid below
                if (prevBit == 1) {
                    // n itself has consecutive ones here — everything counted so far
                    // already excludes n and beyond, so we stop early
                    ans--;
                    return ans + 1;
                }
                prevBit = 1;
            } else {
                prevBit = 0;
            }
        }
        return ans + 1; // +1 accounts for n itself (valid, since loop completed without early exit)
    }
}
```
**Complexity:** O(log n) time (32-bit scan), O(1) space (fixed 32-entry table) — a massive improvement over the O(n log n) brute force.

---

## 2. Numbers At Most N Given Digit Set

**Problem:** Given a sorted array of distinct single-digit strings `digits` and a positive integer `n`, return how many positive integers ≤ n can be built using only digits from the array (digits may be reused any number of times).

**Example:**
```
Input: digits = ["1","3","5","7"], n = 100
Output: 20
Explanation: The 20 valid numbers are: 1,3,5,7 (1-digit, 4 numbers),
11,13,15,17,31,33,35,37,51,53,55,57,71,73,75,77 (2-digit, 16 numbers).
No 3-digit number ≤ 100 can be formed (smallest would be 111 > 100).
```

**Brute force:** generate every combination of digits up to `len(str(n))` digits, check ≤ n → exponential in digit count.
**Optimized:** classic digit DP — count all numbers with FEWER digits than n freely (each digit position has `|digits|` choices), then digit-by-digit match numbers with the SAME digit-count as n, using strictly-smaller digit choices at each tight position.
```java
class NumbersAtMostNGivenDigitSet {
    public int atMostNGivenDigitSet(String[] digits, int n) {
        String s = String.valueOf(n);
        int len = s.length();
        int d = digits.length;

        int[] pow = new int[len + 1];
        pow[0] = 1;
        for (int i = 1; i <= len; i++) pow[i] = pow[i - 1] * d;

        int result = 0;
        // count all numbers with fewer digits than n (freely composed)
        for (int i = 1; i < len; i++) result += pow[i];

        // count numbers with the SAME digit-length as n, digit by digit
        boolean tight = true;
        for (int i = 0; i < len && tight; i++) {
            char c = s.charAt(i);
            boolean matchedEqual = false;
            for (String digitStr : digits) {
                char dc = digitStr.charAt(0);
                if (dc < c) {
                    result += pow[len - i - 1]; // free choice for all remaining positions
                } else if (dc == c) {
                    matchedEqual = true;
                }
            }
            if (!matchedEqual) {
                tight = false; // can't match this position exactly — no further exact matches possible
            } else if (i == len - 1) {
                result += 1; // n itself is exactly formable
            }
        }
        return result;
    }
}
```
**Complexity:** O(len(n) · |digits|) time, O(len(n)) space.

---

## 3. Numbers With Repeated Digits

**Problem:** Given a positive integer `n`, return the count of positive integers in `[1, n]` that have at least one repeated digit.

**Example:**
```
Input: n = 100
Output: 10
Explanation: The numbers with a repeated digit are: 11, 22, 33, 44, 55, 66, 77, 88, 
99, and 100 (which has two 0's). That's 10 numbers.
```

**Brute force:** check every number from 1 to n for repeated digits directly → O(n log n).
**Optimized insight:** it's much easier to count numbers WITHOUT repeated digits (unique-digit numbers), then subtract from n: `answer = n - countUniqueDigitNumbers(n)`. Counting unique-digit numbers uses digit DP with permutation counts.
```java
class NumbersWithRepeatedDigits {
    public int numDupDigitsAtMostN(int n) {
        String s = String.valueOf(n);
        int len = s.length();

        // perm[i][j] = number of ways to arrange j digits chosen from i available digits (P(i,j))
        int[][] perm = new int[11][11];
        for (int i = 0; i <= 10; i++) {
            perm[i][0] = 1;
            for (int j = 1; j <= i; j++) {
                perm[i][j] = perm[i][j - 1] * (i - j + 1);
            }
        }

        int totalUnique = 0;
        // count all unique-digit numbers with FEWER digits than n (no leading zero: 9 choices for first digit)
        for (int i = 1; i < len; i++) {
            totalUnique += 9 * perm[9][i - 1];
        }

        // count unique-digit numbers with the SAME digit-length as n, digit by digit
        boolean[] used = new boolean[10];
        boolean allMatched = true;
        for (int i = 0; i < len; i++) {
            int digit = s.charAt(i) - '0';
            int start = (i == 0) ? 1 : 0; // no leading zero on first digit
            for (int d = start; d < digit; d++) {
                if (used[d]) continue;
                totalUnique += perm[9 - i][len - i - 1];
            }
            if (used[digit]) { allMatched = false; break; }
            used[digit] = true;
            if (i == len - 1) totalUnique += 1; // n itself has all-unique digits
        }

        return n - totalUnique;
    }
}
```
**Complexity:** O(len(n)) time (with O(10) precompute for permutation table), O(1) space — a large improvement over O(n log n).

---

## 🎯 Part 6 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | Non-negative Integers w/o Consecutive Ones | O(log n) | O(1) |
| 2 | Numbers At Most N Given Digit Set | O(len(n)·\|digits\|) | O(len(n)) |
| 3 | Numbers With Repeated Digits | O(len(n)) | O(1) |

---

**Next: Part 7 — DP on Trees (8 problems).** Say "continue" to proceed, or name a category to jump to.
