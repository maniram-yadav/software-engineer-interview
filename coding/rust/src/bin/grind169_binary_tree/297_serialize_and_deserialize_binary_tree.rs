//! Grind 169 — LeetCode #297 Serialize and Deserialize Binary Tree (Hard)
//!
//! Design an algorithm to serialize a binary tree to a string and
//! deserialize that string back to the original tree structure. Uses a
//! comma-separated preorder traversal with "#" marking null children,
//! which is enough to reconstruct the exact tree shape unambiguously.
//!
//! Example:
//!   Input: root = [1,2,3,null,null,4,5]
//!   Output (round-trip): [1,2,3,null,null,4,5]

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Box<TreeNode>>) -> String {
        fn helper(node: &Option<Box<TreeNode>>, result: &mut Vec<String>) {
            match node {
                None => result.push("#".to_string()),
                Some(n) => {
                    result.push(n.val.to_string());
                    helper(&n.left, result);
                    helper(&n.right, result);
                }
            }
        }
        let mut result = Vec::new();
        helper(&root, &mut result);
        result.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Box<TreeNode>> {
        let values: Vec<&str> = data.split(',').collect();
        let mut idx = 0;
        fn helper(values: &[&str], idx: &mut usize) -> Option<Box<TreeNode>> {
            if values[*idx] == "#" {
                *idx += 1;
                return None;
            }
            let val: i32 = values[*idx].parse().unwrap();
            *idx += 1;
            let mut n = Box::new(TreeNode::new(val));
            n.left = helper(values, idx);
            n.right = helper(values, idx);
            Some(n)
        }
        helper(&values, &mut idx)
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let codec = Codec::new();
    let root = node(1, leaf(2), node(3, leaf(4), leaf(5)));
    let serialized = codec.serialize(root);
    println!("{}", serialized);
    println!("{:?}", codec.deserialize(serialized.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_roundtrip() {
        let codec = Codec::new();
        let root = node(1, leaf(2), node(3, leaf(4), leaf(5)));
        let serialized = codec.serialize(root);
        let expected = node(1, leaf(2), node(3, leaf(4), leaf(5)));
        assert_eq!(codec.deserialize(serialized), expected);
    }

    #[test]
    fn empty_tree_roundtrip() {
        let codec = Codec::new();
        let serialized = codec.serialize(None);
        assert_eq!(codec.deserialize(serialized), None);
    }

    #[test]
    fn single_node_roundtrip() {
        let codec = Codec::new();
        let serialized = codec.serialize(leaf(42));
        assert_eq!(codec.deserialize(serialized), leaf(42));
    }

    #[test]
    fn negative_values_roundtrip() {
        let codec = Codec::new();
        let root = node(-1, leaf(-2), leaf(-3));
        let serialized = codec.serialize(root);
        let expected = node(-1, leaf(-2), leaf(-3));
        assert_eq!(codec.deserialize(serialized), expected);
    }
}
