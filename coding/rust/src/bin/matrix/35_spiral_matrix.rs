//! LeetCode Top Interview 150 — #35 Spiral Matrix (Medium)
//!
//! Given an m x n matrix, return all elements in spiral order.
//!
//! Example:
//!   Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//!   Output: [1,2,3,6,9,8,7,4,5]

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }
        let mut result = Vec::new();
        let mut top = 0i32;
        let mut bottom = matrix.len() as i32 - 1;
        let mut left = 0i32;
        let mut right = matrix[0].len() as i32 - 1;

        while top <= bottom && left <= right {
            for c in left..=right {
                result.push(matrix[top as usize][c as usize]);
            }
            top += 1;

            for r in top..=bottom {
                result.push(matrix[r as usize][right as usize]);
            }
            right -= 1;

            if top <= bottom {
                for c in (left..=right).rev() {
                    result.push(matrix[bottom as usize][c as usize]);
                }
                bottom -= 1;
            }

            if left <= right {
                for r in (top..=bottom).rev() {
                    result.push(matrix[r as usize][left as usize]);
                }
                left += 1;
            }
        }

        result
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:?}", Solution::spiral_order(matrix));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn example_2_non_square() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn single_row() {
        let matrix = vec![vec![1, 2, 3]];
        assert_eq!(Solution::spiral_order(matrix), vec![1, 2, 3]);
    }

    #[test]
    fn single_column() {
        let matrix = vec![vec![1], vec![2], vec![3]];
        assert_eq!(Solution::spiral_order(matrix), vec![1, 2, 3]);
    }
}
