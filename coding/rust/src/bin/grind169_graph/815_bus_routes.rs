//! Grind 169 — LeetCode #815 Bus Routes (Hard)
//!
//! Given bus routes (each a list of stops) and a source/target stop,
//! return the minimum number of buses needed to travel from source to
//! target, or -1. Solved with BFS over stops, where taking any bus that
//! serves the current stop advances to all of that bus's other stops in
//! one hop; each bus route is expanded at most once (`visited_buses`).
//!
//! Example:
//!   Input: routes = [[1,2,7],[3,6,7]], source = 1, target = 6
//!   Output: 2

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut stop_to_buses: HashMap<i32, Vec<usize>> = HashMap::new();
        for (bus_idx, route) in routes.iter().enumerate() {
            for &stop in route {
                stop_to_buses.entry(stop).or_insert_with(Vec::new).push(bus_idx);
            }
        }

        let mut visited_buses: HashSet<usize> = HashSet::new();
        let mut visited_stops: HashSet<i32> = HashSet::new();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((source, 0));
        visited_stops.insert(source);

        while let Some((stop, buses)) = queue.pop_front() {
            if let Some(bus_list) = stop_to_buses.get(&stop) {
                for &bus in bus_list {
                    if visited_buses.contains(&bus) {
                        continue;
                    }
                    visited_buses.insert(bus);
                    for &next_stop in &routes[bus] {
                        if next_stop == target {
                            return buses + 1;
                        }
                        if !visited_stops.contains(&next_stop) {
                            visited_stops.insert(next_stop);
                            queue.push_back((next_stop, buses + 1));
                        }
                    }
                }
            }
        }
        -1
    }
}

fn main() {
    let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
    println!(
        "{}",
        Solution::num_buses_to_destination(routes, 1, 6)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 2);
    }

    #[test]
    fn example_2_unreachable() {
        let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        assert_eq!(Solution::num_buses_to_destination(routes, 1, 5), -1);
    }

    #[test]
    fn source_equals_target() {
        let routes = vec![vec![1, 2]];
        assert_eq!(Solution::num_buses_to_destination(routes, 1, 1), 0);
    }

    #[test]
    fn direct_single_bus() {
        let routes = vec![vec![1, 2, 3]];
        assert_eq!(Solution::num_buses_to_destination(routes, 1, 3), 1);
    }
}
