//! Grind 169 — LeetCode #362 Design Hit Counter (Medium, Premium)
//!
//! Design a hit counter that counts hits received in the past 5 minutes
//! (300 seconds), supporting hit(timestamp) and getHits(timestamp), with
//! timestamps monotonically increasing. A queue of hit timestamps is
//! pruned from the front whenever it falls more than 300 seconds behind
//! the query timestamp.
//!
//! Example:
//!   hit(1); hit(2); hit(3);
//!   getHits(4); // 3
//!   hit(300);
//!   getHits(300); // 4
//!   getHits(301); // 3

use std::collections::VecDeque;

struct HitCounter {
    hits: VecDeque<i32>,
}

impl HitCounter {
    fn new() -> Self {
        HitCounter {
            hits: VecDeque::new(),
        }
    }

    fn hit(&mut self, timestamp: i32) {
        self.hits.push_back(timestamp);
    }

    fn get_hits(&mut self, timestamp: i32) -> i32 {
        while let Some(&front) = self.hits.front() {
            if timestamp - front >= 300 {
                self.hits.pop_front();
            } else {
                break;
            }
        }
        self.hits.len() as i32
    }
}

fn main() {
    let mut hc = HitCounter::new();
    hc.hit(1);
    hc.hit(2);
    hc.hit(3);
    println!("{}", hc.get_hits(4));
    hc.hit(300);
    println!("{}", hc.get_hits(300));
    println!("{}", hc.get_hits(301));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut hc = HitCounter::new();
        hc.hit(1);
        hc.hit(2);
        hc.hit(3);
        assert_eq!(hc.get_hits(4), 3);
        hc.hit(300);
        assert_eq!(hc.get_hits(300), 4);
        assert_eq!(hc.get_hits(301), 3);
    }

    #[test]
    fn no_hits_yet() {
        let mut hc = HitCounter::new();
        assert_eq!(hc.get_hits(1), 0);
    }

    #[test]
    fn all_hits_expired() {
        let mut hc = HitCounter::new();
        hc.hit(1);
        assert_eq!(hc.get_hits(302), 0);
    }

    #[test]
    fn hit_exactly_at_window_boundary() {
        let mut hc = HitCounter::new();
        hc.hit(1);
        assert_eq!(hc.get_hits(300), 0);
        assert_eq!(hc.get_hits(299), 1);
    }
}
