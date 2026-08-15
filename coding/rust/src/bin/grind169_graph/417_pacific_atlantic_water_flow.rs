//! Grind 169 — LeetCode #417 Pacific Atlantic Water Flow (Medium)
//!
//! Given an m x n grid of heights, find all cells from which water can
//! flow to both the Pacific (top/left edges) and Atlantic (bottom/right
//! edges) oceans. Solved backwards: DFS from every Pacific-adjacent
//! border cell (and separately from every Atlantic-adjacent border
//! cell), climbing to equal-or-higher neighbors; a cell reachable in
//! both searches can drain to both oceans.
//!
//! Example:
//!   Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],
//!                      [6,7,1,4,5],[5,1,1,2,4]]
//!   Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]

struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = heights.len();
        let cols = heights[0].len();
        let mut pacific = vec![vec![false; cols]; rows];
        let mut atlantic = vec![vec![false; cols]; rows];

        fn dfs(
            heights: &Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            r: i32,
            c: i32,
            rows: i32,
            cols: i32,
            prev_height: i32,
        ) {
            if r < 0 || r >= rows || c < 0 || c >= cols {
                return;
            }
            if visited[r as usize][c as usize] {
                return;
            }
            if heights[r as usize][c as usize] < prev_height {
                return;
            }
            visited[r as usize][c as usize] = true;
            let h = heights[r as usize][c as usize];
            dfs(heights, visited, r + 1, c, rows, cols, h);
            dfs(heights, visited, r - 1, c, rows, cols, h);
            dfs(heights, visited, r, c + 1, rows, cols, h);
            dfs(heights, visited, r, c - 1, rows, cols, h);
        }

        let (rr, cc) = (rows as i32, cols as i32);
        for c in 0..cols {
            dfs(&heights, &mut pacific, 0, c as i32, rr, cc, i32::MIN);
            dfs(&heights, &mut atlantic, rows as i32 - 1, c as i32, rr, cc, i32::MIN);
        }
        for r in 0..rows {
            dfs(&heights, &mut pacific, r as i32, 0, rr, cc, i32::MIN);
            dfs(&heights, &mut atlantic, r as i32, cols as i32 - 1, rr, cc, i32::MIN);
        }

        let mut result = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if pacific[r][c] && atlantic[r][c] {
                    result.push(vec![r as i32, c as i32]);
                }
            }
        }
        result
    }
}

fn main() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    println!("{:?}", Solution::pacific_atlantic(heights));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        assert_eq!(
            Solution::pacific_atlantic(heights),
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ]
        );
    }

    #[test]
    fn single_cell_reaches_both() {
        assert_eq!(
            Solution::pacific_atlantic(vec![vec![1]]),
            vec![vec![0, 0]]
        );
    }

    #[test]
    fn flat_grid_all_cells_reach_both() {
        let heights = vec![vec![1, 1], vec![1, 1]];
        let mut result = Solution::pacific_atlantic(heights);
        result.sort();
        assert_eq!(result, vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]);
    }
}
