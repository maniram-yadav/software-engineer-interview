//! LeetCode Top Interview 150 — #118 Find First and Last Position of
//! Element in Sorted Array (Medium)
//!
//! Given a sorted array of integers and a target, find the starting and
//! ending index of the target's occurrences in O(log n). Return [-1,-1]
//! if not found. Solved with two binary searches for lower_bound(target)
//! and lower_bound(target + 1).
//!
//! Example:
//!   Input: nums = [5,7,7,8,8,10], target = 8
//!   Output: [3,4]

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn lower_bound(nums: &[i32], target: i32) -> i32 {
            let (mut lo, mut hi) = (0i32, nums.len() as i32);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid as usize] < target {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        }

        let left = lower_bound(&nums, target);
        if left == nums.len() as i32 || nums[left as usize] != target {
            return vec![-1, -1];
        }
        let right = lower_bound(&nums, target + 1) - 1;
        vec![left, right]
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }

    #[test]
    fn example_2_not_found() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }

    #[test]
    fn example_3_empty_array() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }

    #[test]
    fn single_occurrence() {
        assert_eq!(Solution::search_range(vec![1, 2, 3], 2), vec![1, 1]);
    }

    #[test]
    fn all_same_value() {
        assert_eq!(Solution::search_range(vec![2, 2, 2], 2), vec![0, 2]);
    }
}
