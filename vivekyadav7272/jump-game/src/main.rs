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
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut curr_pos = 0usize;
        let mut total_jumps = 0i32;
        let mut curr_jmp = nums[0];
        while (curr_pos + curr_jmp as usize) < nums.len() - 1 {
            let largest_offset = curr_jmp.min((nums.len() - curr_pos - 1) as i32) as usize;
            let (curr_pos_, &curr_jmp_) = nums[curr_pos..=curr_pos + largest_offset]
                .iter()
                .enumerate()
                .max_by_key(|&(idx, &jmp)| idx + jmp as usize)
                .unwrap();

            curr_jmp = curr_jmp_;
            curr_pos += curr_pos_;
            total_jumps += 1;
        }
        total_jumps + 1
    }
}

fn main() {
    check_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    check_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    check_eq!(Solution::jump(vec![0]), 0);
    check_eq!(Solution::jump(vec![2, 3, 1]), 1);
}
