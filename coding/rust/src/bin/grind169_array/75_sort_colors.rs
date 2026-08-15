//! Grind 169 — LeetCode #75 Sort Colors (Medium)
//!
//! Given an array with n objects colored red, white, or blue
//! (represented as 0, 1, 2), sort them in place so objects of the same
//! color are adjacent, in the order red, white, blue. Solved with the
//! Dutch national flag three-way partition, one pass.
//!
//! Example:
//!   Input: nums = [2,0,2,1,1,0]
//!   Output: [0,0,1,1,2,2]

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0i32;
        let mut mid = 0i32;
        let mut high = nums.len() as i32 - 1;

        while mid <= high {
            match nums[mid as usize] {
                0 => {
                    nums.swap(low as usize, mid as usize);
                    low += 1;
                    mid += 1;
                }
                1 => mid += 1,
                _ => {
                    nums.swap(mid as usize, high as usize);
                    high -= 1;
                }
            }
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn already_sorted() {
        let mut nums = vec![0, 1, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn single_color() {
        let mut nums = vec![1, 1, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1, 1, 1]);
    }
}
