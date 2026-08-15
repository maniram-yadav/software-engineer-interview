//! LeetCode Top Interview 150 — #38 Game of Life (Medium)
//!
//! Given an m x n board representing Conway's Game of Life (1 = live,
//! 0 = dead), compute the next state in place using the standard rules:
//! a live cell with 2-3 live neighbors survives; a dead cell with exactly
//! 3 live neighbors becomes alive; otherwise a cell dies or stays dead.
//! A snapshot of the original board is taken so mutations don't affect
//! neighbor counts mid-computation.
//!
//! Example:
//!   Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
//!   Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]

struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let rows = board.len() as i32;
        let cols = board[0].len() as i32;
        let dirs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let snapshot = board.clone();

        for r in 0..rows {
            for c in 0..cols {
                let mut live_neighbors = 0;
                for &(dr, dc) in dirs.iter() {
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr >= 0
                        && nr < rows
                        && nc >= 0
                        && nc < cols
                        && snapshot[nr as usize][nc as usize] == 1
                    {
                        live_neighbors += 1;
                    }
                }

                if snapshot[r as usize][c as usize] == 1 {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        board[r as usize][c as usize] = 0;
                    }
                } else if live_neighbors == 3 {
                    board[r as usize][c as usize] = 1;
                }
            }
        }
    }
}

fn main() {
    let mut board = vec![
        vec![0, 1, 0],
        vec![0, 0, 1],
        vec![1, 1, 1],
        vec![0, 0, 0],
    ];
    Solution::game_of_life(&mut board);
    println!("{:?}", board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut board = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];
        Solution::game_of_life(&mut board);
        assert_eq!(
            board,
            vec![
                vec![0, 0, 0],
                vec![1, 0, 1],
                vec![0, 1, 1],
                vec![0, 1, 0]
            ]
        );
    }

    #[test]
    fn example_2_two_by_two_block_stable() {
        let mut board = vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![1, 1], vec![1, 1]]);
    }

    #[test]
    fn all_dead_stays_dead() {
        let mut board = vec![vec![0, 0], vec![0, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![0, 0], vec![0, 0]]);
    }

    #[test]
    fn single_cell_dies() {
        let mut board = vec![vec![1]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![0]]);
    }
}
