#[cfg(debug_assertions)]
macro_rules! dbg_print {
    ($garbage:expr) => {
        dbg!($garbage)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_print {
    ($garbage:expr) => {
        $garbage
    };
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

/*
pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len());

        let mut current_min = i32::MAX;
        for mut num in nums.into_iter() {
            if num & 1 == 1 {
                num = num << 1;
            }
            current_min = current_min.min(num);
            heap.push(num);
        }
        let mut deviation = i32::MAX;
        while let Some(current_max) = heap.pop() {
            deviation = deviation.min(current_max - current_min);
            if deviation == 0 {
                return 0;
            }
            if current_max & 1 == 0 {
                let current_max = current_max >> 1;
                heap.push(current_max);
                current_min = current_min.min(current_max);
            } else {
                break;
            }
        }
        deviation
    }
 */
struct Solution;
use std::collections::BinaryHeap;
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let nums: Vec<i32> = nums
            .into_iter()
            .map(|x| if x % 2 == 0 { x } else { 2 * x })
            .collect();
        let mut min_elem = nums.iter().min().unwrap().to_owned();

        let mut heap = BinaryHeap::from_iter(nums.into_iter());
        let mut min_diff = i32::MAX;

        while let Some(max_elem) = heap.pop() {
            min_diff = min_diff.min(max_elem - min_elem);
            // The only way diff is min is if max_elem is odd, which means it can't be reduced down further.
            if max_elem == min_elem {
                return 0;
            }
            if max_elem % 2 != 0 {
                return min_diff;
            }
            min_elem = min_elem.min(max_elem / 2);
            heap.push(max_elem / 2);
        }
        min_diff
    }
}

fn main() {
    check_eq!(Solution::minimum_deviation(vec![3, 5]), 1);
}
