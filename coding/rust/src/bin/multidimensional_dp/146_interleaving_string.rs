//! LeetCode Top Interview 150 — #146 Interleaving String (Medium)
//!
//! Given strings s1, s2, and s3, determine if s3 is formed by an
//! interleaving of s1 and s2 (preserving each string's relative
//! character order). dp[i][j] means the first i+j characters of s3 can
//! be formed by interleaving s1[..i] and s2[..j].
//!
//! Example:
//!   Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
//!   Output: true

struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (a, b, c) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (m, n) = (a.len(), b.len());
        if m + n != c.len() {
            return false;
        }

        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        for i in 0..=m {
            for j in 0..=n {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut ok = false;
                if i > 0 && dp[i - 1][j] && a[i - 1] == c[i + j - 1] {
                    ok = true;
                }
                if !ok && j > 0 && dp[i][j - 1] && b[j - 1] == c[i + j - 1] {
                    ok = true;
                }
                dp[i][j] = ok;
            }
        }

        dp[m][n]
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbcbcac".to_string()
            ),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
    }

    #[test]
    fn example_3_all_empty() {
        assert_eq!(
            Solution::is_interleave("".to_string(), "".to_string(), "".to_string()),
            true
        );
    }

    #[test]
    fn length_mismatch_is_false() {
        assert_eq!(
            Solution::is_interleave("a".to_string(), "b".to_string(), "ab".to_string()),
            true
        );
        assert_eq!(
            Solution::is_interleave("a".to_string(), "b".to_string(), "abc".to_string()),
            false
        );
    }
}
