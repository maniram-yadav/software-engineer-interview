//! LeetCode Top Interview 150 — #110 Construct Quad Tree (Medium)
//!
//! Given an n x n binary grid, build a Quad-Tree representation,
//! recursively splitting into 4 quadrants until each region is uniform.
//!
//! Example:
//!   Input: grid = [[0,1],[1,0]]
//!   Output: quad-tree with 4 leaf nodes for each cell

#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: bool,
    is_leaf: bool,
    top_left: Option<Box<Node>>,
    top_right: Option<Box<Node>>,
    bottom_left: Option<Box<Node>>,
    bottom_right: Option<Box<Node>>,
}

impl Node {
    fn leaf(val: bool) -> Option<Box<Node>> {
        Some(Box::new(Node {
            val,
            is_leaf: true,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }))
    }

    fn internal(
        top_left: Option<Box<Node>>,
        top_right: Option<Box<Node>>,
        bottom_left: Option<Box<Node>>,
        bottom_right: Option<Box<Node>>,
    ) -> Option<Box<Node>> {
        Some(Box::new(Node {
            val: true,
            is_leaf: false,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }))
    }
}

struct Solution;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Box<Node>> {
        fn build(grid: &Vec<Vec<i32>>, r: usize, c: usize, size: usize) -> Option<Box<Node>> {
            let first = grid[r][c];
            let uniform = (r..r + size).all(|i| (c..c + size).all(|j| grid[i][j] == first));
            if uniform {
                return Node::leaf(first == 1);
            }
            let half = size / 2;
            let tl = build(grid, r, c, half);
            let tr = build(grid, r, c + half, half);
            let bl = build(grid, r + half, c, half);
            let br = build(grid, r + half, c + half, half);
            Node::internal(tl, tr, bl, br)
        }

        let n = grid.len();
        build(&grid, 0, 0, n)
    }
}

fn main() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    println!("{:?}", Solution::construct(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_non_uniform_two_by_two() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let result = Solution::construct(grid);
        let expected = Node::internal(
            Node::leaf(false),
            Node::leaf(true),
            Node::leaf(true),
            Node::leaf(false),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2_uniform_grid_is_single_leaf() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::construct(grid), Node::leaf(true));
    }

    #[test]
    fn all_zero_grid_is_single_leaf() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::construct(grid), Node::leaf(false));
    }

    #[test]
    fn single_cell_grid() {
        assert_eq!(Solution::construct(vec![vec![1]]), Node::leaf(true));
    }
}
