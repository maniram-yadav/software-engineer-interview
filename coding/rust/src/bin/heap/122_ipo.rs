//! LeetCode Top Interview 150 — #122 IPO (Hard)
//!
//! Given k projects (with profits and required capital), starting
//! capital w, choose at most k projects to maximize final capital (each
//! finished project's profit is added to capital, enabling more
//! projects). Solved greedily: among all projects affordable with
//! current capital, always take the most profitable one, using a
//! min-heap of capital requirements to find newly-affordable projects
//! and a max-heap of profits to pick the best.
//!
//! Example:
//!   Input: k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
//!   Output: 4

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
        projects.sort_unstable_by_key(|&(c, _)| c);

        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut w = w;
        let mut idx = 0;

        for _ in 0..k {
            while idx < n && projects[idx].0 <= w {
                max_heap.push(projects[idx].1);
                idx += 1;
            }
            match max_heap.pop() {
                Some(p) => w += p,
                None => break,
            }
        }

        w
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }

    #[test]
    fn no_affordable_project() {
        assert_eq!(
            Solution::find_maximized_capital(1, 0, vec![5], vec![10]),
            0
        );
    }

    #[test]
    fn k_larger_than_project_count() {
        assert_eq!(
            Solution::find_maximized_capital(10, 0, vec![1, 2], vec![0, 0]),
            3
        );
    }
}
