mod unique_paths_ii;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::_unique_paths(m, n, &mut memo)
    }

    fn _unique_paths(m: i32, n: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        if m == 0 || n == 0 {
            return 0;
        }

        if let Some(&num_path) = memo.get(&(m, n)) {
            return num_path;
        }

        let ans = Self::_unique_paths(m - 1, n, memo) + Self::_unique_paths(m, n - 1, memo);
        memo.insert((m, n), ans);
        memo.insert((n, m), ans);

        ans
    }
}

fn main() {
    dbg!(unique_paths_ii::Solution::unique_paths_with_obstacles(
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
    ));
    dbg!(unique_paths_ii::Solution::unique_paths_with_obstacles(
        vec![vec![0, 1], vec![0, 0]]
    ));
}
