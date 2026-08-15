//! LeetCode Top Interview 150 — #140 Coin Change (Medium)
//!
//! Given coin denominations and an amount, return the fewest number of
//! coins needed to make up that amount, or -1 if impossible. Classic
//! bottom-up unbounded knapsack: dp[a] is the fewest coins to make
//! amount a.
//!
//! Example:
//!   Input: coins = [1,2,5], amount = 11
//!   Output: 3

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;

        for i in 1..=amount {
            for &c in &coins {
                let c = c as usize;
                if c <= i && dp[i - c] != i32::MAX {
                    dp[i] = dp[i].min(dp[i - c] + 1);
                }
            }
        }

        if dp[amount] == i32::MAX {
            -1
        } else {
            dp[amount]
        }
    }
}

fn main() {
    println!("{}", Solution::coin_change(vec![1, 2, 5], 11));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn example_2_impossible() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn example_3_zero_amount() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }

    #[test]
    fn exact_single_coin() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 5), 1);
    }
}
