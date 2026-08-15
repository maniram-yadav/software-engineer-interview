//! Grind 169 — LeetCode #787 Cheapest Flights Within K Stops (Medium)
//!
//! Given n cities connected by flights with costs, find the cheapest
//! price from src to dst using at most k stops, or -1 if no such route
//! exists. Solved with Bellman-Ford limited to k+1 relaxation rounds
//! (each round allows one more edge/stop), relaxing from a snapshot of
//! the previous round so costs don't chain within the same round.
//!
//! Example:
//!   Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]],
//!          src = 0, dst = 3, k = 1
//!   Output: 700

struct Solution;

impl Solution {
    pub fn find_cheapest_price(
        n: i32,
        flights: Vec<Vec<i32>>,
        src: i32,
        dst: i32,
        k: i32,
    ) -> i32 {
        let n = n as usize;
        let mut dist = vec![i32::MAX; n];
        dist[src as usize] = 0;

        for _ in 0..=k {
            let mut new_dist = dist.clone();
            for f in &flights {
                let (u, v, w) = (f[0] as usize, f[1] as usize, f[2]);
                if dist[u] != i32::MAX && dist[u] + w < new_dist[v] {
                    new_dist[v] = dist[u] + w;
                }
            }
            dist = new_dist;
        }

        if dist[dst as usize] == i32::MAX {
            -1
        } else {
            dist[dst as usize]
        }
    }
}

fn main() {
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    println!(
        "{}",
        Solution::find_cheapest_price(4, flights, 0, 3, 1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        assert_eq!(
            Solution::find_cheapest_price(4, flights, 0, 3, 1),
            700
        );
    }

    #[test]
    fn example_2_more_stops_allowed() {
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        assert_eq!(
            Solution::find_cheapest_price(4, flights, 0, 3, 2),
            400
        );
    }

    #[test]
    fn example_3_no_route() {
        let flights = vec![vec![0, 1, 100]];
        assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 1), -1);
    }

    #[test]
    fn same_source_and_destination() {
        assert_eq!(Solution::find_cheapest_price(2, vec![], 0, 0, 0), 0);
    }
}
