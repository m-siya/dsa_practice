struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut start = 0;
        let mut total_subarrs = 0;

        for (i, &num) in nums.iter().chain(std::iter::once(&i32::MAX)).enumerate() {
            if num < min_k || num > max_k {
                total_subarrs += Self::magic_fn(&nums[start..i], min_k, max_k);
                start = i + 1;
            }
        }

        total_subarrs
    }

    fn magic_fn(arr: &[i32], min_k: i32, max_k: i32) -> i64 {
        let mut min_latest_ind = -1i64;
        let mut max_latest_ind = -1i64;

        arr.iter().enumerate().fold(0, |acc, (i, &x)| {
            if x == min_k {
                min_latest_ind = i as i64;
            }
            if x == max_k {
                max_latest_ind = i as i64;
            }
            acc + min_latest_ind.min(max_latest_ind) + 1
        })
    }
}

fn main() {
    assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 1], 1, 5), 5);
    assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 1, 5], 1, 5), 10);
    assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    assert_eq!(
        Solution::count_subarrays(
            vec![
                1, 3, 5, 2, 7, 5, 7, 7, 1, 1, 1, 5, 1, 7, 1, 1, 1, 8, 5, 5, 2, 4, 2, 2, 1, 7, 8, 3,
                2, 1, 4, 9, 1, 7, 8, 5, 2, 3, 1, 5,
            ],
            1,
            5,
        ),
        16
    );
    assert_eq!(
        Solution::count_subarrays(
            vec![
                8, 8, 1, 3, 5, 2, 7, 5, 7, 7, 1, 1, 1, 5, 1, 7, 1, 1, 1, 8, 5, 5, 3, 4, 2, 2, 1, 7,
                8, 3, 2, 1, 4, 9, 1, 7, 8, 5, 2, 3, 1, 5, 5, 5, 8,
            ],
            1,
            5,
        ),
        24
    );
}
