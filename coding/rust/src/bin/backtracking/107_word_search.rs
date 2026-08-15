//! LeetCode Top Interview 150 — #107 Word Search (Medium)
//!
//! Given an m x n grid of characters and a word, return true if the word
//! can be constructed from letters of sequentially adjacent cells (no
//! cell reused). Solved with DFS backtracking, temporarily marking
//! visited cells so they aren't reused within the same path.
//!
//! Example:
//!   Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
//!   Output: true

struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len() as i32;
        let cols = board[0].len() as i32;
        let word_chars: Vec<char> = word.chars().collect();

        fn dfs(
            board: &mut Vec<Vec<char>>,
            r: i32,
            c: i32,
            rows: i32,
            cols: i32,
            word: &[char],
            idx: usize,
        ) -> bool {
            if idx == word.len() {
                return true;
            }
            if r < 0 || r >= rows || c < 0 || c >= cols {
                return false;
            }
            if board[r as usize][c as usize] != word[idx] {
                return false;
            }

            let temp = board[r as usize][c as usize];
            board[r as usize][c as usize] = '#';
            let found = dfs(board, r + 1, c, rows, cols, word, idx + 1)
                || dfs(board, r - 1, c, rows, cols, word, idx + 1)
                || dfs(board, r, c + 1, rows, cols, word, idx + 1)
                || dfs(board, r, c - 1, rows, cols, word, idx + 1);
            board[r as usize][c as usize] = temp;
            found
        }

        for r in 0..rows {
            for c in 0..cols {
                if dfs(&mut board, r, c, rows, cols, &word_chars, 0) {
                    return true;
                }
            }
        }
        false
    }
}

fn board_of(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.chars().collect()).collect()
}

fn main() {
    let board = board_of(&["ABCE", "SFCS", "ADEE"]);
    println!("{}", Solution::exist(board, "ABCCED".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let board = board_of(&["ABCE", "SFCS", "ADEE"]);
        assert_eq!(Solution::exist(board, "ABCCED".to_string()), true);
    }

    #[test]
    fn example_2() {
        let board = board_of(&["ABCE", "SFCS", "ADEE"]);
        assert_eq!(Solution::exist(board, "SEE".to_string()), true);
    }

    #[test]
    fn example_3_no_reuse() {
        let board = board_of(&["ABCE", "SFCS", "ADEE"]);
        assert_eq!(Solution::exist(board, "ABCB".to_string()), false);
    }

    #[test]
    fn single_cell_match() {
        let board = board_of(&["A"]);
        assert_eq!(Solution::exist(board, "A".to_string()), true);
    }
}
