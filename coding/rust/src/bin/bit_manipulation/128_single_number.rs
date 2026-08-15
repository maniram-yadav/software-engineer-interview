//! LeetCode Top Interview 150 — #128 Single Number (Easy)
//!
//! Given a non-empty array of integers where every element appears twice
//! except for one, find that single one, in O(n) time and O(1) space.
//! XOR-ing everything cancels out every paired value, leaving only the
//! unique one.
//!
//! Example:
//!   Input: nums = [4,1,2,1,2]
//!   Output: 4

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

fn main() {
    println!("{}", Solution::single_number(vec![4, 1, 2, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn example_3_single_element() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::single_number(vec![-1, -1, -2]), -2);
    }
}
