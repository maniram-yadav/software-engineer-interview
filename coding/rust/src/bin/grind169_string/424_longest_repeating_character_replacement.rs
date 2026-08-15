//! Grind 169 — LeetCode #424 Longest Repeating Character Replacement
//! (Medium)
//!
//! Given a string s and an integer k, you can replace up to k characters
//! with any other uppercase letter. Return the length of the longest
//! substring containing the same letter after such replacements. A
//! sliding window is valid as long as (window length - most frequent
//! char count) <= k; shrink from the left otherwise.
//!
//! Example:
//!   Input: s = "ABAB", k = 2
//!   Output: 4

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let mut counts = [0i32; 26];
        let mut left = 0;
        let mut max_count = 0;
        let mut best = 0;

        for right in 0..bytes.len() {
            let idx = (bytes[right] - b'A') as usize;
            counts[idx] += 1;
            max_count = max_count.max(counts[idx]);

            while (right - left + 1) as i32 - max_count > k {
                let lidx = (bytes[left] - b'A') as usize;
                counts[lidx] -= 1;
                left += 1;
            }

            best = best.max((right - left + 1) as i32);
        }

        best
    }
}

fn main() {
    println!(
        "{}",
        Solution::character_replacement("ABAB".to_string(), 2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::character_replacement("ABAB".to_string(), 2),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::character_replacement("AABABBA".to_string(), 1),
            4
        );
    }

    #[test]
    fn k_zero_finds_longest_run() {
        assert_eq!(
            Solution::character_replacement("ABAB".to_string(), 0),
            1
        );
    }

    #[test]
    fn all_same_char() {
        assert_eq!(
            Solution::character_replacement("AAAA".to_string(), 2),
            4
        );
    }
}
