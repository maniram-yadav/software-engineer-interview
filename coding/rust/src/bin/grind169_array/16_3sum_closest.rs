//! Grind 169 — LeetCode #16 3Sum Closest (Medium)
//!
//! Given an integer array nums and a target, find three integers whose
//! sum is closest to target and return that sum. Sort, then for each
//! fixed first element, two-pointer scan the rest.
//!
//! Example:
//!   Input: nums = [-1,2,1,-4], target = 1
//!   Output: 2   (-1 + 2 + 1 = 2)

struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut best = nums[0] + nums[1] + nums[2];

        for i in 0..n {
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (best - target).abs() {
                    best = sum;
                }
                if sum == target {
                    return sum;
                } else if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }

        best
    }
}

fn main() {
    println!(
        "{}",
        Solution::three_sum_closest(vec![-1, 2, 1, -4], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn example_2_exact_match() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn three_elements_only() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1], 0), 3);
    }

    #[test]
    fn negative_target() {
        assert_eq!(
            Solution::three_sum_closest(vec![1, 1, -1, -1, 3], -1),
            -1
        );
    }
}
