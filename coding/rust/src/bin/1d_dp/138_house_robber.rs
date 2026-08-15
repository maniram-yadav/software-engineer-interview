//! LeetCode Top Interview 150 — #138 House Robber (Medium)
//!
//! Given an array of non-negative integers representing money in houses
//! along a street, find the max amount you can rob without robbing two
//! adjacent houses. Solved with a rolling DP: at each house, either skip
//! it (keep `cur`) or rob it (`prev` + this house's value).
//!
//! Example:
//!   Input: nums = [1,2,3,1]
//!   Output: 4

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev, mut cur) = (0, 0);
        for n in nums {
            let next = cur.max(prev + n);
            prev = cur;
            cur = next;
        }
        cur
    }
}

fn main() {
    println!("{}", Solution::rob(vec![1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn single_house() {
        assert_eq!(Solution::rob(vec![5]), 5);
    }

    #[test]
    fn two_houses_picks_larger() {
        assert_eq!(Solution::rob(vec![2, 5]), 5);
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::rob(vec![]), 0);
    }
}
