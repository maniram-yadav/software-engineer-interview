//! LeetCode Top Interview 150 — #25 Valid Palindrome (Easy)
//!
//! Given a string `s`, considering only alphanumeric characters and
//! ignoring case, determine if it reads the same forward and backward.
//!
//! Example:
//!   Input: s = "A man, a plan, a canal: Panama"
//!   Output: true

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let n = chars.len();
        for i in 0..n / 2 {
            if chars[i] != chars[n - 1 - i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn example_3_empty_after_filtering() {
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }

    #[test]
    fn mixed_alnum() {
        assert_eq!(Solution::is_palindrome("0P".to_string()), false);
    }
}
