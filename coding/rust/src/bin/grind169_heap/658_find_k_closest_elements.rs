//! Grind 169 — LeetCode #658 Find K Closest Elements (Medium)
//!
//! Given a sorted integer array arr, and integers k and x, return the k
//! closest integers to x in the array, sorted ascending. Binary search
//! for the optimal window's left boundary: at each candidate window
//! [mid, mid+k], compare how far x is from each end to decide which way
//! to shift.
//!
//! Example:
//!   Input: arr = [1,2,3,4,5], k = 4, x = 3
//!   Output: [1,2,3,4]

struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as i32;
        let mut lo = 0i32;
        let mut hi = arr.len() as i32 - k;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if x - arr[mid as usize] > arr[(mid + k) as usize] - x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        arr[lo as usize..(lo + k) as usize].to_vec()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn example_2_x_below_range() {
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn x_above_range() {
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 2, 100),
            vec![4, 5]
        );
    }

    #[test]
    fn k_equals_length_returns_all() {
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3], 3, 2),
            vec![1, 2, 3]
        );
    }
}
