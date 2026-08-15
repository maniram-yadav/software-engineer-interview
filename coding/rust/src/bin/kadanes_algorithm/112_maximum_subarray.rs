//! LeetCode Top Interview 150 — #112 Maximum Subarray (Medium)
//!
//! Given an integer array nums, find the contiguous subarray (at least
//! one element) with the largest sum, and return that sum. The classic
//! Kadane's algorithm.
//!
//! Example:
//!   Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
//!   Output: 6

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best = nums[0];
        let mut cur = nums[0];
        for &n in &nums[1..] {
            cur = n.max(cur + n);
            best = best.max(cur);
        }
        best
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn example_2_single_element() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn example_3_all_negative() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn all_negative_picks_least_negative() {
        assert_eq!(Solution::max_sub_array(vec![-3, -1, -2]), -1);
    }
}
