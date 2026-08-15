//! Grind 169 — LeetCode #7 Reverse Integer (Medium)
//!
//! Given a 32-bit signed integer x, return x with its digits reversed;
//! return 0 if the reversed value overflows a 32-bit signed integer.
//! Accumulates in i64 to detect overflow before casting back to i32.
//!
//! Example:
//!   Input: x = 123
//!   Output: 321

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut n = x as i64;
        let mut result: i64 = 0;
        while n != 0 {
            result = result * 10 + n % 10;
            n /= 10;
        }
        if result < i32::MIN as i64 || result > i32::MAX as i64 {
            0
        } else {
            result as i32
        }
    }
}

fn main() {
    println!("{}", Solution::reverse(123));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn example_2_negative() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn example_3_trailing_zero_dropped() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn overflow_returns_zero() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn zero_reverses_to_zero() {
        assert_eq!(Solution::reverse(0), 0);
    }
}
