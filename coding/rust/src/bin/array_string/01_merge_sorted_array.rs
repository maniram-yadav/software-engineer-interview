//! LeetCode Top Interview 150 — #1 Merge Sorted Array (Easy)
//!
//! You're given two sorted integer arrays `nums1` and `nums2`, where `nums1`
//! has extra trailing space (length m + n) to hold all elements. Merge
//! `nums2` into `nums1` in place so the result is a single sorted array.
//!
//! Example:
//!   Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
//!   Output: [1,2,2,3,5,6]

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let mut i = m as isize - 1;
        let mut j = n as isize - 1;
        let mut k = (m + n) as isize - 1;

        while j >= 0 {
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    println!("merged: {:?}", nums1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn nums2_empty() {
        let mut nums1 = vec![1];
        let mut nums2: Vec<i32> = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn nums1_starts_all_zero() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn interleaved() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let mut nums2 = vec![1, 2, 3];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
