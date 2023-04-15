struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let (first_player_score, second_player_score) =
            Self::_predict_the_winner(&nums, 0, nums.len(), &mut HashMap::new());
        first_player_score >= second_player_score
    }

    fn _predict_the_winner(
        nums: &[i32],
        start_idx: usize,
        len: usize,
        dp: &mut HashMap<(usize, usize), (u32, u32)>,
    ) -> (u32, u32) {
        if nums.len() == 0 {
            return (0, 0);
        }
        if nums.len() == 1 {
            return (nums[0] as u32, 0);
        }

        if let Some(&ans) = dp.get(&(start_idx, len)) {
            return ans;
        }

        let front_score = nums[0] as u32;
        let last_score = nums[nums.len() - 1] as u32;

        let chop_front = Self::_predict_the_winner(&nums[1..], start_idx + 1, len - 1, dp);
        let chop_last = Self::_predict_the_winner(&nums[..nums.len() - 1], start_idx, len - 1, dp);

        let ans = if front_score + chop_front.1 > last_score + chop_last.1 {
            (front_score + chop_front.1, chop_front.0)
        } else {
            (last_score + chop_last.1, chop_last.0)
        };

        dp.insert((start_idx, len), ans);

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
