//! LeetCode Top Interview 150 — #149 Best Time to Buy and Sell Stock IV
//! (Hard)
//!
//! Same as Stock III, generalized to at most k transactions. `buy[i]`
//! and `sell[i]` track the best position after completing i buys / i
//! sells so far, updated in increasing order of i each day.
//!
//! Example:
//!   Input: k = 2, prices = [2,4,1]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        if prices.is_empty() || k == 0 {
            return 0;
        }
        let mut buy = vec![i32::MIN; k + 1];
        let mut sell = vec![0; k + 1];

        for p in prices {
            for i in 1..=k {
                buy[i] = buy[i].max(sell[i - 1] - p);
                sell[i] = sell[i].max(buy[i] + p);
            }
        }

        sell[k]
    }
}

fn main() {
    println!("{}", Solution::max_profit(2, vec![2, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]),
            7
        );
    }

    #[test]
    fn k_zero_is_zero_profit() {
        assert_eq!(Solution::max_profit(0, vec![1, 2, 3]), 0);
    }

    #[test]
    fn empty_prices() {
        assert_eq!(Solution::max_profit(2, vec![]), 0);
    }

    #[test]
    fn k_one_matches_single_transaction() {
        assert_eq!(Solution::max_profit(1, vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
