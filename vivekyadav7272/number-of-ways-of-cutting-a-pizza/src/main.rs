struct Solution;
use std::collections::HashMap;

struct Soln {
    pizza: Vec<String>,
    rows: usize,
    cols: usize,
    apple_count: Vec<Vec<usize>>,
    dp: Vec<Vec<Vec<Option<usize>>>>,
}

impl Soln {
    const LIMIT: usize = 1000000007;

    fn get_apple_count(&self, row: usize, col: usize) -> usize {
        if row >= self.rows || col >= self.cols {
            return 0;
        }
        *unsafe { self.apple_count.get_unchecked(row).get_unchecked(col) }
    }

    fn get_dp(&self, k: i32, x: usize, y: usize) -> Option<usize> {
        *unsafe {
            self.dp
                .get_unchecked((k - 1) as usize)
                .get_unchecked(x)
                .get_unchecked(y)
        }
    }

    fn set_dp(&mut self, k: i32, x: usize, y: usize, value: usize) {
        unsafe {
            *self
                .dp
                .get_unchecked_mut((k - 1) as usize)
                .get_unchecked_mut(x)
                .get_unchecked_mut(y) = Some(value);
        }
    }

    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let rows = pizza.len();
        let cols = pizza[0].len();

        let apple_count = vec![vec![0; cols]; rows];
        let dp = vec![vec![vec![None; cols]; rows]; k as usize];
        let mut me = Self {
            pizza,
            rows,
            cols,
            apple_count,
            dp,
        };

        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                me.apple_count[i][j] = (me.pizza[i].as_bytes()[j] == b'A') as usize
                    + me.get_apple_count(i + 1, j)
                    + me.get_apple_count(i, j + 1)
                    - me.get_apple_count(i + 1, j + 1);
            }
        }

        me._ways(0, 0, k) as i32
    }

    fn _ways(&mut self, x: usize, y: usize, k: i32) -> usize {
        if x == self.rows || y == self.cols {
            return 0;
        }

        if self.get_apple_count(x, y) < k as usize {
            return 0;
        }
        // if k == 1, we just need to see if our box has any apple, if so, yeah this last person has a valid slice
        // (no. of ways to give this last person a valid slice = 1, give him the whole slice (he the last person
        // so can't bifurcate it further)
        // if not, no ways of giving last person any slice (sad)
        if k == 1 {
            if self.get_apple_count(x, y) > 0 {
                return 1;
            }
            return 0;
        }

        if let Some(ans) = self.get_dp(k, x, y) {
            return ans;
        }

        let mut total_ways = 0;

        let mut has_apples = false;

        for i in x..self.rows - 1 {
            if has_apples || self.has_apples_row(i, y) {
                has_apples = true;
                let ans = self._ways(i + 1, y, k - 1);
                total_ways += ans;
            }
        }

        has_apples = false;
        for j in y..self.cols - 1 {
            if has_apples || self.has_apples_col(x, j) {
                has_apples = true;
                let ans = self._ways(x, j + 1, k - 1);
                total_ways += ans;
            }
        }

        total_ways = total_ways % Soln::LIMIT;
        self.set_dp(k, x, y, total_ways);
        // self.dp.insert((x, y, k), total_ways);

        total_ways
    }

    fn has_apples_row(&self, row: usize, col_begin: usize) -> bool {
        (self.get_apple_count(row, col_begin) - self.get_apple_count(row + 1, col_begin)) > 0
    }

    fn has_apples_col(&self, row_begin: usize, col: usize) -> bool {
        (self.get_apple_count(row_begin, col) - self.get_apple_count(row_begin, col + 1)) > 0
    }
}
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        Soln::ways(pizza, k)
    }
}

fn main() {}
