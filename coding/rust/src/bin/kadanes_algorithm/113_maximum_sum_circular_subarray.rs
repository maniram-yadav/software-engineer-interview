//! LeetCode Top Interview 150 — #113 Maximum Sum Circular Subarray (Medium)
//!
//! Given a circular integer array nums (end connects to start), find the
//! maximum possible sum of a non-empty contiguous subarray. Solved by
//! computing both the standard Kadane max and the Kadane min in one pass;
//! a wrap-around subarray's sum equals total - (minimum subarray sum).
//! If everything is negative, the wrap-around trick degenerates to an
//! empty subarray, so that case falls back to the plain max.
//!
//! Example:
//!   Input: nums = [5,-3,5]
//!   Output: 10

struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut max_cur = 0;
        let mut max_best = i32::MIN;
        let mut min_cur = 0;
        let mut min_best = i32::MAX;

        for &n in &nums {
            max_cur = n.max(max_cur + n);
            max_best = max_best.max(max_cur);
            min_cur = n.min(min_cur + n);
            min_best = min_best.min(min_cur);
        }

        if max_best < 0 {
            max_best
        } else {
            max_best.max(total - min_best)
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_subarray_sum_circular(vec![5, -3, 5])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]),
            3
        );
    }

    #[test]
    fn example_2_wraps_around() {
        assert_eq!(
            Solution::max_subarray_sum_circular(vec![5, -3, 5]),
            10
        );
    }

    #[test]
    fn example_3_all_negative() {
        assert_eq!(
            Solution::max_subarray_sum_circular(vec![-3, -2, -3]),
            -2
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![7]), 7);
    }
}
