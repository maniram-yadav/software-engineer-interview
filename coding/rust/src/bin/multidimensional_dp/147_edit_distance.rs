//! LeetCode Top Interview 150 — #147 Edit Distance (Medium)
//!
//! Given two strings word1 and word2, return the minimum number of
//! operations (insert, delete, replace) to convert word1 into word2.
//! Classic Levenshtein distance DP: dp[i][j] is the edit distance between
//! word1[..i] and word2[..j].
//!
//! Example:
//!   Input: word1 = "horse", word2 = "ros"
//!   Output: 3

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (a, b) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0i32; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }

        for i in 1..=m {
            for j in 1..=n {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
                }
            }
        }

        dp[m][n]
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_distance("horse".to_string(), "ros".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }

    #[test]
    fn identical_strings() {
        assert_eq!(
            Solution::min_distance("abc".to_string(), "abc".to_string()),
            0
        );
    }

    #[test]
    fn one_empty_string() {
        assert_eq!(
            Solution::min_distance("".to_string(), "abc".to_string()),
            3
        );
    }
}
