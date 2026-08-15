//! LeetCode Top Interview 150 — #6 Rotate Array (Medium)
//!
//! Rotate an array `nums` to the right by `k` steps, in place.
//! Solved with the "triple reverse" trick: reverse the whole array, then
//! reverse each of the two resulting segments.
//!
//! Example:
//!   Input: nums = [1,2,3,4,5,6,7], k = 3
//!   Output: [5,6,7,1,2,3,4]

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n == 0 {
            return;
        }
        let k = (k as usize) % n;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    println!("rotated: {:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn k_larger_than_len() {
        let mut nums = vec![1, 2, 3];
        Solution::rotate(&mut nums, 4);
        assert_eq!(nums, vec![3, 1, 2]);
    }

    #[test]
    fn single_element() {
        let mut nums = vec![1];
        Solution::rotate(&mut nums, 5);
        assert_eq!(nums, vec![1]);
    }
}
