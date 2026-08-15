//! LeetCode Top Interview 150 — #102 Combinations (Medium)
//!
//! Given two integers n and k, return all possible combinations of k
//! numbers chosen from 1..n.
//!
//! Example:
//!   Input: n = 4, k = 2
//!   Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrack(start: i32, n: i32, k: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if current.len() as i32 == k {
                result.push(current.clone());
                return;
            }
            for i in start..=n {
                current.push(i);
                backtrack(i + 1, n, k, current, result);
                current.pop();
            }
        }

        let mut result = Vec::new();
        let mut current = Vec::new();
        backtrack(1, n, k, &mut current, &mut result);
        result
    }
}

fn main() {
    println!("{:?}", Solution::combine(4, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
    }

    #[test]
    fn example_2_k_equals_one() {
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }

    #[test]
    fn k_equals_n() {
        assert_eq!(Solution::combine(3, 3), vec![vec![1, 2, 3]]);
    }
}
