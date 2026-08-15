//! LeetCode Top Interview 150 — #114 Search Insert Position (Easy)
//!
//! Given a sorted array of distinct integers and a target, return the
//! index if found; otherwise the index where it would be inserted, in
//! order. O(log n).
//!
//! Example:
//!   Input: nums = [1,3,5,6], target = 5
//!   Output: 2

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
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
        lo
    }
}

fn main() {
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_found() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn example_2_insert_in_middle() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn example_3_insert_at_end() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn insert_at_start() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
