//! LeetCode Top Interview 150 — #37 Set Matrix Zeroes (Medium)
//!
//! Given an m x n matrix, if an element is 0, set its entire row and
//! column to 0, in place.
//!
//! Example:
//!   Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
//!   Output: [[1,0,1],[0,0,0],[1,0,1]]

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut zero_rows = vec![false; rows];
        let mut zero_cols = vec![false; cols];

        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == 0 {
                    zero_rows[r] = true;
                    zero_cols[c] = true;
                }
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if zero_rows[r] || zero_cols[c] {
                    matrix[r][c] = 0;
                }
            }
        }
    }
}

fn main() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]
        );
    }

    #[test]
    fn example_2() {
        let mut matrix = vec![
            vec![0, 1, 2, 0],
            vec![3, 4, 5, 2],
            vec![1, 3, 1, 5],
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 4, 5, 0],
                vec![0, 3, 1, 0]
            ]
        );
    }

    #[test]
    fn no_zeroes_unchanged() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn single_zero_cell() {
        let mut matrix = vec![vec![0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![0]]);
    }
}
