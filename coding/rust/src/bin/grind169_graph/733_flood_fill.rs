//! Grind 169 — LeetCode #733 Flood Fill (Easy)
//!
//! Given an image (2D grid of pixel values), a starting pixel (sr, sc),
//! and a new color, recolor the starting pixel and all 4-directionally
//! connected pixels of the same original color.
//!
//! Example:
//!   Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
//!   Output: [[2,2,2],[2,2,0],[2,0,1]]

struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let old = image[sr as usize][sc as usize];
        if old == color {
            return image;
        }
        let rows = image.len() as i32;
        let cols = image[0].len() as i32;

        fn dfs(image: &mut Vec<Vec<i32>>, r: i32, c: i32, rows: i32, cols: i32, old: i32, new: i32) {
            if r < 0 || r >= rows || c < 0 || c >= cols || image[r as usize][c as usize] != old {
                return;
            }
            image[r as usize][c as usize] = new;
            dfs(image, r + 1, c, rows, cols, old, new);
            dfs(image, r - 1, c, rows, cols, old, new);
            dfs(image, r, c + 1, rows, cols, old, new);
            dfs(image, r, c - 1, rows, cols, old, new);
        }

        dfs(&mut image, sr, sc, rows, cols, old, color);
        image
    }
}

fn main() {
    let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    println!("{:?}", Solution::flood_fill(image, 1, 1, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        assert_eq!(
            Solution::flood_fill(image, 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }

    #[test]
    fn example_2_same_color_is_noop() {
        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(
            Solution::flood_fill(image, 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn single_pixel() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1]], 0, 0, 5),
            vec![vec![5]]
        );
    }

    #[test]
    fn disconnected_regions_unaffected() {
        let image = vec![vec![1, 0, 1]];
        assert_eq!(
            Solution::flood_fill(image, 0, 0, 9),
            vec![vec![9, 0, 1]]
        );
    }
}
