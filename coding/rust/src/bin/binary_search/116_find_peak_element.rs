//! LeetCode Top Interview 150 — #116 Find Peak Element (Medium)
//!
//! Given an integer array nums where nums[i] != nums[i+1], find any peak
//! element (greater than both neighbors, treating out-of-bounds as -inf)
//! in O(log n). Solved by binary search toward the ascending side, which
//! always leads to a peak.
//!
//! Example:
//!   Input: nums = [1,2,3,1]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0usize, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] > nums[mid + 1] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }
}

fn is_peak(nums: &[i32], idx: usize) -> bool {
    let left_ok = idx == 0 || nums[idx - 1] < nums[idx];
    let right_ok = idx == nums.len() - 1 || nums[idx] > nums[idx + 1];
    left_ok && right_ok
}

fn main() {
    println!("{}", Solution::find_peak_element(vec![1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    }

    #[test]
    fn example_2_returns_a_valid_peak() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let idx = Solution::find_peak_element(nums.clone());
        assert!(is_peak(&nums, idx as usize));
    }

    #[test]
    fn single_element_is_peak() {
        assert_eq!(Solution::find_peak_element(vec![1]), 0);
    }

    #[test]
    fn strictly_decreasing_peak_at_start() {
        assert_eq!(Solution::find_peak_element(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn strictly_increasing_peak_at_end() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 4, 5]), 4);
    }
}
