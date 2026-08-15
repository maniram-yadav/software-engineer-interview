//! LeetCode Top Interview 150 — #21 Reverse Words in a String (Medium)
//!
//! Given a string `s`, reverse the order of the words (words are separated
//! by one or more spaces); collapse extra spaces and trim leading/trailing
//! spaces.
//!
//! Example:
//!   Input: s = "  the sky is blue  "
//!   Output: "blue is sky the"

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::reverse_words("  the sky is blue  ".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );
    }

    #[test]
    fn example_2_extra_spaces() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello".to_string()
        );
    }

    #[test]
    fn example_3_multiple_internal_spaces() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
    }

    #[test]
    fn single_word() {
        assert_eq!(
            Solution::reverse_words("hello".to_string()),
            "hello".to_string()
        );
    }
}
