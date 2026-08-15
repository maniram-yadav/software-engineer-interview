//! LeetCode Top Interview 150 — #120 Median of Two Sorted Arrays (Hard)
//!
//! Given two sorted arrays nums1 and nums2 of size m and n, return the
//! median of the combined sorted array in O(log(m+n)) time. Solved by
//! binary searching a partition point in the smaller array such that
//! every element on the "left" side of the combined partition is <=
//! every element on the "right" side.
//!
//! Example:
//!   Input: nums1 = [1,3], nums2 = [2]
//!   Output: 2.0

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };
        let (m, n) = (a.len(), b.len());
        let half = (m + n + 1) / 2;
        let (mut lo, mut hi) = (0i32, m as i32);

        while lo <= hi {
            let i = (lo + hi) / 2;
            let j = half as i32 - i;

            let a_left = if i == 0 { i32::MIN } else { a[(i - 1) as usize] };
            let a_right = if i == m as i32 { i32::MAX } else { a[i as usize] };
            let b_left = if j == 0 { i32::MIN } else { b[(j - 1) as usize] };
            let b_right = if j == n as i32 { i32::MAX } else { b[j as usize] };

            if a_left <= b_right && b_left <= a_right {
                return if (m + n) % 2 == 1 {
                    a_left.max(b_left) as f64
                } else {
                    (a_left.max(b_left) as f64 + a_right.min(b_right) as f64) / 2.0
                };
            } else if a_left > b_right {
                hi = i - 1;
            } else {
                lo = i + 1;
            }
        }

        unreachable!("input arrays are assumed sorted, per problem constraints")
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn one_array_empty() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![1]),
            1.0
        );
    }

    #[test]
    fn disjoint_ranges() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6]),
            3.5
        );
    }
}
