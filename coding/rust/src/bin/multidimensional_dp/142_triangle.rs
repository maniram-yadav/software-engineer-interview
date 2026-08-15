//! LeetCode Top Interview 150 — #142 Triangle (Medium)
//!
//! Given a triangle array, return the minimum path sum from top to
//! bottom (each step moves to an adjacent number on the row below).
//! Solved bottom-up: start with the last row as-is, then fold each row
//! upward by adding the cheaper of the two children below.
//!
//! Example:
//!   Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
//!   Output: 11

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = triangle[n - 1].clone();
        for row in (0..n - 1).rev() {
            for col in 0..=row {
                dp[col] = triangle[row][col] + dp[col].min(dp[col + 1]);
            }
        }
        dp[0]
    }
}

fn main() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    println!("{}", Solution::minimum_total(triangle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle), 11);
    }

    #[test]
    fn example_2_single_element() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }

    #[test]
    fn two_rows() {
        let triangle = vec![vec![1], vec![2, 3]];
        assert_eq!(Solution::minimum_total(triangle), 3);
    }

    #[test]
    fn negative_values() {
        let triangle = vec![vec![-1], vec![2, 3], vec![1, -1, -3]];
        assert_eq!(Solution::minimum_total(triangle), -1);
    }
}
