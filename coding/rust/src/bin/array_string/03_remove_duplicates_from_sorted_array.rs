//! LeetCode Top Interview 150 — #3 Remove Duplicates from Sorted Array (Easy)
//!
//! Given a sorted array `nums`, remove duplicates in place so each unique
//! element appears once, keeping relative order, and return the new length.
//!
//! Example:
//!   Input: nums = [0,0,1,1,1,2,2,3,3,4]
//!   Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut k = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[k - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let len = Solution::remove_duplicates(&mut nums);
    println!("new length: {}, nums: {:?}", len, &nums[..len as usize]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 2);
        assert_eq!(&nums[..len as usize], &[1, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 5);
        assert_eq!(&nums[..len as usize], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn all_unique() {
        let mut nums = vec![1, 2, 3, 4];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 4);
        assert_eq!(&nums[..len as usize], &[1, 2, 3, 4]);
    }

    #[test]
    fn empty_input() {
        let mut nums: Vec<i32> = vec![];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 0);
    }
}
