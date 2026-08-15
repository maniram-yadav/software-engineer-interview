//! Grind 169 — LeetCode #560 Subarray Sum Equals K (Medium)
//!
//! Given an integer array nums and an integer k, return the total number
//! of contiguous subarrays whose sum equals k. Solved with a running
//! prefix sum and a HashMap counting how many times each prefix sum has
//! occurred; a subarray [j+1..i] sums to k exactly when
//! prefixSum[i] - prefixSum[j] == k.
//!
//! Example:
//!   Input: nums = [1,1,1], k = 2
//!   Output: 2

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_counts: HashMap<i32, i32> = HashMap::new();
        prefix_counts.insert(0, 1);
        let mut sum = 0;
        let mut count = 0;

        for n in nums {
            sum += n;
            if let Some(&c) = prefix_counts.get(&(sum - k)) {
                count += c;
            }
            *prefix_counts.entry(sum).or_insert(0) += 1;
        }

        count
    }
}

fn main() {
    println!("{}", Solution::subarray_sum(vec![1, 1, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::subarray_sum(vec![1, -1, 0], 0), 3);
    }

    #[test]
    fn no_match() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 100), 0);
    }
}
