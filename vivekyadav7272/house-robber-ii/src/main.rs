struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn rob_1(nums: &[i32], dp: &mut HashMap<usize, i32>) -> i32 {
        dp.insert(0, 0);
        (0..nums.len()).fold(0, |max_heist, i| {
            max_heist.max(Self::_rob_1(&nums[i..], dp))
        })
    }

    fn _rob_1(nums: &[i32], dp: &mut HashMap<usize, i32>) -> i32 {
        if let Some(&heist_score) = dp.get(&nums.len()) {
            return heist_score;
        }

        let ans = nums[0]
            + (2..nums.len()).fold(0, |max_heist, i| {
                max_heist.max(Self::_rob_1(&nums[i..], dp))
            });
        dp.insert(nums.len(), ans);
        ans
    }

    pub fn rob(mut nums: Vec<i32>) -> i32 {
        let mut dp = HashMap::<usize, i32>::new();
        let first_elem = nums[0];
        let excluded = Self::rob_1(&nums[1..], &mut dp);
        dp.clear();
        nums[0] = 0;
        if let Some(second) = nums.get_mut(1) {
            *second = 0;
        }
        if let Some(last) = nums.last_mut() {
            *last = 0;
        }
        let included = first_elem + Self::rob_1(&nums, &mut dp);
        excluded.max(included)
    }
}
fn main() {
    println!("Hello, world!");
}
