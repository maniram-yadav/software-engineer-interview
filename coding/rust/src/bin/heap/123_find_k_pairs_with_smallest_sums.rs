//! LeetCode Top Interview 150 — #123 Find K Pairs with Smallest Sums (Medium)
//!
//! Given two sorted integer arrays and integer k, return the k pairs
//! (u, v) with u from nums1 and v from nums2 that have the smallest
//! sums. Solved with a min-heap seeded with (nums1[i], nums2[0]) for
//! each i, expanding to (nums1[i], nums2[j+1]) each time a pair is
//! popped — this never misses a smaller pair since nums2 is sorted.
//!
//! Example:
//!   Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
//!   Output: [[1,2],[1,4],[1,6]]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        for i in 0..nums1.len().min(k as usize) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
        }

        let mut result = Vec::new();
        while result.len() < k as usize {
            let Reverse((_, i, j)) = match heap.pop() {
                Some(x) => x,
                None => break,
            };
            result.push(vec![nums1[i], nums2[j]]);
            if j + 1 < nums2.len() {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
    }

    #[test]
    fn example_2_with_duplicates() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );
    }

    #[test]
    fn k_larger_than_all_pairs() {
        let result = Solution::k_smallest_pairs(vec![1, 2], vec![3], 10);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn empty_input_returns_empty() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![], vec![1, 2], 3),
            Vec::<Vec<i32>>::new()
        );
    }
}
