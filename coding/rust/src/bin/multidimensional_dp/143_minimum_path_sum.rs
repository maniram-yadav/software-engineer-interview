//! LeetCode Top Interview 150 — #143 Minimum Path Sum (Medium)
//!
//! Given an m x n grid filled with non-negative numbers, find a path
//! from top-left to bottom-right (moving only down or right) that
//! minimizes the sum of numbers along the path.
//!
//! Example:
//!   Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
//!   Output: 7

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut dp = grid.clone();

        for r in 0..rows {
            for c in 0..cols {
                if r == 0 && c == 0 {
                    continue;
                }
                let up = if r > 0 { dp[r - 1][c] } else { i32::MAX };
                let left = if c > 0 { dp[r][c - 1] } else { i32::MAX };
                dp[r][c] = grid[r][c] + up.min(left);
            }
        }

        dp[rows - 1][cols - 1]
    }
}

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    println!("{}", Solution::min_path_sum(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }

    #[test]
    fn example_2_single_row() {
        let grid = vec![vec![1, 2, 3]];
        assert_eq!(Solution::min_path_sum(grid), 6);
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::min_path_sum(vec![vec![5]]), 5);
    }

    #[test]
    fn single_column() {
        let grid = vec![vec![1], vec![2], vec![3]];
        assert_eq!(Solution::min_path_sum(grid), 6);
    }
}
