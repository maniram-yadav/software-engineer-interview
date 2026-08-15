//! LeetCode Top Interview 150 — #129 Single Number II (Medium)
//!
//! Given an integer array where every element appears three times except
//! for one, find that single one, in O(n) time and O(1) space. `ones`
//! and `twos` track, per bit position, whether the running count of that
//! bit (mod 3) is 1 or 2; a bit set in both is impossible by
//! construction, so the pair acts as a base-3 counter per bit.
//!
//! Example:
//!   Input: nums = [2,2,3,2]
//!   Output: 3

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones: i32 = 0;
        let mut twos: i32 = 0;
        for num in nums {
            ones = (ones ^ num) & !twos;
            twos = (twos ^ num) & !ones;
        }
        ones
    }
}

fn main() {
    println!("{}", Solution::single_number(vec![2, 2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]),
            99
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::single_number(vec![42]), 42);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::single_number(vec![-2, -2, -2, 5]), 5);
    }
}
