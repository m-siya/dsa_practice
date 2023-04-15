struct Solution;
macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        let expr = $left;
        let ans = $right;
        if expr != ans {
            println!(
                "`{left_stringified}` expected to be `{:?}`, but was {:?}",
                $right, expr
            );
        }
    };
}
use std::collections::HashMap;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![-1i32; m]; n];
        Self::_min_path_sum(&grid, &mut dp, m, n, 0, 0)
    }

    fn _min_path_sum(
        grid: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<i32>>,
        m: usize,
        n: usize,
        x: usize,
        y: usize,
    ) -> i32 {
        if x >= m || y >= n {
            return i32::MAX;
        }

        let dp_ans = dp[y][x];
        if dp_ans != -1 {
            return dp_ans;
        }

        let down = Self::_min_path_sum(grid, dp, m, n, x, y + 1);
        let right = Self::_min_path_sum(grid, dp, m, n, x + 1, y);

        let ans = grid[y][x] + Self::min_or_valid(down, right);
        dp[y][x] = ans;

        ans
    }

    fn min_or_valid(ans1: i32, ans2: i32) -> i32 {
        if ans1 == i32::MAX {
            if ans2 == i32::MAX {
                0
            } else {
                ans2
            }
        } else {
            ans1.min(ans2)
        }
    }
}
fn main() {
    // check_eq!(Solution::min_path_sum(vec![]))
}
