//! Grind 169 — LeetCode #1197 Minimum Knight Moves (Medium)
//!
//! On an infinite chessboard, a knight starts at (0,0). Return the
//! minimum number of moves to reach (x, y). By symmetry, only the
//! first-quadrant target |x|, |y| needs solving; BFS is bounded to a
//! small margin beyond the target since going too far negative/past the
//! target is never optimal.
//!
//! Example:
//!   Input: x = 2, y = 1
//!   Output: 1

use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        let x = x.abs();
        let y = y.abs();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
        queue.push_back((0, 0, 0));
        visited.insert((0, 0));

        let moves = [
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, -1),
            (-1, -2),
            (-2, -1),
        ];

        while let Some((cx, cy, steps)) = queue.pop_front() {
            if cx == x && cy == y {
                return steps;
            }
            for &(dx, dy) in &moves {
                let (nx, ny) = (cx + dx, cy + dy);
                if nx >= -2 && ny >= -2 && nx <= x + 2 && ny <= y + 2 && !visited.contains(&(nx, ny))
                {
                    visited.insert((nx, ny));
                    queue.push_back((nx, ny, steps + 1));
                }
            }
        }
        -1
    }
}

fn main() {
    println!("{}", Solution::min_knight_moves(2, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::min_knight_moves(2, 1), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::min_knight_moves(5, 5), 4);
    }

    #[test]
    fn origin_needs_no_moves() {
        assert_eq!(Solution::min_knight_moves(0, 0), 0);
    }

    #[test]
    fn negative_coordinates_by_symmetry() {
        assert_eq!(Solution::min_knight_moves(-2, -1), 1);
    }
}
