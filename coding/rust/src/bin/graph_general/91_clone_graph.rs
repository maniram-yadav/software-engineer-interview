//! LeetCode Top Interview 150 — #91 Clone Graph (Medium)
//!
//! Given a reference to a node in a connected undirected graph, return a
//! deep copy of the graph. Nodes may reference each other in cycles, so
//! they use `Rc<RefCell<Node>>` rather than `Box`. Solved with DFS plus a
//! HashMap (keyed by original node value — values are unique per the
//! problem's constraints) mapping original nodes to their clones, so
//! shared/cyclic references are cloned exactly once.
//!
//! Example:
//!   Input: adjList = [[2,4],[1,3],[2,4],[1,3]]
//!   Output: deep-cloned graph with identical adjacency

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    val: i32,
    neighbors: Vec<NodeRef>,
}

impl Node {
    fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(Node {
            val,
            neighbors: Vec::new(),
        }))
    }
}

struct Solution;

impl Solution {
    pub fn clone_graph(node: Option<NodeRef>) -> Option<NodeRef> {
        fn dfs(node: &NodeRef, visited: &mut HashMap<i32, NodeRef>) -> NodeRef {
            let val = node.borrow().val;
            if let Some(cloned) = visited.get(&val) {
                return cloned.clone();
            }
            let clone = Node::new(val);
            visited.insert(val, clone.clone());

            let neighbors = node.borrow().neighbors.clone();
            for n in &neighbors {
                let cloned_neighbor = dfs(n, visited);
                clone.borrow_mut().neighbors.push(cloned_neighbor);
            }
            clone
        }

        let mut visited: HashMap<i32, NodeRef> = HashMap::new();
        node.map(|n| dfs(&n, &mut visited))
    }
}

// Builds a graph from an adjacency list (1-indexed values, as in LC's
// format) and returns the node for value 1 (or None if the list is empty).
fn build(adj_list: &[Vec<i32>]) -> Option<NodeRef> {
    if adj_list.is_empty() {
        return None;
    }
    let nodes: Vec<NodeRef> = (1..=adj_list.len() as i32).map(Node::new).collect();
    for (i, neighbors) in adj_list.iter().enumerate() {
        for &nv in neighbors {
            nodes[i].borrow_mut().neighbors.push(nodes[(nv - 1) as usize].clone());
        }
    }
    Some(nodes[0].clone())
}

// Flattens a graph (reachable from `start`) into a sorted adjacency list
// of (val, sorted neighbor vals) for order-independent comparison.
fn flatten(start: &NodeRef) -> Vec<(i32, Vec<i32>)> {
    let mut visited: HashMap<i32, NodeRef> = HashMap::new();
    let mut stack = vec![start.clone()];
    while let Some(n) = stack.pop() {
        let val = n.borrow().val;
        if visited.contains_key(&val) {
            continue;
        }
        visited.insert(val, n.clone());
        for neighbor in n.borrow().neighbors.iter() {
            stack.push(neighbor.clone());
        }
    }
    let mut result: Vec<(i32, Vec<i32>)> = visited
        .values()
        .map(|n| {
            let val = n.borrow().val;
            let mut neighbor_vals: Vec<i32> =
                n.borrow().neighbors.iter().map(|x| x.borrow().val).collect();
            neighbor_vals.sort_unstable();
            (val, neighbor_vals)
        })
        .collect();
    result.sort_by_key(|&(v, _)| v);
    result
}

fn main() {
    let graph = build(&[vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]);
    let cloned = Solution::clone_graph(graph);
    println!("{:?}", flatten(&cloned.unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let graph = build(&[vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]);
        let original_flat = flatten(graph.as_ref().unwrap());
        let cloned = Solution::clone_graph(graph.clone()).unwrap();
        assert_eq!(flatten(&cloned), original_flat);
        assert!(!Rc::ptr_eq(&cloned, graph.as_ref().unwrap()));
    }

    #[test]
    fn example_2_single_node_no_neighbors() {
        let graph = build(&[vec![]]);
        let cloned = Solution::clone_graph(graph).unwrap();
        assert_eq!(flatten(&cloned), vec![(1, vec![])]);
    }

    #[test]
    fn example_3_empty_graph() {
        assert!(Solution::clone_graph(None).is_none());
    }

    #[test]
    fn clone_is_deep_not_shallow() {
        let graph = build(&[vec![2], vec![1]]);
        let cloned = Solution::clone_graph(graph.clone()).unwrap();
        // Same value/structure, but distinct underlying nodes.
        assert_eq!(flatten(&cloned), flatten(graph.as_ref().unwrap()));
        assert!(!Rc::ptr_eq(
            &cloned.borrow().neighbors[0],
            &graph.as_ref().unwrap().borrow().neighbors[0]
        ));
    }
}
