//! Grind 169 — LeetCode #51 N-Queens (Hard)
//!
//! Place n queens on an n x n chessboard so that no two attack each
//! other; return all distinct board configurations. Backtracking with
//! O(1) attack checks via boolean arrays for used columns and the two
//! diagonal directions.
//!
//! Example:
//!   Input: n = 4
//!   Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n - 1];
        let mut diag2 = vec![false; 2 * n - 1];
        let mut positions = vec![0usize; n];
        let mut result = Vec::new();

        fn backtrack(
            row: usize,
            n: usize,
            cols: &mut Vec<bool>,
            diag1: &mut Vec<bool>,
            diag2: &mut Vec<bool>,
            positions: &mut Vec<usize>,
            result: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                let board: Vec<String> = positions
                    .iter()
                    .map(|&c| {
                        let mut row_chars = vec!['.'; n];
                        row_chars[c] = 'Q';
                        row_chars.into_iter().collect()
                    })
                    .collect();
                result.push(board);
                return;
            }
            for c in 0..n {
                let d1 = row + c;
                let d2 = row + n - 1 - c;
                if cols[c] || diag1[d1] || diag2[d2] {
                    continue;
                }
                cols[c] = true;
                diag1[d1] = true;
                diag2[d2] = true;
                positions[row] = c;
                backtrack(row + 1, n, cols, diag1, diag2, positions, result);
                cols[c] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
        }

        if n > 0 {
            backtrack(0, n, &mut cols, &mut diag1, &mut diag2, &mut positions, &mut result);
        }
        result
    }
}

// Verifies each row has exactly one queen, and no two queens share a
// column or diagonal.
fn is_valid_board(board: &[String]) -> bool {
    let n = board.len();
    let queens: Vec<usize> = board
        .iter()
        .map(|row| row.chars().position(|c| c == 'Q').unwrap())
        .collect();
    if queens.len() != n {
        return false;
    }
    for r1 in 0..n {
        for r2 in (r1 + 1)..n {
            let (c1, c2) = (queens[r1] as i32, queens[r2] as i32);
            if c1 == c2 || (r1 as i32 - r2 as i32).abs() == (c1 - c2).abs() {
                return false;
            }
        }
    }
    true
}

fn main() {
    let result = Solution::solve_n_queens(4);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_has_two_valid_solutions() {
        let result = Solution::solve_n_queens(4);
        assert_eq!(result.len(), 2);
        for board in &result {
            assert!(is_valid_board(board));
        }
    }

    #[test]
    fn example_2_single_queen() {
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q".to_string()]]);
    }

    #[test]
    fn no_solution_for_two_or_three() {
        assert_eq!(Solution::solve_n_queens(2).len(), 0);
        assert_eq!(Solution::solve_n_queens(3).len(), 0);
    }

    #[test]
    fn eight_queens_classic_count() {
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
}
