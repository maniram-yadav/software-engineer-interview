//! LeetCode Top Interview 150 — #36 Rotate Image (Medium)
//!
//! Given an n x n 2D matrix representing an image, rotate it 90 degrees
//! clockwise, in place. Solved by transposing the matrix, then reversing
//! each row.
//!
//! Example:
//!   Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//!   Output: [[7,4,1],[8,5,2],[9,6,3]]

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..n {
            for j in (i + 1)..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}

fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    println!("{:?}", matrix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]
        );
    }

    #[test]
    fn example_2_four_by_four() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn single_element() {
        let mut matrix = vec![vec![1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);
    }

    #[test]
    fn two_by_two() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);
    }
}
