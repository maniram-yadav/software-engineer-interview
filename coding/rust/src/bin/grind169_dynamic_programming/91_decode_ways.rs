//! Grind 169 — LeetCode #91 Decode Ways (Medium)
//!
//! A message of digits can be decoded via 'A'->1, ..., 'Z'->26. Given a
//! digit string s, return the number of ways to decode it. dp[i] is the
//! number of ways to decode the first i characters, combining a
//! single-digit step (dp[i-1], if s[i-1] != '0') and a two-digit step
//! (dp[i-2], if s[i-2..i] is in 10..=26).
//!
//! Example:
//!   Input: s = "12"
//!   Output: 2   ("AB" or "L")

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 || bytes[0] == b'0' {
            return 0;
        }
        let mut dp = vec![0i64; n + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            if bytes[i - 1] != b'0' {
                dp[i] += dp[i - 1];
            }
            let two = (bytes[i - 2] - b'0') as i32 * 10 + (bytes[i - 1] - b'0') as i32;
            if (10..=26).contains(&two) {
                dp[i] += dp[i - 2];
            }
        }

        dp[n] as i32
    }
}

fn main() {
    println!("{}", Solution::num_decodings("12".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
    }

    #[test]
    fn example_3_leading_zero_is_invalid() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }

    #[test]
    fn internal_zero_forces_two_digit_group() {
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
    }
}
