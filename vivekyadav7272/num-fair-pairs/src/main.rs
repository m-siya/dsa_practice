#[cfg(debug_assertions)]
macro_rules! dbg_print {
    ($garbage:expr) => {
        dbg!($garbage)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_print {
    ($garbage:expr) => {};
}

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
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut counter = 0;
        for (i, num) in nums.iter().enumerate() {
            let first_valid = nums[i + 1..].partition_point(|x| x + num < lower);
            let first_invalid = nums[i + 1..].partition_point(|x| x + num <= upper);
            counter += first_invalid - first_valid;
        }
        counter as i64
    }
}

fn main() {
    check_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
    check_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
}
