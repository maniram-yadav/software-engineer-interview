//! Grind 169 — LeetCode #528 Random Pick with Weight (Medium)
//!
//! Given an array w of positive weights, design a structure that picks
//! an index i with probability proportional to w[i]. Solved with a
//! prefix-sum array and binary search: pick a uniformly random point in
//! [1, total], then find the first prefix sum >= that point. Uses a
//! small dependency-free xorshift PRNG since no `rand` crate is
//! available offline.
//!
//! Example:
//!   Input: w = [1,3]
//!   Output: pickIndex() returns 0 with probability 1/4, 1 with probability 3/4

struct Solution {
    prefix: Vec<i32>,
    total: i32,
    rng_state: u64,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        let mut sum = 0;
        for x in &w {
            sum += x;
            prefix.push(sum);
        }
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
            | 1;
        Solution {
            prefix,
            total: sum,
            rng_state: seed,
        }
    }

    fn pick_index(&mut self) -> i32 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        let target = (self.rng_state % self.total as u64) as i32 + 1;
        let pos = self.prefix.partition_point(|&p| p < target);
        pos as i32
    }
}

fn main() {
    let mut sol = Solution::new(vec![1, 3]);
    println!("{}", sol.pick_index());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_index_always_in_bounds() {
        let mut sol = Solution::new(vec![1, 3, 2]);
        for _ in 0..200 {
            let idx = sol.pick_index();
            assert!(idx >= 0 && idx < 3);
        }
    }

    #[test]
    fn single_weight_always_picks_zero() {
        let mut sol = Solution::new(vec![5]);
        for _ in 0..20 {
            assert_eq!(sol.pick_index(), 0);
        }
    }

    #[test]
    fn zero_weight_index_is_never_picked() {
        let mut sol = Solution::new(vec![0, 1, 0]);
        for _ in 0..200 {
            assert_eq!(sol.pick_index(), 1);
        }
    }

    #[test]
    fn distribution_favors_higher_weight() {
        let mut sol = Solution::new(vec![1, 99]);
        let mut count_one = 0;
        let trials = 500;
        for _ in 0..trials {
            if sol.pick_index() == 1 {
                count_one += 1;
            }
        }
        // Index 1 has 99% weight; expect it to dominate heavily.
        assert!(count_one > trials / 2);
    }
}
