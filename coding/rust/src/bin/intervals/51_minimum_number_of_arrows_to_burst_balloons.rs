//! LeetCode Top Interview 150 — #51 Minimum Number of Arrows to Burst
//! Balloons (Medium)
//!
//! Balloons are represented as horizontal diameter intervals
//! [xstart, xend]. An arrow shot straight up at x bursts every balloon
//! whose interval contains x. Return the minimum number of arrows needed
//! to burst all balloons. Solved greedily: sort by end coordinate, shoot
//! an arrow at the end of the first unburst balloon, skip all balloons it
//! also bursts.
//!
//! Example:
//!   Input: points = [[10,16],[2,8],[1,6],[7,12]]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort_unstable_by_key(|p| p[1]);

        let mut arrows = 1;
        let mut end = points[0][1];
        for p in &points[1..] {
            if p[0] > end {
                arrows += 1;
                end = p[1];
            }
        }

        arrows
    }
}

fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    println!("{}", Solution::find_min_arrow_shots(points));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }

    #[test]
    fn example_2_no_overlap() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(Solution::find_min_arrow_shots(points), 4);
    }

    #[test]
    fn example_3_chained_overlap() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::find_min_arrow_shots(vec![]), 0);
    }

    #[test]
    fn single_balloon() {
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![5, 9]]), 1);
    }
}
