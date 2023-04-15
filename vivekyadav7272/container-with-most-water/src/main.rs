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
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut biggest_area = 0;

        while i < j {
            // assert!(j < height.len());
            while i < j && height[i] <= height[j] {
                biggest_area = biggest_area.max((j - i) as i32 * height[i]);
                i += 1;
            }

            while i < j && height[j] < height[i] {
                biggest_area = biggest_area.max((j - i) as i32 * height[j]);
                j -= 1;
            }
        }
        biggest_area
    }
}

fn main() {
    check_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    check_eq!(Solution::max_area(vec![1, 1]), 1);
}
