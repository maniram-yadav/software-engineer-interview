//! LeetCode Top Interview 150 — #115 Search a 2D Matrix (Medium)
//!
//! Given an m x n matrix where each row is sorted ascending and the
//! first integer of each row is greater than the last integer of the
//! previous row, determine if target exists, in O(log(mn)). Treated as a
//! single sorted virtual array indexed 0..rows*cols.
//!
//! Example:
//!   Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
//!   Output: true

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let (mut lo, mut hi) = (0i32, (rows * cols) as i32 - 1);

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let r = (mid as usize) / cols;
            let c = (mid as usize) % cols;
            let val = matrix[r][c];
            if val == target {
                return true;
            } else if val < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        false
    }
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    println!("{}", Solution::search_matrix(matrix, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(Solution::search_matrix(matrix, 3), true);
    }

    #[test]
    fn example_2_not_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(Solution::search_matrix(matrix, 13), false);
    }

    #[test]
    fn single_cell_found() {
        assert_eq!(Solution::search_matrix(vec![vec![5]], 5), true);
    }

    #[test]
    fn last_element() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(Solution::search_matrix(matrix, 60), true);
    }
}
