//! LeetCode Top Interview 150 — #106 Generate Parentheses (Medium)
//!
//! Given n pairs of parentheses, generate all combinations of
//! well-formed parentheses.
//!
//! Example:
//!   Input: n = 3
//!   Output: ["((()))","(()())","(())()","()(())","()()()"]

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(open: i32, close: i32, n: i32, current: &mut String, result: &mut Vec<String>) {
            if current.len() as i32 == 2 * n {
                result.push(current.clone());
                return;
            }
            if open < n {
                current.push('(');
                backtrack(open + 1, close, n, current, result);
                current.pop();
            }
            if close < open {
                current.push(')');
                backtrack(open, close + 1, n, current, result);
                current.pop();
            }
        }

        let mut result = Vec::new();
        let mut current = String::new();
        backtrack(0, 0, n, &mut current, &mut result);
        result
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        let mut result = Solution::generate_parenthesis(3);
        result.sort();
        let mut expected = v(&["((()))", "(()())", "(())()", "()(())", "()()()"]);
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2_single_pair() {
        assert_eq!(Solution::generate_parenthesis(1), v(&["()"]));
    }

    #[test]
    fn count_matches_catalan_number() {
        // C(4) = 14 well-formed combinations for n = 4.
        assert_eq!(Solution::generate_parenthesis(4).len(), 14);
    }
}
