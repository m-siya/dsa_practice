struct Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut land = Vec::new();
        let n = grid.len();
        assert!(n >= 1 && n == grid[0].len());
        let mut visited = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    land.push((i, j));
                    visited[i][j] = true;
                }
            }
        }
        if land.len() == 0 || land.len() == n * n {
            return -1;
        }
        let mut virus = -1;
        while !land.is_empty() {
            let mut captured_land = Vec::new(); // land captured with virus+1
            for (i, j) in land {
                if i > 0 && !visited[i - 1][j] {
                    // up
                    captured_land.push((i - 1, j));
                    visited[i - 1][j] = true;
                }
                if i < n - 1 && !visited[i + 1][j] {
                    // down
                    captured_land.push((i + 1, j));
                    visited[i + 1][j] = true;
                }
                if j > 0 && !visited[i][j - 1] {
                    // left
                    captured_land.push((i, j - 1));
                    visited[i][j - 1] = true;
                }
                if j < n - 1 && !visited[i][j + 1] {
                    // right
                    captured_land.push((i, j + 1));
                    visited[i][j + 1] = true;
                }
            }
            land = captured_land;
            virus += 1;
        }
        virus
    }
}

fn main() {
    assert_eq!(
        Solution::max_distance(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::max_distance(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
        4
    );
    assert_eq!(
        Solution::max_distance(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
        -1
    );
}
