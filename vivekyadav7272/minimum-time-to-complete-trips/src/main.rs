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
impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as u64;
        // let denom: u64 = time.iter().map(|&x| x as u64).product();
        // let numer: u64 = time.iter().map(|&x| denom / (x as u64)).sum();

        let min_trip_time = 1u64;
        // (((total_trips * denom) as f64) / numer as f64).ceil() as u64;
        // This assumes they do trips in 1/n fashion, i.e also a fraction of trips every second,
        // which ain't true.

        let max_trip_time = total_trips * *time.iter().min().unwrap() as u64;

        let mut left = min_trip_time;
        let mut right = max_trip_time;
        let mut mid = (left + right) / 2;
        while left < right {
            let trips = Self::calc_days_for_t(&time, mid);
            if trips < total_trips {
                left = mid + 1;
            } else {
                right = mid;
            }
            mid = (left + right) / 2;
        }

        mid as i64
    }

    fn calc_days_for_t(time: &[i32], t: u64) -> u64 {
        time.iter().map(|&x| t / (x as u64)).sum()
    }
}

fn main() {
    check_eq!(Solution::minimum_time(vec![1, 2, 3], 5), 3);
    check_eq!(Solution::minimum_time(vec![2], 1), 2);
    check_eq!(Solution::minimum_time(vec![1, 3, 7, 11], 87), 56);
    check_eq!(Solution::minimum_time(vec![5, 10, 10], 9), 25);
}
