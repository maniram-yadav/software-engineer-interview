//! Grind 169 — LeetCode #227 Basic Calculator II (Medium)
//!
//! Given a string expression containing non-negative integers and
//! + - * / (no parentheses), evaluate it following normal operator
//! precedence. Solved with a stack: '+' and '-' push signed numbers,
//! while '*' and '/' apply immediately to the top of the stack, so the
//! final sum of the stack is the answer.
//!
//! Example:
//!   Input: s = "3+2*2"
//!   Output: 7

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut num: i64 = 0;
        let mut op = '+';
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            let c = chars[i];
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap() as i64;
            }
            if (!c.is_ascii_digit() && c != ' ') || i == chars.len() - 1 {
                match op {
                    '+' => stack.push(num as i32),
                    '-' => stack.push(-(num as i32)),
                    '*' => {
                        let top = stack.pop().unwrap();
                        stack.push(top * num as i32);
                    }
                    '/' => {
                        let top = stack.pop().unwrap();
                        stack.push(top / num as i32);
                    }
                    _ => {}
                }
                op = c;
                num = 0;
            }
        }

        stack.iter().sum()
    }
}

fn main() {
    println!("{}", Solution::calculate("3+2*2".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
    }

    #[test]
    fn example_2_division_truncates() {
        assert_eq!(Solution::calculate(" 3/2 ".to_string()), 1);
    }

    #[test]
    fn example_3_mixed_operators() {
        assert_eq!(Solution::calculate(" 3+5 / 2 ".to_string()), 5);
    }

    #[test]
    fn single_number() {
        assert_eq!(Solution::calculate("42".to_string()), 42);
    }
}
