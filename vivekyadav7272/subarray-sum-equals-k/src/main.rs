macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        println!("`{left_stringified}` expected to be `{}`", $right);
        let expr = $left;
        let ans = $right;
        assert_eq!(expr, ans);
    };
}
struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cumsum = 0;
        let mut counters = HashMap::<i32, i32>::with_capacity(nums.len());
        let mut total_subarrs = 0i32;
        counters.insert(0, 1);
        for &num in nums.iter() {
            cumsum += num;
            let target = cumsum - k;
            if let Some(target_count) = counters.get(&target) {
                total_subarrs += target_count;
            }
            *counters.entry(cumsum).or_default() += 1;
        }

        total_subarrs
    }
}
fn main() {
    check_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    check_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
}
