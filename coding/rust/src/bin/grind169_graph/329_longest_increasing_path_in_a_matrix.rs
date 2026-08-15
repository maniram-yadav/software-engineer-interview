//! Grind 169 — LeetCode #329 Longest Increasing Path in a Matrix (Hard)
//!
//! Given an m x n integer matrix, return the length of the longest
//! strictly increasing path (moving in any of 4 directions). Solved
//! with DFS plus memoization: the longest increasing path starting at
//! each cell only needs to be computed once.
//!
//! Example:
//!   Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
//!   Output: 4   (path 1->2->6->9)

struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len() as i32;
        let cols = matrix[0].len() as i32;
        let mut memo = vec![vec![0i32; cols as usize]; rows as usize];

        fn dfs(
            matrix: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<i32>>,
            r: i32,
            c: i32,
            rows: i32,
            cols: i32,
        ) -> i32 {
            if memo[r as usize][c as usize] != 0 {
                return memo[r as usize][c as usize];
            }
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut best = 1;
            for &(dr, dc) in &dirs {
                let (nr, nc) = (r + dr, c + dc);
                if nr >= 0
                    && nr < rows
                    && nc >= 0
                    && nc < cols
                    && matrix[nr as usize][nc as usize] > matrix[r as usize][c as usize]
                {
                    best = best.max(1 + dfs(matrix, memo, nr, nc, rows, cols));
                }
            }
            memo[r as usize][c as usize] = best;
            best
        }

        let mut result = 1;
        for r in 0..rows {
            for c in 0..cols {
                result = result.max(dfs(&matrix, &mut memo, r, c, rows, cols));
            }
        }
        result
    }
}

fn main() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    println!("{}", Solution::longest_increasing_path(matrix));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
    }

    #[test]
    fn example_2() {
        let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
    }

    #[test]
    fn all_equal_values() {
        let matrix = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 1);
    }
}
