struct Solution;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let num_ys = grid.len();
        let num_xs = grid[0].len();
        let mut visited = vec![vec![false; num_xs]; num_ys];
        let mut islands = 0;
        for y in 0..num_ys {
            for x in 0..num_xs {
                if !visited[y][x] && grid[y][x] != 1 {
                    let was_fake_island =
                        Self::dfs(&grid, x as isize, y as isize, num_xs, num_ys, &mut visited);
                    islands += !was_fake_island as usize;
                }
            }
        }

        islands as i32
    }

    fn dfs(
        grid: &Vec<Vec<i32>>,
        x: isize,
        y: isize,
        num_xs: usize,
        num_ys: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        let mut is_fake_island = false;
        if x == num_xs as isize - 1 || y == num_ys as isize - 1 || x == 0 || y == 0 {
            is_fake_island = true;
        }

        visited[y as usize][x as usize] = true;

        for (xi, yi) in [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)] {
            if xi < 0 || yi < 0 || xi >= num_xs as isize || yi >= num_ys as isize {
                continue;
            }
            if !visited[yi as usize][xi as usize] && grid[yi as usize][xi as usize] != 1 {
                let fake_island = Self::dfs(grid, xi, yi, num_xs, num_ys, visited);
                is_fake_island = is_fake_island || fake_island;
            }
        }

        is_fake_island
    }
}
fn main() {
    println!("Hello, world!");
}
