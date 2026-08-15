//! LeetCode Top Interview 150 — #98 Implement Trie (Prefix Tree) (Medium)
//!
//! Implement a trie with insert(word), search(word) (exact match), and
//! starts_with(prefix). Each node holds a HashMap of children keyed by
//! character plus an is_end flag.
//!
//! Example:
//!   trie.insert("apple");
//!   trie.search("apple");   // true
//!   trie.search("app");     // false
//!   trie.startsWith("app"); // true

use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(Trie::new);
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.find(&word).map_or(false, |n| n.is_end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix).is_some()
    }

    fn find(&self, s: &str) -> Option<&Trie> {
        let mut node = self;
        for c in s.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return None,
            }
        }
        Some(node)
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    println!("{}", trie.search("apple".to_string()));
    println!("{}", trie.search("app".to_string()));
    println!("{}", trie.starts_with("app".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
    }

    #[test]
    fn search_nonexistent_word() {
        let mut trie = Trie::new();
        trie.insert("cat".to_string());
        assert_eq!(trie.search("car".to_string()), false);
        assert_eq!(trie.starts_with("ca".to_string()), true);
    }

    #[test]
    fn empty_prefix_always_matches() {
        let mut trie = Trie::new();
        trie.insert("x".to_string());
        assert_eq!(trie.starts_with("".to_string()), true);
    }

    #[test]
    fn search_empty_word() {
        let trie = Trie::new();
        assert_eq!(trie.search("".to_string()), false);
    }
}
