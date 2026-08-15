//! Grind 169 — LeetCode #994 Rotting Oranges (Medium)
//!
//! Given a grid where cells are empty (0), fresh orange (1), or rotten
//! orange (2), each minute a fresh orange adjacent to a rotten one
//! becomes rotten. Return the minimum minutes until no cell has a fresh
//! orange, or -1 if impossible. Solved with multi-source BFS starting
//! from all initially rotten oranges simultaneously.
//!
//! Example:
//!   Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
//!   Output: 4

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut fresh = 0;

        for r in 0..rows {
            for c in 0..cols {
                match grid[r as usize][c as usize] {
                    2 => queue.push_back((r, c)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }
        if fresh == 0 {
            return 0;
        }

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut minutes = -1;
        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let (r, c) = queue.pop_front().unwrap();
                for &(dr, dc) in &dirs {
                    let (nr, nc) = (r + dr, c + dc);
                    if nr >= 0
                        && nr < rows
                        && nc >= 0
                        && nc < cols
                        && grid[nr as usize][nc as usize] == 1
                    {
                        grid[nr as usize][nc as usize] = 2;
                        fresh -= 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
            minutes += 1;
        }

        if fresh == 0 { minutes.max(0) } else { -1 }
    }
}

fn main() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    println!("{}", Solution::oranges_rotting(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 4);
    }

    #[test]
    fn example_2_unreachable_orange() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(Solution::oranges_rotting(grid), -1);
    }

    #[test]
    fn example_3_no_fresh_oranges() {
        let grid = vec![vec![0, 2]];
        assert_eq!(Solution::oranges_rotting(grid), 0);
    }

    #[test]
    fn all_already_rotten() {
        let grid = vec![vec![2, 2], vec![2, 2]];
        assert_eq!(Solution::oranges_rotting(grid), 0);
    }
}
