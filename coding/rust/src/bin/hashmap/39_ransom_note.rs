//! LeetCode Top Interview 150 — #39 Ransom Note (Easy)
//!
//! Given strings `ransom_note` and `magazine`, return true if
//! `ransom_note` can be constructed using letters from `magazine` (each
//! letter used at most once).
//!
//! Example:
//!   Input: ransomNote = "aa", magazine = "aab"
//!   Output: true

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        for c in ransom_note.chars() {
            match counts.get_mut(&c) {
                Some(cnt) if *cnt > 0 => {
                    *cnt -= 1;
                }
                _ => return false,
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_construct("aa".to_string(), "aab".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::can_construct("a".to_string(), "b".to_string()),
            false
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::can_construct("aa".to_string(), "ab".to_string()),
            false
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::can_construct("aa".to_string(), "aab".to_string()),
            true
        );
    }

    #[test]
    fn empty_ransom_note() {
        assert_eq!(
            Solution::can_construct("".to_string(), "anything".to_string()),
            true
        );
    }
}
