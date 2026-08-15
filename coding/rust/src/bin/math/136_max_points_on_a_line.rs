//! LeetCode Top Interview 150 — #136 Max Points on a Line (Hard)
//!
//! Given an array of points on the X-Y plane, return the maximum number
//! of points that lie on the same straight line. For each point, group
//! every other point by its direction vector from that point (reduced to
//! lowest terms via gcd, with a canonical sign), and the largest group
//! plus one (for the anchor point itself) is the best line through that
//! anchor.
//!
//! Example:
//!   Input: points = [[1,1],[2,2],[3,3]]
//!   Output: 3

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let n = points.len();
        if n <= 2 {
            return n as i32;
        }

        let mut best = 1;
        for i in 0..n {
            let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();
            for j in 0..n {
                if i == j {
                    continue;
                }
                let dx = points[j][0] - points[i][0];
                let dy = points[j][1] - points[i][1];
                let g = gcd(dx.abs(), dy.abs());
                let (mut ndx, mut ndy) = if g == 0 { (dx, dy) } else { (dx / g, dy / g) };
                if ndx < 0 || (ndx == 0 && ndy < 0) {
                    ndx = -ndx;
                    ndy = -ndy;
                }
                *slopes.entry((ndx, ndy)).or_insert(0) += 1;
            }
            let max_slope = slopes.values().max().copied().unwrap_or(0);
            best = best.max(max_slope + 1);
        }
        best
    }
}

fn main() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    println!("{}", Solution::max_points(points));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::max_points(points), 3);
    }

    #[test]
    fn example_2() {
        let points = vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ];
        assert_eq!(Solution::max_points(points), 4);
    }

    #[test]
    fn two_points_always_a_line() {
        let points = vec![vec![0, 0], vec![1, 1]];
        assert_eq!(Solution::max_points(points), 2);
    }

    #[test]
    fn vertical_line() {
        let points = vec![vec![1, 1], vec![1, 5], vec![1, -3], vec![2, 2]];
        assert_eq!(Solution::max_points(points), 3);
    }
}
