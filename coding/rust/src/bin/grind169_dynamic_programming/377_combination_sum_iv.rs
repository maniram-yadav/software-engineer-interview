//! Grind 169 — LeetCode #377 Combination Sum IV (Medium)
//!
//! Given an array of distinct positive integers and a target, return the
//! number of possible combinations (order matters, elements reusable)
//! that add up to target. dp[i] sums, over every number n <= i, the
//! count of ways to make (i - n), since each ordering is distinguished
//! by which number comes last.
//!
//! Example:
//!   Input: nums = [1,2,3], target = 4
//!   Output: 7

struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0i64; target + 1];
        dp[0] = 1;

        for i in 1..=target {
            for &n in &nums {
                let n = n as usize;
                if n <= i {
                    dp[i] += dp[i - n];
                }
            }
        }

        dp[target] as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::combination_sum4(vec![1, 2, 3], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }

    #[test]
    fn example_2_no_combination() {
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
    }

    #[test]
    fn target_zero_has_one_empty_combination() {
        assert_eq!(Solution::combination_sum4(vec![1, 2], 0), 1);
    }

    #[test]
    fn single_number_exact_match() {
        assert_eq!(Solution::combination_sum4(vec![5], 5), 1);
    }
}
