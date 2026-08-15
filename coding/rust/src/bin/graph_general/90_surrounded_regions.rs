//! LeetCode Top Interview 150 — #90 Surrounded Regions (Medium)
//!
//! Given an m x n board of 'X' and 'O', capture (flip to 'X') all regions
//! of 'O' that are fully surrounded and not connected to the border.
//! Solved by first marking every 'O' reachable from a border cell (these
//! survive), then flipping remaining 'O's to 'X' and restoring marked
//! cells back to 'O'.
//!
//! Example:
//!   Input: board = [["X","X","X","X"],["X","O","O","X"],
//!                    ["X","X","O","X"],["X","O","X","X"]]
//!   Output: [["X","X","X","X"],["X","X","X","X"],
//!             ["X","X","X","X"],["X","O","X","X"]]

struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len() as i32;
        if rows == 0 {
            return;
        }
        let cols = board[0].len() as i32;

        fn mark(board: &mut Vec<Vec<char>>, r: i32, c: i32, rows: i32, cols: i32) {
            if r < 0 || r >= rows || c < 0 || c >= cols || board[r as usize][c as usize] != 'O' {
                return;
            }
            board[r as usize][c as usize] = '#';
            mark(board, r + 1, c, rows, cols);
            mark(board, r - 1, c, rows, cols);
            mark(board, r, c + 1, rows, cols);
            mark(board, r, c - 1, rows, cols);
        }

        for r in 0..rows {
            mark(board, r, 0, rows, cols);
            mark(board, r, cols - 1, rows, cols);
        }
        for c in 0..cols {
            mark(board, 0, c, rows, cols);
            mark(board, rows - 1, c, rows, cols);
        }

        for r in 0..rows as usize {
            for c in 0..cols as usize {
                match board[r][c] {
                    'O' => board[r][c] = 'X',
                    '#' => board[r][c] = 'O',
                    _ => {}
                }
            }
        }
    }
}

fn board_of(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.chars().collect()).collect()
}

fn main() {
    let mut board = board_of(&["XXXX", "XOOX", "XXOX", "XOXX"]);
    Solution::solve(&mut board);
    println!("{:?}", board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut board = board_of(&["XXXX", "XOOX", "XXOX", "XOXX"]);
        Solution::solve(&mut board);
        assert_eq!(board, board_of(&["XXXX", "XXXX", "XXXX", "XOXX"]));
    }

    #[test]
    fn example_2_single_cell() {
        let mut board = board_of(&["X"]);
        Solution::solve(&mut board);
        assert_eq!(board, board_of(&["X"]));
    }

    #[test]
    fn border_o_survives() {
        let mut board = board_of(&["OXX", "XXX", "XXO"]);
        Solution::solve(&mut board);
        assert_eq!(board, board_of(&["OXX", "XXX", "XXO"]));
    }

    #[test]
    fn fully_enclosed_o_flips() {
        let mut board = board_of(&["XXX", "XOX", "XXX"]);
        Solution::solve(&mut board);
        assert_eq!(board, board_of(&["XXX", "XXX", "XXX"]));
    }
}
