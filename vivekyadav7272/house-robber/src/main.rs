struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp: HashMap<usize, i32> = [(0usize, 0), (1, nums[nums.len() - 1])]
            .iter()
            .map(|&(idx, ans)| (idx, ans))
            .collect();
        (0..nums.len()).fold(0, |max_heist, i| {
            max_heist.max(Self::_rob(&nums[i..], &mut dp))
        })
    }

    fn _rob(nums: &[i32], dp: &mut HashMap<usize, i32>) -> i32 {
        if let Some(&heist_score) = dp.get(&nums.len()) {
            return heist_score;
        }

        let ans = nums[0]
            + (2..nums.len()).fold(0, |max_heist, i| max_heist.max(Self::_rob(&nums[i..], dp)));
        dp.insert(nums.len(), ans);
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
