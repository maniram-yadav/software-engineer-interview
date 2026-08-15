//! LeetCode Top Interview 150 — #119 Find Minimum in Rotated Sorted
//! Array (Medium)
//!
//! Given a rotated sorted array of unique elements, find the minimum
//! element in O(log n). Solved by comparing the middle element against
//! the right boundary to decide which half contains the rotation point.
//!
//! Example:
//!   Input: nums = [3,4,5,1,2]
//!   Output: 1

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0usize, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] > nums[hi] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }
}

fn main() {
    println!("{}", Solution::find_min(vec![3, 4, 5, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn example_3_no_rotation() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::find_min(vec![5]), 5);
    }

    #[test]
    fn rotation_at_end() {
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
    }
}
