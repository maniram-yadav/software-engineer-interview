//! LeetCode Top Interview 150 — #100 Word Search II (Hard)
//!
//! Given an m x n board of characters and a list of words, return all
//! words from the list that can be formed by a path of adjacent cells
//! (no cell reused within a single word). Solved by building a trie of
//! all target words, then DFS-backtracking from every board cell,
//! walking the trie alongside the board so shared prefixes are explored
//! only once. A found word is recorded and its trie leaf cleared so it's
//! never reported twice.
//!
//! Example:
//!   Input: board = [["o","a","a","n"],["e","t","a","e"],
//!                    ["i","h","k","r"],["i","f","l","v"]],
//!          words = ["oath","pea","eat","rain"]
//!   Output: ["eat","oath"]

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

struct Solution;

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::default();
        for w in &words {
            let mut node = &mut root;
            for c in w.chars() {
                node = node.children.entry(c).or_insert_with(TrieNode::default);
            }
            node.word = Some(w.clone());
        }

        let rows = board.len() as i32;
        let cols = board[0].len() as i32;
        let mut result = Vec::new();

        fn dfs(
            board: &mut Vec<Vec<char>>,
            r: i32,
            c: i32,
            rows: i32,
            cols: i32,
            node: &mut TrieNode,
            result: &mut Vec<String>,
        ) {
            if r < 0 || r >= rows || c < 0 || c >= cols {
                return;
            }
            let ch = board[r as usize][c as usize];
            if ch == '#' {
                return;
            }
            let next_node = match node.children.get_mut(&ch) {
                Some(n) => n,
                None => return,
            };
            if let Some(w) = next_node.word.take() {
                result.push(w);
            }

            board[r as usize][c as usize] = '#';
            dfs(board, r + 1, c, rows, cols, next_node, result);
            dfs(board, r - 1, c, rows, cols, next_node, result);
            dfs(board, r, c + 1, rows, cols, next_node, result);
            dfs(board, r, c - 1, rows, cols, next_node, result);
            board[r as usize][c as usize] = ch;
        }

        for r in 0..rows {
            for c in 0..cols {
                dfs(&mut board, r, c, rows, cols, &mut root, &mut result);
            }
        }

        result
    }
}

fn board_of(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.chars().collect()).collect()
}

fn v(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn main() {
    let board = board_of(&["oaan", "etae", "ihkr", "iflv"]);
    let mut result = Solution::find_words(board, v(&["oath", "pea", "eat", "rain"]));
    result.sort();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let board = board_of(&["oaan", "etae", "ihkr", "iflv"]);
        let mut result = Solution::find_words(board, v(&["oath", "pea", "eat", "rain"]));
        result.sort();
        assert_eq!(result, vec!["eat".to_string(), "oath".to_string()]);
    }

    #[test]
    fn example_2_no_matches() {
        let board = board_of(&["ab", "cd"]);
        let result = Solution::find_words(board, v(&["abcb"]));
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn single_letter_word() {
        let board = board_of(&["a"]);
        let result = Solution::find_words(board, v(&["a"]));
        assert_eq!(result, vec!["a".to_string()]);
    }

    #[test]
    fn word_not_reusing_cell() {
        // "aaa" would need the single 'a' cell three times, so this
        // should NOT be found — the path can't reuse a cell.
        let board = board_of(&["a"]);
        let result = Solution::find_words(board, v(&["aaa"]));
        assert_eq!(result, Vec::<String>::new());
    }
}
