//! Grind 169 — LeetCode #32 Longest Valid Parentheses (Hard)
//!
//! Given a string containing just ( and ), find the length of the
//! longest valid (well-formed) parentheses substring. A stack of
//! indices, seeded with -1 as a "base" for width calculations: '(' is
//! pushed; ')' pops, and if the stack becomes empty the current index
//! becomes the new base, otherwise the gap to the new top is a valid
//! span's length.
//!
//! Example:
//!   Input: s = ")()())"
//!   Output: 4   ("()()")

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![-1];
        let mut best = 0;

        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    best = best.max(i as i32 - stack.last().unwrap());
                }
            }
        }

        best
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_valid_parentheses(")()())".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_valid_parentheses(")()())".to_string()),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_valid_parentheses("(()".to_string()),
            2
        );
    }

    #[test]
    fn example_3_empty() {
        assert_eq!(
            Solution::longest_valid_parentheses("".to_string()),
            0
        );
    }

    #[test]
    fn fully_valid() {
        assert_eq!(
            Solution::longest_valid_parentheses("()()".to_string()),
            4
        );
    }
}
