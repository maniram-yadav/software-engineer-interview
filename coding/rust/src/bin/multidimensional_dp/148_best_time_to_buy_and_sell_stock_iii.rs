//! LeetCode Top Interview 150 — #148 Best Time to Buy and Sell Stock III
//! (Hard)
//!
//! Given stock prices, find the max profit with at most two transactions
//! (must sell before buying again). Tracks the best position after each
//! of: first buy, first sell, second buy, second sell.
//!
//! Example:
//!   Input: prices = [3,3,5,0,0,3,1,4]
//!   Output: 6

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy1 = i32::MIN;
        let mut sell1 = 0;
        let mut buy2 = i32::MIN;
        let mut sell2 = 0;

        for p in prices {
            buy1 = buy1.max(-p);
            sell1 = sell1.max(buy1 + p);
            buy2 = buy2.max(sell1 - p);
            sell2 = sell2.max(buy2 + p);
        }

        sell2
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]),
            6
        );
    }

    #[test]
    fn example_2_strictly_increasing() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn example_3_strictly_decreasing() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn empty_prices() {
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
