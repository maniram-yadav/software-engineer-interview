//! LeetCode Top Interview 150 — #46 Contains Duplicate II (Easy)
//!
//! Given an array `nums` and an integer `k`, return true if there are two
//! distinct indices i, j such that nums[i] == nums[j] and |i - j| <= k.
//!
//! Example:
//!   Input: nums = [1,2,3,1], k = 3
//!   Output: true

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut last_index: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = last_index.get(&num) {
                if (i as i32 - j) <= k {
                    return true;
                }
            }
            last_index.insert(num, i as i32);
        }
        false
    }
}

fn main() {
    println!(
        "{}",
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
    }

    #[test]
    fn example_3_too_far_apart() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }

    #[test]
    fn no_duplicates() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 4], 3),
            false
        );
    }
}
