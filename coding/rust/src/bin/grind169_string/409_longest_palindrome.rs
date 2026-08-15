//! Grind 169 — LeetCode #409 Longest Palindrome (Easy)
//!
//! Given a string of lowercase/uppercase letters, return the length of
//! the longest palindrome that can be built from those letters
//! (case-sensitive, rearrangement allowed). Every character can
//! contribute its largest even count to the palindrome's two halves; at
//! most one leftover odd-count character can sit in the center.
//!
//! Example:
//!   Input: s = "abccccdd"
//!   Output: 7   (e.g. "dccaccd")

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut length = 0;
        let mut has_odd = false;
        for &c in counts.values() {
            length += c - (c % 2);
            if c % 2 == 1 {
                has_odd = true;
            }
        }
        if has_odd {
            length += 1;
        }
        length
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_palindrome("abccccdd".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_palindrome("abccccdd".to_string()),
            7
        );
    }

    #[test]
    fn example_2_single_char() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn all_pairs_no_odd() {
        assert_eq!(Solution::longest_palindrome("bb".to_string()), 2);
    }

    #[test]
    fn case_sensitive() {
        assert_eq!(Solution::longest_palindrome("Aa".to_string()), 1);
    }
}
