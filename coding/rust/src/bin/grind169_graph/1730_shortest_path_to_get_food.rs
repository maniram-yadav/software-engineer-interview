//! Grind 169 — LeetCode #1730 Shortest Path to Get Food (Medium)
//!
//! Given a grid with your position (*), food cells (#), obstacles (X),
//! and empty cells (O), return the shortest path length to any food
//! cell, or -1 if unreachable. Plain BFS from the starting position.
//!
//! Example:
//!   Input: grid = [["X","X","X","X","X","X"],["X","*","O","O","O","X"],
//!                   ["X","O","O","#","O","X"],["X","X","X","X","X","X"]]
//!   Output: 3

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn get_food(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut start = (0, 0);
        'outer: for r in 0..rows {
            for c in 0..cols {
                if grid[r as usize][c as usize] == '*' {
                    start = (r, c);
                    break 'outer;
                }
            }
        }

        let mut visited = vec![vec![false; cols as usize]; rows as usize];
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
        queue.push_back((start.0, start.1, 0));
        visited[start.0 as usize][start.1 as usize] = true;
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((r, c, steps)) = queue.pop_front() {
            if grid[r as usize][c as usize] == '#' {
                return steps;
            }
            for &(dr, dc) in &dirs {
                let (nr, nc) = (r + dr, c + dc);
                if nr >= 0
                    && nr < rows
                    && nc >= 0
                    && nc < cols
                    && !visited[nr as usize][nc as usize]
                    && grid[nr as usize][nc as usize] != 'X'
                {
                    visited[nr as usize][nc as usize] = true;
                    queue.push_back((nr, nc, steps + 1));
                }
            }
        }
        -1
    }
}

fn grid_of(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.chars().collect()).collect()
}

fn main() {
    let grid = grid_of(&["XXXXXX", "X*OOOX", "XOO#OX", "XXXXXX"]);
    println!("{}", Solution::get_food(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = grid_of(&["XXXXXX", "X*OOOX", "XOO#OX", "XXXXXX"]);
        assert_eq!(Solution::get_food(grid), 3);
    }

    #[test]
    fn example_2_unreachable() {
        let grid = grid_of(&["X*X", "X#X"]);
        assert_eq!(Solution::get_food(grid), -1);
    }

    #[test]
    fn food_adjacent() {
        let grid = grid_of(&["*#"]);
        assert_eq!(Solution::get_food(grid), 1);
    }

    #[test]
    fn start_is_food_impossible_per_constraints_but_no_move_needed() {
        // Not a valid LC input (start != food), but exercises BFS with the
        // nearest reachable food one step away in a bigger open area.
        let grid = grid_of(&["*OO", "OO#"]);
        assert_eq!(Solution::get_food(grid), 3);
    }
}
