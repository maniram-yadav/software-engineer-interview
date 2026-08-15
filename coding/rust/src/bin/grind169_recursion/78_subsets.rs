//! Grind 169 — LeetCode #78 Subsets (Medium)
//!
//! Given an integer array of unique elements, return all possible
//! subsets (the power set). Built iteratively: for each new number,
//! duplicate every existing subset with that number appended.
//!
//! Example:
//!   Input: nums = [1,2,3]
//!   Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![]];
        for n in nums {
            let new_subsets: Vec<Vec<i32>> = result
                .iter()
                .map(|s| {
                    let mut s2 = s.clone();
                    s2.push(n);
                    s2
                })
                .collect();
            result.extend(new_subsets);
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_has_all_eight_subsets() {
        let result = Solution::subsets(vec![1, 2, 3]);
        assert_eq!(result.len(), 8);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1, 2, 3]));
        assert!(result.contains(&vec![1]));
        assert!(result.contains(&vec![2, 3]));
    }

    #[test]
    fn example_2_single_element() {
        let mut result = Solution::subsets(vec![0]);
        result.sort();
        assert_eq!(result, vec![vec![], vec![0]]);
    }

    #[test]
    fn empty_input_has_only_empty_subset() {
        assert_eq!(Solution::subsets(vec![]), vec![Vec::<i32>::new()]);
    }

    #[test]
    fn all_subsets_are_unique() {
        let result = Solution::subsets(vec![1, 2, 3, 4]);
        assert_eq!(result.len(), 16);
        let mut sorted = result.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 16);
    }
}
