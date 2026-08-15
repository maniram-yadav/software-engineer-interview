//! Grind 169 — LeetCode #41 First Missing Positive (Hard)
//!
//! Given an unsorted integer array nums, return the smallest missing
//! positive integer, in O(n) time and O(1) extra space. Solved with
//! index-as-hash-set placement: swap each value v (in 1..=n) into slot
//! v-1, then scan for the first slot whose value doesn't match its
//! expected 1-indexed position.
//!
//! Example:
//!   Input: nums = [3,4,-1,1]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            while nums[i] > 0 && nums[i] <= n && nums[(nums[i] - 1) as usize] != nums[i] {
                let target = (nums[i] - 1) as usize;
                nums.swap(i, target);
            }
        }
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        n + 1
    }
}

fn main() {
    println!(
        "{}",
        Solution::first_missing_positive(vec![3, 4, -1, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::first_missing_positive(vec![1, 2, 0]),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::first_missing_positive(vec![3, 4, -1, 1]),
            2
        );
    }

    #[test]
    fn example_3_consecutive_range() {
        assert_eq!(
            Solution::first_missing_positive(vec![7, 8, 9, 11, 12]),
            1
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::first_missing_positive(vec![]), 1);
    }
}
