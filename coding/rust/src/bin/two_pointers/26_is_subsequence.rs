//! LeetCode Top Interview 150 — #26 Is Subsequence (Easy)
//!
//! Given strings `s` and `t`, determine if `s` is a subsequence of `t`
//! (characters of `s` appear in `t` in order, not necessarily contiguous).
//!
//! Example:
//!   Input: s = "abc", t = "ahbgdc"
//!   Output: true

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut it = t.chars();
        for c in s.chars() {
            loop {
                match it.next() {
                    Some(tc) if tc == c => break,
                    Some(_) => continue,
                    None => return false,
                }
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }

    #[test]
    fn empty_s_is_always_subsequence() {
        assert_eq!(Solution::is_subsequence("".to_string(), "abc".to_string()), true);
    }

    #[test]
    fn s_longer_than_t() {
        assert_eq!(Solution::is_subsequence("abcd".to_string(), "abc".to_string()), false);
    }
}
