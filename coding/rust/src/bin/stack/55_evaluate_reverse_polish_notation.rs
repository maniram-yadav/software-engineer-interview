//! LeetCode Top Interview 150 — #55 Evaluate Reverse Polish Notation (Medium)
//!
//! Evaluate an arithmetic expression given in Reverse Polish (postfix)
//! notation as an array of tokens.
//!
//! Example:
//!   Input: tokens = ["2","1","+","3","*"]
//!   Output: 9

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let res = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => unreachable!(),
                    };
                    stack.push(res);
                }
                num => stack.push(num.parse::<i32>().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}

fn main() {
    let tokens = vec!["2", "1", "+", "3", "*"]
        .into_iter()
        .map(String::from)
        .collect();
    println!("{}", Solution::eval_rpn(tokens));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::eval_rpn(v(&["2", "1", "+", "3", "*"])), 9);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::eval_rpn(v(&["4", "13", "5", "/", "+"])),
            6
        );
    }

    #[test]
    fn negative_result() {
        assert_eq!(Solution::eval_rpn(v(&["3", "4", "-"])), -1);
    }

    #[test]
    fn single_number() {
        assert_eq!(Solution::eval_rpn(v(&["42"])), 42);
    }
}
