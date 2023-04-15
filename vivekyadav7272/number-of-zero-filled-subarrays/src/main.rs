use core::num;

struct Solution;

// #LCSTART https://leetcode.com/problems/number-of-zero-filled-subarrays/

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut num_subarrays = 0;
        let mut last_zero = 0usize;
        for (i, &elem) in nums.iter().chain(std::iter::once(&i32::MAX)).enumerate() {
            if elem != 0 {
                let slice_len = i - last_zero;
                num_subarrays += slice_len * (slice_len + 1) / 2;
                last_zero = i + 1;
            }
        }
        num_subarrays as i64
    }
}

// #LCEND

fn main() {
    println!("Hello, world!");
}
