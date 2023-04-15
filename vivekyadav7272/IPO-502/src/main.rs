struct Solution;
macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        println!("`{left_stringified}` expected to be `{}`", $right);
        let expr = $left;
        let ans = $right;
        assert_eq!(expr, ans);
    };
}
#[cfg(debug_assertions)]
macro_rules! dbg_print {
    ($garbage:expr) => {
        dbg!($garbage)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_print {
    ($garbage:expr) => {
        $garbage
    };
}

use std::collections::BinaryHeap;
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut net_profits: Vec<(i32, i32)> =
            capital.into_iter().zip(profits.into_iter()).collect();
        net_profits.sort_unstable_by(|&a, &b| a.0.cmp(&b.0));
        dbg_print!(&net_profits);
        let mut curr_cap = dbg_print!(w);
        let mut profitable_projects = BinaryHeap::with_capacity(4);
        let mut future_projects = net_profits.as_slice();
        for _ in 0..k {
            let workable_projects =
                dbg_print!(future_projects.partition_point(|x| x.0 <= curr_cap));
            for &(_, pure_profit) in dbg_print!(&future_projects[..workable_projects]) {
                profitable_projects.push(pure_profit);
            }
            future_projects = &future_projects[workable_projects..];
            if let Some(biggest_profit) = profitable_projects.pop() {
                curr_cap += biggest_profit;
            } else {
                break;
            }
        }
        curr_cap
    }
}
fn main() {
    check_eq!(
        Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
        4
    );
    check_eq!(
        Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
        6
    );
}
