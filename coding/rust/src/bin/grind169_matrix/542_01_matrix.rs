//! Grind 169 — LeetCode #542 01 Matrix (Medium)
//!
//! Given an m x n binary matrix, return the distance to the nearest 0
//! for each cell. Solved with multi-source BFS starting from every 0
//! cell simultaneously.
//!
//! Example:
//!   Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
//!   Output: [[0,0,0],[0,1,0],[1,2,1]]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = mat.len();
        let cols = mat[0].len();
        let mut dist = vec![vec![-1i32; cols]; rows];
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        for r in 0..rows {
            for c in 0..cols {
                if mat[r][c] == 0 {
                    dist[r][c] = 0;
                    queue.push_back((r as i32, c as i32));
                }
            }
        }

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (rr, cc) = (rows as i32, cols as i32);
        while let Some((r, c)) = queue.pop_front() {
            for &(dr, dc) in &dirs {
                let (nr, nc) = (r + dr, c + dc);
                if nr >= 0 && nr < rr && nc >= 0 && nc < cc && dist[nr as usize][nc as usize] == -1
                {
                    dist[nr as usize][nc as usize] = dist[r as usize][c as usize] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }

        dist
    }
}

fn main() {
    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    println!("{:?}", Solution::update_matrix(mat));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        assert_eq!(
            Solution::update_matrix(mat),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }

    #[test]
    fn example_2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(
            Solution::update_matrix(mat),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn single_zero_cell() {
        assert_eq!(Solution::update_matrix(vec![vec![0]]), vec![vec![0]]);
    }

    #[test]
    fn all_zeros() {
        let mat = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(
            Solution::update_matrix(mat),
            vec![vec![0, 0], vec![0, 0]]
        );
    }
}
