//! LeetCode Top Interview 150 — #133 Factorial Trailing Zeroes (Medium)
//!
//! Given an integer n, return the number of trailing zeroes in n!, in
//! logarithmic time. Trailing zeroes come from factors of 10 = 2*5, and
//! factors of 2 are always more plentiful than factors of 5, so the
//! count of trailing zeroes equals the count of factors of 5 in n!.
//!
//! Example:
//!   Input: n = 5
//!   Output: 1

struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let n = n as i64;
        let mut count = 0i64;
        let mut divisor = 5i64;
        while divisor <= n {
            count += n / divisor;
            divisor *= 5;
        }
        count as i32
    }
}

fn main() {
    println!("{}", Solution::trailing_zeroes(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::trailing_zeroes(5), 1);
    }

    #[test]
    fn example_3_zero() {
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }

    #[test]
    fn multiple_of_twenty_five() {
        assert_eq!(Solution::trailing_zeroes(25), 6);
    }
}
