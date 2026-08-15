//! Grind 169 — LeetCode #704 Binary Search (Easy)
//!
//! Given a sorted array of unique integers and a target, return its
//! index using binary search, or -1 if absent.
//!
//! Example:
//!   Input: nums = [-1,0,3,5,9,12], target = 9
//!   Output: 4

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        -1
    }
}

fn main() {
    println!(
        "{}",
        Solution::search(vec![-1, 0, 3, 5, 9, 12], 9)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_found() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn example_2_not_found() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn single_element_found() {
        assert_eq!(Solution::search(vec![5], 5), 0);
    }

    #[test]
    fn empty_array() {
        assert_eq!(Solution::search(vec![], 1), -1);
    }
}
