//! LeetCode Top Interview 150 — #8 Best Time to Buy and Sell Stock II (Medium)
//!
//! Given an array `prices`, you may complete as many buy/sell transactions
//! as you like (one share at a time; must sell before buying again).
//! Maximize total profit by summing every positive day-over-day gain.
//!
//! Example:
//!   Input: prices = [7,1,5,3,6,4]
//!   Output: 7   (buy@1 sell@5 = 4, buy@3 sell@6 = 3)

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }
        profit
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("max profit: {}", Solution::max_profit(prices));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn strictly_increasing() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn strictly_decreasing() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
