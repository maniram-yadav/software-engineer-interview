//! Grind 169 — LeetCode #62 Unique Paths (Medium)
//!
//! A robot on an m x n grid starts at the top-left corner and can only
//! move down or right. Return the number of unique paths to the
//! bottom-right corner.
//!
//! Example:
//!   Input: m = 3, n = 7
//!   Output: 28

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1i64; n];
        for _ in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }
        dp[n - 1] as i32
    }
}

fn main() {
    println!("{}", Solution::unique_paths(3, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn single_row() {
        assert_eq!(Solution::unique_paths(1, 5), 1);
    }

    #[test]
    fn square_grid() {
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
