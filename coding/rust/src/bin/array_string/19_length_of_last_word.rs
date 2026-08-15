//! LeetCode Top Interview 150 — #19 Length of Last Word (Easy)
//!
//! Given a string `s` of words separated by spaces, return the length of
//! the last word.
//!
//! Example:
//!   Input: s = "Hello World"
//!   Output: 5

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end()
            .rsplit(' ')
            .next()
            .map(|w| w.len())
            .unwrap_or(0) as i32
    }
}

fn main() {
    println!(
        "length: {}",
        Solution::length_of_last_word("Hello World".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::length_of_last_word("Hello World".to_string()),
            5
        );
    }

    #[test]
    fn example_2_trailing_spaces() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn example_3_single_word() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }

    #[test]
    fn single_char() {
        assert_eq!(Solution::length_of_last_word("a".to_string()), 1);
    }
}
