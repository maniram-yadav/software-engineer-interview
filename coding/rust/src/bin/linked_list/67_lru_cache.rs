//! LeetCode Top Interview 150 — #67 LRU Cache (Medium)
//!
//! Design a Least Recently Used (LRU) cache with get(key) and
//! put(key, value), evicting the least recently used entry when capacity
//! is exceeded. Backed by a HashMap for values plus a Vec tracking access
//! order (front = least recently used, back = most recently used).
//!
//! Example:
//!   LRUCache cache = new LRUCache(2);
//!   cache.put(1,1); cache.put(2,2);
//!   cache.get(1);       // 1
//!   cache.put(3,3);     // evicts key 2
//!   cache.get(2);       // -1 (not found)

use std::collections::HashMap;

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    order: Vec<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            order: Vec::new(),
        }
    }

    fn touch(&mut self, key: i32) {
        if let Some(pos) = self.order.iter().position(|&k| k == key) {
            self.order.remove(pos);
        }
        self.order.push(key);
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&val) = self.map.get(&key) {
            self.touch(key);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
            self.touch(key);
            return;
        }
        if self.map.len() >= self.capacity && !self.order.is_empty() {
            let lru_key = self.order.remove(0);
            self.map.remove(&lru_key);
        }
        self.map.insert(key, value);
        self.order.push(key);
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("get(1): {}", cache.get(1));
    cache.put(3, 3);
    println!("get(2): {}", cache.get(2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn get_missing_key() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(1), -1);
    }

    #[test]
    fn put_updates_existing_key_without_eviction() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        cache.put(1, 10);
        assert_eq!(cache.get(1), 10);
    }

    #[test]
    fn capacity_one_evicts_immediately() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }
}
