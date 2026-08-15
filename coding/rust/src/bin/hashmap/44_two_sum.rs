//! LeetCode Top Interview 150 — #44 Two Sum (Easy)
//!
//! Given an array `nums` and an integer `target`, return the indices of
//! the two numbers that add up to `target`. Exactly one solution exists.
//!
//! Example:
//!   Input: nums = [2,7,11,15], target = 9
//!   Output: [0,1]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = seen.get(&(target - num)) {
                return vec![j, i as i32];
            }
            seen.insert(num, i as i32);
        }
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn example_3_duplicate_values() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::two_sum(vec![-3, 4, 3, 90], 0), vec![0, 2]);
    }
}
