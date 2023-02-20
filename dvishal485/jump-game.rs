struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut nums = &nums[..];
        while nums.len() > 1 {
            let jump = nums[0] as usize;
            if jump == 0 {
                return false;
            }
            nums = &nums[1..];
            if jump >= nums.len() {
                return true;
            }
            let local_avaliable = &nums[..jump];
            let (idx, _) = local_avaliable
                .iter()
                .enumerate()
                .max_by_key(|&(idx, &elem)| idx as i32 + elem)
                .unwrap();
            nums = &nums[idx..];
        }
        true
    }
}

fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(Solution::can_jump(vec![3, 0, 8, 2, 0, 0, 1]), true);
    assert_eq!(
        Solution::can_jump(vec![4, 2, 0, 0, 1, 1, 4, 4, 4, 0, 4, 0]),
        true
    );
}
