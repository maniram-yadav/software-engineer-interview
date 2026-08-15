//! Grind 169 — LeetCode #981 Time Based Key-Value Store (Medium)
//!
//! Design a time-based key-value store: set(key, value, timestamp)
//! stores the value, and get(key, timestamp) returns the value set at
//! the largest timestamp <= the given timestamp. Values for a key are
//! appended in increasing timestamp order (as guaranteed by the
//! problem), so binary search finds the latest entry not after the
//! query timestamp.
//!
//! Example:
//!   set("foo","bar",1);
//!   get("foo",1); // "bar"
//!   get("foo",3); // "bar"
//!   set("foo","bar2",4);
//!   get("foo",4); // "bar2"

use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_insert_with(Vec::new).push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let entries = match self.store.get(&key) {
            Some(e) => e,
            None => return String::new(),
        };

        let mut lo = 0i32;
        let mut hi = entries.len() as i32 - 1;
        let mut result = String::new();
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if entries[mid as usize].0 <= timestamp {
                result = entries[mid as usize].1.clone();
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        result
    }
}

fn main() {
    let mut tkv = TimeMap::new();
    tkv.set("foo".to_string(), "bar".to_string(), 1);
    println!("{}", tkv.get("foo".to_string(), 1));
    println!("{}", tkv.get("foo".to_string(), 3));
    tkv.set("foo".to_string(), "bar2".to_string(), 4);
    println!("{}", tkv.get("foo".to_string(), 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut tkv = TimeMap::new();
        tkv.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(tkv.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(tkv.get("foo".to_string(), 3), "bar".to_string());
        tkv.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(tkv.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(tkv.get("foo".to_string(), 5), "bar2".to_string());
    }

    #[test]
    fn get_before_any_set_timestamp() {
        let mut tkv = TimeMap::new();
        tkv.set("foo".to_string(), "bar".to_string(), 5);
        assert_eq!(tkv.get("foo".to_string(), 1), String::new());
    }

    #[test]
    fn get_missing_key() {
        let tkv = TimeMap::new();
        assert_eq!(tkv.get("missing".to_string(), 1), String::new());
    }

    #[test]
    fn multiple_updates_same_key() {
        let mut tkv = TimeMap::new();
        tkv.set("k".to_string(), "v1".to_string(), 1);
        tkv.set("k".to_string(), "v2".to_string(), 2);
        tkv.set("k".to_string(), "v3".to_string(), 3);
        assert_eq!(tkv.get("k".to_string(), 2), "v2".to_string());
    }
}
