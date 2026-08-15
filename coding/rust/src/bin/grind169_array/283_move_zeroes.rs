//! Grind 169 — LeetCode #283 Move Zeroes (Easy)
//!
//! Given an integer array nums, move all zeroes to the end while
//! maintaining the relative order of non-zero elements, in place.
//!
//! Example:
//!   Input: nums = [0,1,0,3,12]
//!   Output: [1,3,12,0,0]

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut insert_pos = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(insert_pos, i);
                insert_pos += 1;
            }
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn example_2_single_zero() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn no_zeroes() {
        let mut nums = vec![1, 2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn all_zeroes() {
        let mut nums = vec![0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0]);
    }
}
