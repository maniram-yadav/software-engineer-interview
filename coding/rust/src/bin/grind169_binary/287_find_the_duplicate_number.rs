//! Grind 169 — LeetCode #287 Find the Duplicate Number (Medium)
//!
//! Given an array of n + 1 integers where each value is in [1, n], and
//! exactly one number repeats, find the duplicate without modifying the
//! array and using O(1) extra space. Treat `nums[i] -> nums[nums[i]]` as
//! a linked-list "next" function; the duplicate value creates a cycle,
//! found with Floyd's tortoise-and-hare.
//!
//! Example:
//!   Input: nums = [1,3,4,2,2]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0];
        let mut fast = nums[0];
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }

        let mut ptr1 = nums[0];
        let mut ptr2 = slow;
        while ptr1 != ptr2 {
            ptr1 = nums[ptr1 as usize];
            ptr2 = nums[ptr2 as usize];
        }
        ptr1
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_duplicate(vec![1, 3, 4, 2, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

    #[test]
    fn duplicate_at_start() {
        assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 2]), 2);
    }

    #[test]
    fn small_input() {
        assert_eq!(Solution::find_duplicate(vec![1, 1]), 1);
    }
}
