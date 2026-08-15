//! Grind 169 — LeetCode #844 Backspace String Compare (Easy)
//!
//! Given two strings s and t containing lowercase letters and '#'
//! (backspace), return true if they're equal after applying the
//! backspaces.
//!
//! Example:
//!   Input: s = "ab#c", t = "ad#c"
//!   Output: true   (both become "ac")

struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn process(s: &str) -> String {
            let mut stack = Vec::new();
            for c in s.chars() {
                if c == '#' {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
            stack.into_iter().collect()
        }
        process(&s) == process(&t)
    }
}

fn main() {
    println!(
        "{}",
        Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
    }

    #[test]
    fn example_3_not_equal() {
        assert_eq!(
            Solution::backspace_compare("a#c".to_string(), "b".to_string()),
            false
        );
    }

    #[test]
    fn backspace_beyond_start_is_safe() {
        assert_eq!(
            Solution::backspace_compare("##a".to_string(), "a".to_string()),
            true
        );
    }
}
