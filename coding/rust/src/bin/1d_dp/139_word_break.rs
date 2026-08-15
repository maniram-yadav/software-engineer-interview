//! LeetCode Top Interview 150 — #139 Word Break (Medium)
//!
//! Given a string s and a dictionary of words wordDict, return true if s
//! can be segmented into a space-separated sequence of one or more
//! dictionary words. dp[i] means s[..i] is segmentable; dp[i] is true if
//! some earlier split point j has dp[j] true and s[j..i] is a dictionary
//! word.
//!
//! Example:
//!   Input: s = "leetcode", wordDict = ["leet","code"]
//!   Output: true

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 1..=n {
            for j in 0..i {
                if dp[j] {
                    let substr = std::str::from_utf8(&s_bytes[j..i]).unwrap();
                    if word_set.contains(substr) {
                        dp[i] = true;
                        break;
                    }
                }
            }
        }

        dp[n]
    }
}

fn v(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::word_break("leetcode".to_string(), v(&["leet", "code"]))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::word_break("leetcode".to_string(), v(&["leet", "code"])),
            true
        );
    }

    #[test]
    fn example_2_reusable_words() {
        assert_eq!(
            Solution::word_break("applepenapple".to_string(), v(&["apple", "pen"])),
            true
        );
    }

    #[test]
    fn example_3_not_breakable() {
        assert_eq!(
            Solution::word_break(
                "catsandog".to_string(),
                v(&["cats", "dog", "sand", "and", "cat"])
            ),
            false
        );
    }

    #[test]
    fn empty_string_is_trivially_breakable() {
        assert_eq!(Solution::word_break("".to_string(), v(&["a"])), true);
    }
}
