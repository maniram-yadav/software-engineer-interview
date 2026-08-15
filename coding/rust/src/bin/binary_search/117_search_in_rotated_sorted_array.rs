//! LeetCode Top Interview 150 — #117 Search in Rotated Sorted Array (Medium)
//!
//! Given a sorted array that has been rotated at an unknown pivot, and a
//! target, search for the target in O(log n) and return its index, or -1.
//! Solved by determining which half of the current window is sorted at
//! each step, then checking if the target lies within that sorted half.
//!
//! Example:
//!   Input: nums = [4,5,6,7,0,1,2], target = 0
//!   Output: 4

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len() as i32;
        let (mut lo, mut hi) = (0i32, n - 1);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[lo as usize] <= nums[mid as usize] {
                if nums[lo as usize] <= target && target < nums[mid as usize] {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[hi as usize] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }
        -1
    }
}

fn main() {
    println!(
        "{}",
        Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_found() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn example_2_not_found() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn example_3_single_element_not_found() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn no_rotation() {
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 3), 2);
    }

    #[test]
    fn target_at_pivot() {
        assert_eq!(Solution::search(vec![6, 7, 0, 1, 2, 4, 5], 0), 2);
    }
}
