//! LeetCode Top Interview 150 — #4 Remove Duplicates from Sorted Array II (Medium)
//!
//! Given a sorted array `nums`, remove duplicates in place so each unique
//! element appears at most twice, keeping relative order, and return the
//! new length.
//!
//! Example:
//!   Input: nums = [1,1,1,2,2,3]
//!   Output: 5, nums = [1,1,2,2,3,_]

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }
        let mut k = 2;
        for i in 2..n {
            if nums[i] != nums[k - 2] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let len = Solution::remove_duplicates(&mut nums);
    println!("new length: {}, nums: {:?}", len, &nums[..len as usize]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 5);
        assert_eq!(&nums[..len as usize], &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 7);
        assert_eq!(&nums[..len as usize], &[0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn short_input() {
        let mut nums = vec![1];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 1);
        assert_eq!(&nums[..len as usize], &[1]);
    }

    #[test]
    fn empty_input() {
        let mut nums: Vec<i32> = vec![];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 0);
    }
}
