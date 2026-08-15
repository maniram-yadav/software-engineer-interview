//! LeetCode Top Interview 150 — #89 Number of Islands (Medium)
//!
//! Given an m x n 2D binary grid of '1' (land) and '0' (water), return
//! the number of islands (connected land, 4-directionally). Solved with
//! DFS flood-fill, sinking each visited land cell to '0' so it's never
//! recounted.
//!
//! Example:
//!   Input: grid = [["1","1","0","0","0"],["1","1","0","0","0"],
//!                   ["0","0","1","0","0"],["0","0","0","1","1"]]
//!   Output: 3

struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        fn sink(grid: &mut Vec<Vec<char>>, r: i32, c: i32, rows: i32, cols: i32) {
            if r < 0 || r >= rows || c < 0 || c >= cols || grid[r as usize][c as usize] != '1' {
                return;
            }
            grid[r as usize][c as usize] = '0';
            sink(grid, r + 1, c, rows, cols);
            sink(grid, r - 1, c, rows, cols);
            sink(grid, r, c + 1, rows, cols);
            sink(grid, r, c - 1, rows, cols);
        }

        let mut count = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r as usize][c as usize] == '1' {
                    count += 1;
                    sink(&mut grid, r, c, rows, cols);
                }
            }
        }
        count
    }
}

fn grid_of(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.chars().collect()).collect()
}

fn main() {
    let grid = grid_of(&["11000", "11000", "00100", "00011"]);
    println!("{}", Solution::num_islands(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = grid_of(&["11110", "11010", "11000", "00000"]);
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn example_2() {
        let grid = grid_of(&["11000", "11000", "00100", "00011"]);
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn all_water() {
        let grid = grid_of(&["000", "000"]);
        assert_eq!(Solution::num_islands(grid), 0);
    }

    #[test]
    fn all_land_is_one_island() {
        let grid = grid_of(&["11", "11"]);
        assert_eq!(Solution::num_islands(grid), 1);
    }
}
