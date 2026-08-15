//! LeetCode Top Interview 150 — #42 Valid Anagram (Easy)
//!
//! Given strings `s` and `t`, return true if `t` is an anagram of `s`.
//! Assumes lowercase English letters, so a fixed 26-slot counter works.
//!
//! Example:
//!   Input: s = "anagram", t = "nagaram"
//!   Output: true

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts = [0i32; 26];
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        for b in t.bytes() {
            counts[(b - b'a') as usize] -= 1;
        }
        counts.iter().all(|&c| c == 0)
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }

    #[test]
    fn different_lengths() {
        assert_eq!(
            Solution::is_anagram("a".to_string(), "ab".to_string()),
            false
        );
    }

    #[test]
    fn identical_strings() {
        assert_eq!(
            Solution::is_anagram("abc".to_string(), "abc".to_string()),
            true
        );
    }
}
