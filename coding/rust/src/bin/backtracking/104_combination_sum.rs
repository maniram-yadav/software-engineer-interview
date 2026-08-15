//! LeetCode Top Interview 150 — #104 Combination Sum (Medium)
//!
//! Given an array of distinct positive integers candidates and a target,
//! return all unique combinations where the chosen numbers (reusable,
//! unlimited) sum to target.
//!
//! Example:
//!   Input: candidates = [2,3,6,7], target = 7
//!   Output: [[2,2,3],[7]]

struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            candidates: &[i32],
            start: usize,
            remaining: i32,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if remaining == 0 {
                result.push(current.clone());
                return;
            }
            if remaining < 0 {
                return;
            }
            for i in start..candidates.len() {
                current.push(candidates[i]);
                backtrack(candidates, i, remaining - candidates[i], current, result);
                current.pop();
            }
        }

        let mut result = Vec::new();
        let mut current = Vec::new();
        backtrack(&candidates, 0, target, &mut current, &mut result);
        result
    }
}

fn main() {
    println!("{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn example_2() {
        let mut result = Solution::combination_sum(vec![2, 3, 5], 8);
        result.sort();
        let mut expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example_3_no_combination() {
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
    }
}
