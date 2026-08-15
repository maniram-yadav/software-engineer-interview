//! LeetCode Top Interview 150 — #56 Basic Calculator (Hard)
//!
//! Given a string expression containing +, -, (, ), digits, and spaces,
//! implement a basic calculator to evaluate it (no * or /). Solved with a
//! running result/sign and an explicit stack for parenthesized groups.
//!
//! Example:
//!   Input: s = "(1+(4+5+2)-3)+(6+8)"
//!   Output: 23

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut result: i64 = 0;
        let mut sign: i64 = 1;
        let mut num: i64 = 0;
        let mut stack: Vec<(i64, i64)> = Vec::new();

        for c in s.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + (c as i64 - '0' as i64);
            } else if c == '+' {
                result += sign * num;
                num = 0;
                sign = 1;
            } else if c == '-' {
                result += sign * num;
                num = 0;
                sign = -1;
            } else if c == '(' {
                stack.push((result, sign));
                result = 0;
                sign = 1;
            } else if c == ')' {
                result += sign * num;
                num = 0;
                let (prev_result, prev_sign) = stack.pop().unwrap();
                result = prev_result + prev_sign * result;
            }
            // whitespace: no-op
        }
        result += sign * num;

        result as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
    }

    #[test]
    fn example_3_nested_parens() {
        assert_eq!(
            Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()),
            23
        );
    }

    #[test]
    fn leading_unary_minus() {
        assert_eq!(Solution::calculate("-2+ 1".to_string()), -1);
    }

    #[test]
    fn plain_number() {
        assert_eq!(Solution::calculate("42".to_string()), 42);
    }
}
