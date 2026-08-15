//! LeetCode Top Interview 150 — #121 Kth Largest Element in an Array (Medium)
//!
//! Given an integer array nums and integer k, return the k-th largest
//! element (k-th largest in sorted order, not the k-th distinct one).
//! Rust's `BinaryHeap` is a max-heap by default, so popping k times
//! yields it directly.
//!
//! Example:
//!   Input: nums = [3,2,1,5,6,4], k = 2
//!   Output: 5

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);
        let mut result = 0;
        for _ in 0..k {
            result = heap.pop().unwrap();
        }
        result
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn example_2_with_duplicates() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }

    #[test]
    fn k_equals_one_is_max() {
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3], 1), 3);
    }

    #[test]
    fn k_equals_length_is_min() {
        assert_eq!(Solution::find_kth_largest(vec![1, 2, 3], 3), 1);
    }
}
