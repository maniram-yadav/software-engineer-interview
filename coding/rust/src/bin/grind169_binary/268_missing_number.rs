//! Grind 169 — LeetCode #268 Missing Number (Easy)
//!
//! Given an array nums containing n distinct numbers in range [0, n],
//! return the one number missing from the range. The sum of 0..=n minus
//! the actual sum of nums is exactly the missing value.
//!
//! Example:
//!   Input: nums = [3,0,1]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let expected = n * (n + 1) / 2;
        let actual: i32 = nums.iter().sum();
        expected - actual
    }
}

fn main() {
    println!("{}", Solution::missing_number(vec![3, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn example_2_missing_middle() {
        assert_eq!(
            Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }

    #[test]
    fn missing_zero() {
        assert_eq!(Solution::missing_number(vec![1]), 0);
    }

    #[test]
    fn missing_last() {
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }
}
