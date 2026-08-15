//! LeetCode Top Interview 150 — #131 Palindrome Number (Easy)
//!
//! Given an integer x, return true if it reads the same backward as
//! forward, without converting to a string. Negative numbers and
//! multiples of 10 (other than 0 itself) can never be palindromes.
//!
//! Example:
//!   Input: x = 121
//!   Output: true

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let original = x as i64;
        let mut n = x;
        let mut reverted: i64 = 0;
        while n > 0 {
            reverted = reverted * 10 + (n % 10) as i64;
            n /= 10;
        }
        reverted == original
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(121));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn example_2_negative() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn example_3_trailing_zero() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn zero_is_palindrome() {
        assert_eq!(Solution::is_palindrome(0), true);
    }

    #[test]
    fn single_digit_is_palindrome() {
        assert_eq!(Solution::is_palindrome(7), true);
    }
}
