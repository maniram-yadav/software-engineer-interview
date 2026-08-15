//! Grind 169 — LeetCode #632 Smallest Range Covering Elements from K
//! Lists (Hard)
//!
//! Given k sorted integer lists, find the smallest range [a, b] that
//! includes at least one number from each list. A min-heap holds one
//! "current" element per list; the range from the heap's min to the
//! running max of everything ever pushed is a candidate, minimized by
//! always advancing whichever list currently holds the smallest value.
//!
//! Example:
//!   Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
//!   Output: [20,24]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        let mut max_val = i32::MIN;
        for (i, list) in nums.iter().enumerate() {
            heap.push(Reverse((list[0], i, 0)));
            max_val = max_val.max(list[0]);
        }

        let mut best = [i32::MIN, i32::MAX];
        while heap.len() == nums.len() {
            let Reverse((min_val, list_idx, elem_idx)) = heap.pop().unwrap();
            if max_val - min_val < best[1] - best[0] {
                best = [min_val, max_val];
            }
            if elem_idx + 1 < nums[list_idx].len() {
                let next_val = nums[list_idx][elem_idx + 1];
                max_val = max_val.max(next_val);
                heap.push(Reverse((next_val, list_idx, elem_idx + 1)));
            }
        }

        best.to_vec()
    }
}

fn main() {
    let nums = vec![
        vec![4, 10, 15, 24, 26],
        vec![0, 9, 12, 20],
        vec![5, 18, 22, 30],
    ];
    println!("{:?}", Solution::smallest_range(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ];
        assert_eq!(Solution::smallest_range(nums), vec![20, 24]);
    }

    #[test]
    fn example_2_single_list() {
        assert_eq!(
            Solution::smallest_range(vec![vec![1, 2, 3]]),
            vec![1, 1]
        );
    }

    #[test]
    fn overlapping_lists() {
        // Multiple width-1 ranges qualify ([1,2], [3,4], [5,6]); the
        // algorithm reports the first one it finds, [1,2].
        let nums = vec![vec![1, 3, 5], vec![2, 4, 6]];
        assert_eq!(Solution::smallest_range(nums), vec![1, 2]);
    }

    #[test]
    fn identical_lists() {
        let nums = vec![vec![1, 2, 3], vec![1, 2, 3]];
        assert_eq!(Solution::smallest_range(nums), vec![1, 1]);
    }
}
