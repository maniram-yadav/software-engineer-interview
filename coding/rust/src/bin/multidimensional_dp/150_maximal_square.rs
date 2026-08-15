//! LeetCode Top Interview 150 — #150 Maximal Square (Medium)
//!
//! Given an m x n binary matrix filled with 0s and 1s, find the largest
//! square containing only 1s, and return its area. dp[r][c] is the side
//! length of the largest all-1s square with its bottom-right corner at
//! (r-1, c-1) in the original matrix; it's bounded by the smallest of
//! its three neighbors (up, left, up-left) plus one.
//!
//! Example:
//!   Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],
//!                     ["1","1","1","1","1"],["1","0","0","1","0"]]
//!   Output: 4

struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![vec![0i32; cols + 1]; rows + 1];
        let mut best = 0;

        for r in 1..=rows {
            for c in 1..=cols {
                if matrix[r - 1][c - 1] == '1' {
                    dp[r][c] = 1 + dp[r - 1][c].min(dp[r][c - 1]).min(dp[r - 1][c - 1]);
                    best = best.max(dp[r][c]);
                }
            }
        }

        best * best
    }
}

fn board_of(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.chars().collect()).collect()
}

fn main() {
    let matrix = board_of(&["10100", "10111", "11111", "10010"]);
    println!("{}", Solution::maximal_square(matrix));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let matrix = board_of(&["10100", "10111", "11111", "10010"]);
        assert_eq!(Solution::maximal_square(matrix), 4);
    }

    #[test]
    fn example_2_two_by_two() {
        let matrix = board_of(&["01", "11"]);
        assert_eq!(Solution::maximal_square(matrix), 1);
    }

    #[test]
    fn example_3_single_zero() {
        assert_eq!(Solution::maximal_square(board_of(&["0"])), 0);
    }

    #[test]
    fn single_one() {
        assert_eq!(Solution::maximal_square(board_of(&["1"])), 1);
    }

    #[test]
    fn all_ones_grid() {
        let matrix = board_of(&["111", "111", "111"]);
        assert_eq!(Solution::maximal_square(matrix), 9);
    }
}
