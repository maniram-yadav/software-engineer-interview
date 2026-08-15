//! Grind 169 — LeetCode #31 Next Permutation (Medium)
//!
//! Given an array of integers representing a permutation, rearrange it
//! in place to the next lexicographically greater permutation; if none
//! exists, rearrange to the lowest order. Find the rightmost ascent,
//! swap it with the smallest element to its right that's still larger,
//! then reverse the suffix to make it ascending (the smallest possible
//! arrangement of that suffix).
//!
//! Example:
//!   Input: nums = [1,2,3]
//!   Output: [1,3,2]

struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n < 2 {
            return;
        }
        let mut i = n as i32 - 2;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }
        if i >= 0 {
            let mut j = n as i32 - 1;
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j as usize);
        }
        nums[(i + 1) as usize..].reverse();
    }
}

fn main() {
    let mut nums = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    println!("{:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn example_2_wraps_to_lowest_order() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn example_3_with_duplicates() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }

    #[test]
    fn single_element_unchanged() {
        let mut nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}
