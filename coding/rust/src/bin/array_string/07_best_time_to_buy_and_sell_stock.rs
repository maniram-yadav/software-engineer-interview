//! LeetCode Top Interview 150 — #7 Best Time to Buy and Sell Stock (Easy)
//!
//! Given an array `prices` where `prices[i]` is the stock price on day i,
//! find the max profit from a single buy followed by a single sell (buy
//! must happen before sell). Return 0 if no profit is possible.
//!
//! Example:
//!   Input: prices = [7,1,5,3,6,4]
//!   Output: 5   (buy at 1, sell at 6)

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut best_profit = 0;
        for price in prices {
            if price < min_price {
                min_price = price;
            } else if price - min_price > best_profit {
                best_profit = price - min_price;
            }
        }
        best_profit
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
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn example_2_no_profit() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn single_day() {
        assert_eq!(Solution::max_profit(vec![5]), 0);
    }

    #[test]
    fn increasing_prices() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
}
