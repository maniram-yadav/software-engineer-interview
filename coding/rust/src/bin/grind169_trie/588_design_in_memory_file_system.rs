//! Grind 169 — LeetCode #588 Design In-Memory File System (Hard)
//!
//! Design an in-memory file system supporting ls, mkdir,
//! addContentToFile, and readContentFromFile, mimicking Unix-style
//! paths. Backed by a trie of path segments, where each node is either a
//! directory (with named children) or a file (with content).
//!
//! Example:
//!   mkdir("/a/b/c");
//!   addContentToFile("/a/b/c/d","hello");
//!   ls("/");        // ["a"]
//!   readContentFromFile("/a/b/c/d"); // "hello"

use std::collections::HashMap;

#[derive(Default)]
struct FsNode {
    is_file: bool,
    content: String,
    children: HashMap<String, FsNode>,
}

struct FileSystem {
    root: FsNode,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            root: FsNode::default(),
        }
    }

    fn parse_path(path: &str) -> Vec<&str> {
        path.split('/').filter(|s| !s.is_empty()).collect()
    }

    fn ls(&self, path: String) -> Vec<String> {
        let parts = Self::parse_path(&path);
        let mut node = &self.root;
        for p in &parts {
            node = node.children.get(*p).unwrap();
        }
        if node.is_file {
            return vec![parts.last().unwrap().to_string()];
        }
        let mut names: Vec<String> = node.children.keys().cloned().collect();
        names.sort();
        names
    }

    fn mkdir(&mut self, path: String) {
        let parts = Self::parse_path(&path);
        let mut node = &mut self.root;
        for p in parts {
            node = node.children.entry(p.to_string()).or_insert_with(FsNode::default);
        }
    }

    fn add_content_to_file(&mut self, file_path: String, content: String) {
        let parts = Self::parse_path(&file_path);
        let mut node = &mut self.root;
        for p in parts {
            node = node.children.entry(p.to_string()).or_insert_with(FsNode::default);
        }
        node.is_file = true;
        node.content.push_str(&content);
    }

    fn read_content_from_file(&self, file_path: String) -> String {
        let parts = Self::parse_path(&file_path);
        let mut node = &self.root;
        for p in &parts {
            node = node.children.get(*p).unwrap();
        }
        node.content.clone()
    }
}

fn main() {
    let mut fs = FileSystem::new();
    fs.mkdir("/a/b/c".to_string());
    fs.add_content_to_file("/a/b/c/d".to_string(), "hello".to_string());
    println!("{:?}", fs.ls("/".to_string()));
    println!("{}", fs.read_content_from_file("/a/b/c/d".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut fs = FileSystem::new();
        fs.mkdir("/a/b/c".to_string());
        fs.add_content_to_file("/a/b/c/d".to_string(), "hello".to_string());
        assert_eq!(fs.ls("/".to_string()), vec!["a".to_string()]);
        assert_eq!(
            fs.read_content_from_file("/a/b/c/d".to_string()),
            "hello".to_string()
        );
    }

    #[test]
    fn ls_on_a_file_returns_just_that_file() {
        let mut fs = FileSystem::new();
        fs.add_content_to_file("/a/b.txt".to_string(), "hi".to_string());
        assert_eq!(fs.ls("/a/b.txt".to_string()), vec!["b.txt".to_string()]);
    }

    #[test]
    fn ls_root_lists_sorted_entries() {
        let mut fs = FileSystem::new();
        fs.mkdir("/zeta".to_string());
        fs.mkdir("/alpha".to_string());
        assert_eq!(
            fs.ls("/".to_string()),
            vec!["alpha".to_string(), "zeta".to_string()]
        );
    }

    #[test]
    fn appending_content_concatenates() {
        let mut fs = FileSystem::new();
        fs.add_content_to_file("/f".to_string(), "hello".to_string());
        fs.add_content_to_file("/f".to_string(), " world".to_string());
        assert_eq!(
            fs.read_content_from_file("/f".to_string()),
            "hello world".to_string()
        );
    }
}
