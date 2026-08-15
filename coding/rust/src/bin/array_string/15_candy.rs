//! LeetCode Top Interview 150 — #15 Candy (Hard)
//!
//! Children stand in a line, each with a rating. Give each child at least
//! one candy; any child with a higher rating than a neighbor must get more
//! candy than that neighbor. Return the minimum total candies needed.
//! Solved with two linear passes (left-to-right, then right-to-left).
//!
//! Example:
//!   Input: ratings = [1,0,2]
//!   Output: 5

struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        if n == 0 {
            return 0;
        }
        let mut candies = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}

fn main() {
    println!("min candies: {}", Solution::candy(vec![1, 0, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }

    #[test]
    fn single_child() {
        assert_eq!(Solution::candy(vec![5]), 1);
    }

    #[test]
    fn strictly_increasing() {
        assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn strictly_decreasing() {
        assert_eq!(Solution::candy(vec![5, 4, 3, 2, 1]), 15);
    }
}
