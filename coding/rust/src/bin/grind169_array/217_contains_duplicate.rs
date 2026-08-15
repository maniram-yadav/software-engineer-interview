//! Grind 169 — LeetCode #217 Contains Duplicate (Easy)
//!
//! Given an integer array nums, return true if any value appears at
//! least twice.
//!
//! Example:
//!   Input: nums = [1,2,3,1]
//!   Output: true

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        for n in nums {
            if !seen.insert(n) {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("{}", Solution::contains_duplicate(vec![1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn example_2_all_unique() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::contains_duplicate(vec![]), false);
    }
}
