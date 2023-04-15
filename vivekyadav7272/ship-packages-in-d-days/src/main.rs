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

struct Solution;
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days_req: i32) -> i32 {
        let (mut min_cap, mut max_cap) = weights
            .iter()
            .fold((0, 0), |acc, &x| (acc.0.max(x), acc.1 + x));
        dbg_print!(min_cap);
        loop {
            if max_cap <= min_cap {
                break min_cap;
            }
            let curr_cap = dbg_print!((min_cap + max_cap) / 2);
            let num_days = dbg_print!(Self::days_required(&weights, curr_cap));
            if num_days <= days_req {
                max_cap = curr_cap;
            } else if num_days > days_req {
                min_cap = curr_cap + 1;
            }
        }
    }
    fn days_required(weights: &[i32], ship_cap: i32) -> i32 {
        let mut days_req = 0;
        let mut cargo_space = ship_cap;
        for &weight in weights {
            assert!(weight <= ship_cap);
            cargo_space -= weight;
            if cargo_space < 0 {
                cargo_space = ship_cap - weight;
                days_req += 1;
            }
        }
        if cargo_space != ship_cap {
            days_req += 1;
        }
        days_req
    }
}

fn main() {
    check_eq!(
        Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
        15
    );

    check_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    check_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    check_eq!(
        Solution::ship_within_days(vec![10, 50, 100, 100, 50, 100, 100, 100], 5),
        160
    );
}
