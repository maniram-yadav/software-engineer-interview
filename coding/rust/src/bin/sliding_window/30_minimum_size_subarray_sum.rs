//! LeetCode Top Interview 150 — #30 Minimum Size Subarray Sum (Medium)
//!
//! Given a positive integer array `nums` and a target, find the minimal
//! length of a contiguous subarray whose sum is >= target. Return 0 if
//! none exists. Solved with a growing/shrinking sliding window.
//!
//! Example:
//!   Input: target = 7, nums = [2,3,1,2,4,3]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut sum = 0;
        let mut best = usize::MAX;

        for right in 0..n {
            sum += nums[right];
            while sum >= target {
                best = best.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }

        if best == usize::MAX {
            0
        } else {
            best as i32
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]),
            2
        );
    }

    #[test]
    fn example_2_no_valid_subarray() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn example_3_impossible() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn whole_array_needed() {
        assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);
    }
}
