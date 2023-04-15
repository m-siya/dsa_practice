pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = HashMap::with_capacity(n as usize);
        Self::_climb_stairs(n, &mut dp)
    }
    fn _climb_stairs(n: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            return 1;
        }
        if let Some(&num_ways) = dp.get(&n) {
            return num_ways;
        }

        let ans = Self::_climb_stairs(n - 1, dp) + Self::_climb_stairs(n - 2, dp);
        dp.insert(n, ans);
        ans
    }
}
