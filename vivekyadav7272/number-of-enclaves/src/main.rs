struct Solution;

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();

        for i in [0, m - 1] {
            for j in 0..n {
                if grid[i][j] == 1 {
                    Self::dfs(&mut grid, i as isize, j as isize, m, n);
                }
            }
        }

        for j in [0, n - 1] {
            for i in 0..m {
                if grid[i][j] == 1 {
                    Self::dfs(&mut grid, i as isize, j as isize, m, n);
                }
            }
        }

        let mut landlocked_cells = 0;
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if grid[i][j] == 1 {
                    landlocked_cells += 1;
                }
            }
        }

        landlocked_cells
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, row: isize, col: isize, rows: usize, cols: usize) {
        grid[row as usize][col as usize] = 0;

        for (i, j) in [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ] {
            if i == rows as isize || j == cols as isize || i < 0 || j < 0 {
                continue;
            }
            if grid[i as usize][j as usize] == 1 {
                Self::dfs(grid, i, j, rows, cols);
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
