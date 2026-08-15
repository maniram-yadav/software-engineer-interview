//! Grind 169 — LeetCode #416 Partition Equal Subset Sum (Medium)
//!
//! Given a non-empty array of positive integers, determine if it can be
//! partitioned into two subsets with equal sum. Equivalent to a 0/1
//! knapsack: can some subset sum to total/2? dp[j] tracks whether sum j
//! is reachable, updated in reverse to avoid reusing an item twice.
//!
//! Example:
//!   Input: nums = [1,5,11,5]
//!   Output: true   ([1,5,5] and [11])

struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false;
        }
        let target = (total / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for n in nums {
            let n = n as usize;
            if n > target {
                continue;
            }
            for j in (n..=target).rev() {
                if dp[j - n] {
                    dp[j] = true;
                }
            }
        }

        dp[target]
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_partition(vec![1, 5, 11, 5])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    }

    #[test]
    fn example_2_odd_total() {
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::can_partition(vec![1]), false);
    }

    #[test]
    fn two_equal_elements() {
        assert_eq!(Solution::can_partition(vec![3, 3]), true);
    }
}
