//! LeetCode Top Interview 150 — #47 Longest Consecutive Sequence (Medium)
//!
//! Given an unsorted array of integers, return the length of the longest
//! run of consecutive integers, in O(n) time. Solved with a HashSet: only
//! start counting a run from numbers that are the start of a run (i.e.
//! `num - 1` is not present).
//!
//! Example:
//!   Input: nums = [100,4,200,1,3,2]
//!   Output: 4

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut best = 0;

        for &num in &set {
            if !set.contains(&(num - 1)) {
                let mut length = 1;
                let mut cur = num;
                while set.contains(&(cur + 1)) {
                    cur += 1;
                    length += 1;
                }
                best = best.max(length);
            }
        }

        best
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
    }

    #[test]
    fn all_duplicates() {
        assert_eq!(Solution::longest_consecutive(vec![1, 1, 1, 1]), 1);
    }
}
