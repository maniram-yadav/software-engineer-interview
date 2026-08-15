//! LeetCode Top Interview 150 — #95 Snakes and Ladders (Medium)
//!
//! Given an n x n boustrophedon-numbered board with some snakes/ladders
//! (board[r][c] != -1 means a jump), return the minimum number of dice
//! rolls (1-6) to reach the last square, or -1. Solved with BFS over
//! square numbers 1..=n*n, converting each square to its (row, col) via
//! the standard boustrophedon formula.
//!
//! Example:
//!   Input: board = 6x6 grid with some ladder/snake destinations
//!   Output: 4

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let total = (n * n) as i32;

        let square_to_cell = |square: i32| -> (usize, usize) {
            let s0 = (square - 1) as usize;
            let row_from_bottom = s0 / n;
            let mut col = s0 % n;
            if row_from_bottom % 2 == 1 {
                col = n - 1 - col;
            }
            let row = n - 1 - row_from_bottom;
            (row, col)
        };

        let mut visited = vec![false; (total + 1) as usize];
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        visited[1] = true;
        queue.push_back((1, 0));

        while let Some((square, moves)) = queue.pop_front() {
            if square == total {
                return moves;
            }
            for next in (square + 1)..=(square + 6).min(total) {
                let (r, c) = square_to_cell(next);
                let dest = if board[r][c] != -1 { board[r][c] } else { next };
                if !visited[dest as usize] {
                    visited[dest as usize] = true;
                    queue.push_back((dest, moves + 1));
                }
            }
        }

        -1
    }
}

fn main() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    println!("{}", Solution::snakes_and_ladders(board));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let board = vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ];
        assert_eq!(Solution::snakes_and_ladders(board), 4);
    }

    #[test]
    fn example_2_no_jumps() {
        let board = vec![vec![-1, -1], vec![-1, 3]];
        assert_eq!(Solution::snakes_and_ladders(board), -1);
    }

    #[test]
    fn tiny_board_reachable_in_one_roll() {
        let board = vec![vec![-1, -1], vec![-1, -1]];
        assert_eq!(Solution::snakes_and_ladders(board), 1);
    }
}
