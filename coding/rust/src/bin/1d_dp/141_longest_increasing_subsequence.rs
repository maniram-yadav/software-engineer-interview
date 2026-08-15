//! LeetCode Top Interview 150 — #141 Longest Increasing Subsequence (Medium)
//!
//! Given an integer array nums, return the length of the longest
//! strictly increasing subsequence. Solved with the O(n log n) patience
//! sorting technique: `tails[i]` is the smallest possible tail value of
//! an increasing subsequence of length i+1; each new number either
//! extends `tails` or replaces the first element >= it.
//!
//! Example:
//!   Input: nums = [10,9,2,5,3,7,101,18]
//!   Output: 4

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = Vec::new();
        for n in nums {
            let pos = tails.partition_point(|&x| x < n);
            if pos == tails.len() {
                tails.push(n);
            } else {
                tails[pos] = n;
            }
        }
        tails.len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn example_3_all_equal() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn strictly_increasing_input() {
        assert_eq!(Solution::length_of_lis(vec![1, 2, 3, 4, 5]), 5);
    }
}
