//! LeetCode Top Interview 150 — #105 N-Queens II (Hard)
//!
//! Return the number of distinct solutions to the n-queens puzzle
//! (placing n queens on an n x n board so no two attack each other).
//! Solved with backtracking, tracking used columns and the two diagonal
//! directions with boolean arrays for O(1) attack checks.
//!
//! Example:
//!   Input: n = 4
//!   Output: 2

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n - 1]; // indexed by row + col
        let mut diag2 = vec![false; 2 * n - 1]; // indexed by row - col + n - 1

        fn backtrack(
            row: usize,
            n: usize,
            cols: &mut Vec<bool>,
            diag1: &mut Vec<bool>,
            diag2: &mut Vec<bool>,
        ) -> i32 {
            if row == n {
                return 1;
            }
            let mut count = 0;
            for c in 0..n {
                let d1 = row + c;
                let d2 = row + n - 1 - c;
                if cols[c] || diag1[d1] || diag2[d2] {
                    continue;
                }
                cols[c] = true;
                diag1[d1] = true;
                diag2[d2] = true;
                count += backtrack(row + 1, n, cols, diag1, diag2);
                cols[c] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
            count
        }

        if n == 0 {
            return 0;
        }
        backtrack(0, n, &mut cols, &mut diag1, &mut diag2)
    }
}

fn main() {
    println!("{}", Solution::total_n_queens(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }

    #[test]
    fn example_2_single_queen() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }

    #[test]
    fn no_solution_for_two_or_three() {
        assert_eq!(Solution::total_n_queens(2), 0);
        assert_eq!(Solution::total_n_queens(3), 0);
    }

    #[test]
    fn eight_queens_classic_count() {
        assert_eq!(Solution::total_n_queens(8), 92);
    }
}
