use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut memo = HashMap::new();

        Self::_unique_paths_with_obstacles(
            &obstacle_grid,
            obstacle_grid.len(),
            obstacle_grid[0].len(),
            0,
            0,
            &mut memo,
        )
    }

    fn _unique_paths_with_obstacles(
        obstacle_grid: &Vec<Vec<i32>>,
        rows: usize,
        cols: usize,
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if i == rows || j == cols || obstacle_grid[i][j] == 1 {
            return 0;
        }

        if i == rows - 1 && j == cols - 1 {
            return 1;
        }

        if let Some(&num_path) = memo.get(&(i, j)) {
            return num_path;
        }

        let ans = Self::_unique_paths_with_obstacles(obstacle_grid, rows, cols, i + 1, j, memo)
            + Self::_unique_paths_with_obstacles(obstacle_grid, rows, cols, i, j + 1, memo);
        memo.insert((i, j), ans);

        ans
    }
}
