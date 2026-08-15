//! Grind 169 — LeetCode #37 Sudoku Solver (Hard)
//!
//! Write a program to solve a Sudoku puzzle by filling the empty cells
//! in place. Solved with classic backtracking: try each candidate digit
//! in the first empty cell, recurse, and undo on failure.
//!
//! Example:
//!   Input: board = partially filled 9x9 Sudoku grid
//!   Output: fully solved 9x9 Sudoku grid

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    for d in '1'..='9' {
                        if Self::is_valid(board, r, c, d) {
                            board[r][c] = d;
                            if Self::solve(board) {
                                return true;
                            }
                            board[r][c] = '.';
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, d: char) -> bool {
        for i in 0..9 {
            if board[row][i] == d {
                return false;
            }
            if board[i][col] == d {
                return false;
            }
            let box_row = 3 * (row / 3) + i / 3;
            let box_col = 3 * (col / 3) + i % 3;
            if board[box_row][box_col] == d {
                return false;
            }
        }
        true
    }
}

fn board_of(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.chars().collect()).collect()
}

// Checks every row, column, and 3x3 box is a permutation of 1-9.
fn is_solved_valid(board: &Vec<Vec<char>>) -> bool {
    let is_permutation = |cells: Vec<char>| -> bool {
        let mut seen = [false; 9];
        for c in cells {
            if c == '.' {
                return false;
            }
            let idx = (c as u8 - b'1') as usize;
            if idx > 8 || seen[idx] {
                return false;
            }
            seen[idx] = true;
        }
        true
    };

    for r in 0..9 {
        if !is_permutation((0..9).map(|c| board[r][c]).collect()) {
            return false;
        }
    }
    for c in 0..9 {
        if !is_permutation((0..9).map(|r| board[r][c]).collect()) {
            return false;
        }
    }
    for br in 0..3 {
        for bc in 0..3 {
            let cells: Vec<char> = (0..9)
                .map(|i| board[br * 3 + i / 3][bc * 3 + i % 3])
                .collect();
            if !is_permutation(cells) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut board = board_of(&[
        "53..7....",
        "6..195...",
        ".98....6.",
        "8...6...3",
        "4..8.3..1",
        "7...2...6",
        ".6....28.",
        "...419..5",
        "....8..79",
    ]);
    Solution::solve_sudoku(&mut board);
    println!("valid solution: {}", is_solved_valid(&board));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_produces_a_valid_solution() {
        let mut board = board_of(&[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        Solution::solve_sudoku(&mut board);
        assert!(is_solved_valid(&board));
    }

    #[test]
    fn solution_preserves_given_clues() {
        let original = board_of(&[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        let mut board = original.clone();
        Solution::solve_sudoku(&mut board);
        for r in 0..9 {
            for c in 0..9 {
                if original[r][c] != '.' {
                    assert_eq!(board[r][c], original[r][c]);
                }
            }
        }
    }

    #[test]
    fn no_empty_cells_remain() {
        let mut board = board_of(&[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        Solution::solve_sudoku(&mut board);
        for row in &board {
            assert!(!row.contains(&'.'));
        }
    }
}
