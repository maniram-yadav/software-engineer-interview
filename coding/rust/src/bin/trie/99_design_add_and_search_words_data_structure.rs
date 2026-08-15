//! LeetCode Top Interview 150 — #99 Design Add and Search Words Data
//! Structure (Medium)
//!
//! Design a data structure supporting add_word(word) and search(word),
//! where search may contain '.' as a wildcard for any single letter.
//! Built on a trie; search recurses, and on '.' tries every child.
//!
//! Example:
//!   wd.addWord("bad");
//!   wd.search("b.d"); // true
//!   wd.search("bad");  // true
//!   wd.search("..d");  // true

use std::collections::HashMap;

struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    is_end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(WordDictionary::new);
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        Self::search_helper(self, &chars, 0)
    }

    fn search_helper(node: &WordDictionary, chars: &[char], idx: usize) -> bool {
        if idx == chars.len() {
            return node.is_end;
        }
        let c = chars[idx];
        if c == '.' {
            node.children
                .values()
                .any(|child| Self::search_helper(child, chars, idx + 1))
        } else {
            match node.children.get(&c) {
                Some(child) => Self::search_helper(child, chars, idx + 1),
                None => false,
            }
        }
    }
}

fn main() {
    let mut wd = WordDictionary::new();
    wd.add_word("bad".to_string());
    println!("{}", wd.search("b.d".to_string()));
    println!("{}", wd.search("bad".to_string()));
    println!("{}", wd.search("..d".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());
        assert_eq!(wd.search("pad".to_string()), false);
        assert_eq!(wd.search("bad".to_string()), true);
        assert_eq!(wd.search(".ad".to_string()), true);
        assert_eq!(wd.search("b..".to_string()), true);
    }

    #[test]
    fn all_wildcards() {
        let mut wd = WordDictionary::new();
        wd.add_word("abc".to_string());
        assert_eq!(wd.search("...".to_string()), true);
    }

    #[test]
    fn wrong_length_never_matches() {
        let mut wd = WordDictionary::new();
        wd.add_word("abc".to_string());
        assert_eq!(wd.search("ab".to_string()), false);
        assert_eq!(wd.search("..".to_string()), false);
    }

    #[test]
    fn search_empty_dictionary() {
        let wd = WordDictionary::new();
        assert_eq!(wd.search("a".to_string()), false);
    }
}
