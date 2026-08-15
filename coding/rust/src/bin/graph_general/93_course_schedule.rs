//! LeetCode Top Interview 150 — #93 Course Schedule (Medium)
//!
//! Given numCourses and prerequisite pairs [a, b] (must take b before a),
//! determine if it's possible to finish all courses. Solved with Kahn's
//! algorithm (BFS topological sort): all courses can finish iff every
//! node can be removed via repeatedly stripping zero-indegree nodes,
//! i.e. the prerequisite graph is acyclic.
//!
//! Example:
//!   Input: numCourses = 2, prerequisites = [[1,0]]
//!   Output: true

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut indegree = vec![0i32; n];

        for p in prerequisites {
            let (course, pre) = (p[0] as usize, p[1] as usize);
            adj[pre].push(course);
            indegree[course] += 1;
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
        let mut visited = 0;
        while let Some(cur) = queue.pop_front() {
            visited += 1;
            for &next in &adj[cur] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        visited == n
    }
}

fn main() {
    println!("{}", Solution::can_finish(2, vec![vec![1, 0]]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_possible() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn example_2_cycle() {
        assert_eq!(
            Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]),
            false
        );
    }

    #[test]
    fn no_prerequisites() {
        assert_eq!(Solution::can_finish(3, vec![]), true);
    }

    #[test]
    fn longer_cycle() {
        assert_eq!(
            Solution::can_finish(3, vec![vec![1, 0], vec![2, 1], vec![0, 2]]),
            false
        );
    }
}
