//! LeetCode Top Interview 150 — #130 Bitwise AND of Numbers Range (Medium)
//!
//! Given two integers left and right, return the bitwise AND of all
//! numbers in the range [left, right] inclusive. The result is the
//! common binary prefix of left and right (any bit that differs somewhere
//! in the range gets AND-ed to 0), found by right-shifting both until
//! they're equal, then shifting back.
//!
//! Example:
//!   Input: left = 5, right = 7
//!   Output: 4

struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut left = left;
        let mut right = right;
        let mut shift = 0;
        while left < right {
            left >>= 1;
            right >>= 1;
            shift += 1;
        }
        left << shift
    }
}

fn main() {
    println!("{}", Solution::range_bitwise_and(5, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::range_bitwise_and(0, 0), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
    }

    #[test]
    fn same_left_and_right() {
        assert_eq!(Solution::range_bitwise_and(9, 9), 9);
    }
}
