//! LeetCode Top Interview 150 — #52 Valid Parentheses (Easy)
//!
//! Given a string of ()[]{} characters, determine if brackets are
//! properly matched and nested.
//!
//! Example:
//!   Input: s = "()[]{}"
//!   Output: true

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}

fn main() {
    println!("{}", Solution::is_valid("()[]{}".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn example_3_wrong_type() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn wrong_order() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }

    #[test]
    fn properly_nested() {
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }

    #[test]
    fn unclosed() {
        assert_eq!(Solution::is_valid("(((".to_string()), false);
    }
}
