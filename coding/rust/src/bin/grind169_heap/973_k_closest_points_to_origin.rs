//! Grind 169 — LeetCode #973 K Closest Points to Origin (Medium)
//!
//! Given an array of points on the X-Y plane and an integer k, return
//! the k closest points to the origin (any order). Solved with a
//! max-heap of size k on squared distance: push every point, and pop the
//! farthest whenever the heap exceeds size k, leaving the k closest.
//!
//! Example:
//!   Input: points = [[1,3],[-2,2]], k = 1
//!   Output: [[-2,2]]

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        for (i, p) in points.iter().enumerate() {
            let dist = (p[0] as i64).pow(2) + (p[1] as i64).pow(2);
            heap.push((dist, i));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.into_iter().map(|(_, i)| points[i].clone()).collect()
    }
}

fn normalize(mut points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    points.sort();
    points
}

fn main() {
    let points = vec![vec![1, 3], vec![-2, 2]];
    println!("{:?}", Solution::k_closest(points, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        assert_eq!(Solution::k_closest(points, 1), vec![vec![-2, 2]]);
    }

    #[test]
    fn example_2() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let result = normalize(Solution::k_closest(points, 2));
        assert_eq!(result, normalize(vec![vec![3, 3], vec![-2, 4]]));
    }

    #[test]
    fn k_equals_length_returns_all() {
        let points = vec![vec![1, 1], vec![2, 2]];
        let result = normalize(Solution::k_closest(points, 2));
        assert_eq!(result, normalize(vec![vec![1, 1], vec![2, 2]]));
    }

    #[test]
    fn single_point() {
        assert_eq!(
            Solution::k_closest(vec![vec![0, 0]], 1),
            vec![vec![0, 0]]
        );
    }
}
