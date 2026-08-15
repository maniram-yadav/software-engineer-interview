//! LeetCode Top Interview 150 — #34 Valid Sudoku (Medium)
//!
//! Determine if a partially filled 9x9 Sudoku board is valid: each row,
//! column, and 3x3 sub-box must contain no repeated digits 1-9 (empty
//! cells are '.'; validity only needs to hold for filled cells, not
//! solvability).
//!
//! Example:
//!   Input: board = [["5","3",".",".","7",".",".",".","."], ...]
//!   Output: true

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' {
                    continue;
                }
                let num = (ch as u8 - b'1') as usize;
                if num > 8 {
                    return false;
                }
                let b = (r / 3) * 3 + c / 3;
                if rows[r][num] || cols[c][num] || boxes[b][num] {
                    return false;
                }
                rows[r][num] = true;
                cols[c][num] = true;
                boxes[b][num] = true;
            }
        }

        true
    }
}

fn board_from(rows: [[char; 9]; 9]) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.to_vec()).collect()
}

fn main() {
    let board = board_from([
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]);
    println!("{}", Solution::is_valid_sudoku(board));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_valid() {
        let board = board_from([
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }

    #[test]
    fn example_2_invalid_duplicate_in_box() {
        // Same as example 1, but top-left "5" changed to "8", creating two
        // 8's in the top-left 3x3 sub-box.
        let board = board_from([
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn duplicate_in_row() {
        let mut rows = [['.'; 9]; 9];
        rows[0][0] = '1';
        rows[0][1] = '1';
        assert_eq!(Solution::is_valid_sudoku(board_from(rows)), false);
    }

    #[test]
    fn all_empty_is_valid() {
        let rows = [['.'; 9]; 9];
        assert_eq!(Solution::is_valid_sudoku(board_from(rows)), true);
    }
}
