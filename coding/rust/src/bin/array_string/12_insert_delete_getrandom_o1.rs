//! LeetCode Top Interview 150 — #12 Insert Delete GetRandom O(1) (Medium)
//!
//! Design a data structure supporting `insert(val)`, `remove(val)`, and
//! `get_random()` (returns a random existing element with equal
//! probability), all in average O(1) time. Backed by a Vec (for O(1)
//! random indexing) plus a HashMap from value to its index in the Vec
//! (for O(1) lookup/removal via swap-with-last).

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct RandomizedSet {
    data: Vec<i32>,
    index: HashMap<i32, usize>,
    rng_state: u64,
}

impl RandomizedSet {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
            | 1;
        RandomizedSet {
            data: Vec::new(),
            index: HashMap::new(),
            rng_state: seed,
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.index.contains_key(&val) {
            return false;
        }
        self.index.insert(val, self.data.len());
        self.data.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.index.get(&val) {
            let last_idx = self.data.len() - 1;
            let last_val = self.data[last_idx];
            self.data.swap(idx, last_idx);
            self.index.insert(last_val, idx);
            self.data.pop();
            self.index.remove(&val);
            true
        } else {
            false
        }
    }

    // xorshift64: a tiny, dependency-free PRNG (good enough for picking a
    // uniformly random index; not cryptographically secure).
    fn get_random(&mut self) -> i32 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        let idx = (self.rng_state as usize) % self.data.len();
        self.data[idx]
    }
}

fn main() {
    let mut rs = RandomizedSet::new();
    println!("insert(1): {}", rs.insert(1));
    println!("remove(2): {}", rs.remove(2));
    println!("insert(2): {}", rs.insert(2));
    println!("get_random(): {}", rs.get_random());
    println!("remove(1): {}", rs.remove(1));
    println!("insert(2): {}", rs.insert(2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut rs = RandomizedSet::new();
        assert_eq!(rs.insert(1), true);
        assert_eq!(rs.remove(2), false);
        assert_eq!(rs.insert(2), true);
        assert!(rs.data.contains(&rs.get_random()));
        assert_eq!(rs.remove(1), true);
        assert_eq!(rs.insert(2), false);
    }

    #[test]
    fn duplicate_insert_fails() {
        let mut rs = RandomizedSet::new();
        assert_eq!(rs.insert(5), true);
        assert_eq!(rs.insert(5), false);
    }

    #[test]
    fn remove_missing_fails() {
        let mut rs = RandomizedSet::new();
        assert_eq!(rs.remove(42), false);
    }

    #[test]
    fn get_random_always_in_set() {
        let mut rs = RandomizedSet::new();
        for v in [10, 20, 30, 40] {
            rs.insert(v);
        }
        for _ in 0..50 {
            let picked = rs.get_random();
            assert!(rs.data.contains(&picked));
        }
    }

    #[test]
    fn remove_middle_keeps_others() {
        let mut rs = RandomizedSet::new();
        rs.insert(1);
        rs.insert(2);
        rs.insert(3);
        assert_eq!(rs.remove(2), true);
        let mut remaining = rs.data.clone();
        remaining.sort();
        assert_eq!(remaining, vec![1, 3]);
    }
}
