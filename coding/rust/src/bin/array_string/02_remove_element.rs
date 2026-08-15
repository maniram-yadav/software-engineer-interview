//! LeetCode Top Interview 150 — #2 Remove Element (Easy)
//!
//! Given an array `nums` and a value `val`, remove all occurrences of `val`
//! in place and return the new length. Order doesn't matter, and elements
//! beyond the returned length are ignored.
//!
//! Example:
//!   Input: nums = [3,2,2,3], val = 3
//!   Output: 2, nums = [2,2,_,_]

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let len = Solution::remove_element(&mut nums, 3);
    println!("new length: {}, nums: {:?}", len, &nums[..len as usize]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![3, 2, 2, 3];
        let len = Solution::remove_element(&mut nums, 3);
        assert_eq!(len, 2);
        assert_eq!(&nums[..len as usize], &[2, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let len = Solution::remove_element(&mut nums, 2);
        assert_eq!(len, 5);
        let mut kept: Vec<i32> = nums[..len as usize].to_vec();
        kept.sort();
        assert_eq!(kept, vec![0, 0, 1, 3, 4]);
    }

    #[test]
    fn no_match() {
        let mut nums = vec![1, 2, 3];
        let len = Solution::remove_element(&mut nums, 9);
        assert_eq!(len, 3);
        assert_eq!(&nums[..len as usize], &[1, 2, 3]);
    }

    #[test]
    fn empty_input() {
        let mut nums: Vec<i32> = vec![];
        let len = Solution::remove_element(&mut nums, 1);
        assert_eq!(len, 0);
    }
}
