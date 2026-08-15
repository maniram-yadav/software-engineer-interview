//! LeetCode Top Interview 150 — #103 Permutations (Medium)
//!
//! Given an array of distinct integers, return all possible permutations.
//!
//! Example:
//!   Input: nums = [1,2,3]
//!   Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            nums: &[i32],
            used: &mut Vec<bool>,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == nums.len() {
                result.push(current.clone());
                return;
            }
            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                used[i] = true;
                current.push(nums[i]);
                backtrack(nums, used, current, result);
                current.pop();
                used[i] = false;
            }
        }

        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut used = vec![false; nums.len()];
        backtrack(&nums, &mut used, &mut current, &mut result);
        result
    }
}

fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = Solution::permute(vec![1, 2, 3]);
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![1, 2, 3]));
        assert!(result.contains(&vec![3, 2, 1]));
    }

    #[test]
    fn example_2_two_elements() {
        let mut result = Solution::permute(vec![0, 1]);
        result.sort();
        assert_eq!(result, vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn example_3_single_element() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn all_permutations_are_unique() {
        let result = Solution::permute(vec![1, 2, 3, 4]);
        assert_eq!(result.len(), 24);
        let mut sorted = result.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 24);
    }
}
