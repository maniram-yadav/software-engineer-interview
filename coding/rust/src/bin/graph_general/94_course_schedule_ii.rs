//! LeetCode Top Interview 150 — #94 Course Schedule II (Medium)
//!
//! Same setup as Course Schedule, but return a valid course order to
//! finish all courses, or an empty array if impossible. Solved with
//! Kahn's algorithm, recording the removal order.
//!
//! Example:
//!   Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
//!   Output: [0,1,2,3]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut indegree = vec![0i32; n];

        for p in prerequisites {
            let (course, pre) = (p[0] as usize, p[1] as usize);
            adj[pre].push(course);
            indegree[course] += 1;
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
        let mut order = Vec::new();
        while let Some(cur) = queue.pop_front() {
            order.push(cur as i32);
            for &next in &adj[cur] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if order.len() == n {
            order
        } else {
            vec![]
        }
    }
}

fn main() {
    let order = Solution::find_order(
        4,
        vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]],
    );
    println!("{:?}", order);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let order = Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn example_2_single_prerequisite() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    }

    #[test]
    fn example_3_no_prerequisites() {
        assert_eq!(Solution::find_order(1, vec![]), vec![0]);
    }

    #[test]
    fn cycle_returns_empty() {
        assert_eq!(
            Solution::find_order(2, vec![vec![1, 0], vec![0, 1]]),
            Vec::<i32>::new()
        );
    }
}
