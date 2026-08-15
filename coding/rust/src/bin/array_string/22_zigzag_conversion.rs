//! LeetCode Top Interview 150 — #22 Zigzag Conversion (Medium)
//!
//! Write a string in a zigzag pattern on a given number of rows, then read
//! it row by row.
//!
//! Example:
//!   Input: s = "PAYPALISHIRING", numRows = 3
//!   Output: "PAHNAPLSIIGYIR"

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut rows = vec![String::new(); num_rows];
        let mut cur_row = 0usize;
        let mut going_down = false;

        for c in s.chars() {
            rows[cur_row].push(c);
            if cur_row == 0 || cur_row == num_rows - 1 {
                going_down = !going_down;
            }
            if going_down {
                cur_row += 1;
            } else {
                cur_row -= 1;
            }
        }

        rows.concat()
    }
}

fn main() {
    println!(
        "{}",
        Solution::convert("PAYPALISHIRING".to_string(), 3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }

    #[test]
    fn single_row_unchanged() {
        assert_eq!(
            Solution::convert("ABCDEF".to_string(), 1),
            "ABCDEF".to_string()
        );
    }

    #[test]
    fn rows_equal_length() {
        assert_eq!(
            Solution::convert("AB".to_string(), 1),
            "AB".to_string()
        );
    }
}
