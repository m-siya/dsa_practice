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
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut nums = &nums[..];
        loop {
            if nums.len() == 1 {
                break nums[0];
            }
            let mid = nums.len() / 2;
            assert!(mid >= 1 && mid <= nums.len() - 2);
            if nums[mid] == nums[mid - 1] {
                if mid % 2 == 0 {
                    nums = &nums[..mid - 1];
                } else {
                    nums = &nums[mid + 1..];
                }
            } else if nums[mid] == nums[mid + 1] {
                if mid % 2 == 0 {
                    nums = &nums[mid + 2..];
                } else {
                    nums = &nums[..mid];
                }
            } else {
                break nums[mid];
            }
        }
    }
}
fn main() {
    check_eq!(
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
        2
    );
    check_eq!(
        Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
        10
    );
}
