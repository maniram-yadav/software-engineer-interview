//! LeetCode Top Interview 150 — #144 Unique Paths II (Medium)
//!
//! Given an m x n grid with obstacles (marked 1), find the number of
//! unique paths from top-left to bottom-right moving only down or right.
//!
//! Example:
//!   Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let rows = obstacle_grid.len();
        let cols = obstacle_grid[0].len();
        let mut dp = vec![vec![0i64; cols]; rows];

        for r in 0..rows {
            for c in 0..cols {
                if obstacle_grid[r][c] == 1 {
                    dp[r][c] = 0;
                } else if r == 0 && c == 0 {
                    dp[r][c] = 1;
                } else {
                    let up = if r > 0 { dp[r - 1][c] } else { 0 };
                    let left = if c > 0 { dp[r][c - 1] } else { 0 };
                    dp[r][c] = up + left;
                }
            }
        }

        dp[rows - 1][cols - 1] as i32
    }
}

fn main() {
    let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    println!("{}", Solution::unique_paths_with_obstacles(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 2);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 1);
    }

    #[test]
    fn start_blocked_is_zero() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);
    }

    #[test]
    fn no_obstacles() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 2);
    }
}
