struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut acc = 0;
        let mut result = 0;
        let mut hasher = HashMap::<i32, i32>::with_capacity(nums.len() + 1);
        hasher.insert(0, 1);
        for &num in nums.iter() {
            acc += num;
            let target = acc - k;
            if let Some(&val) = hasher.get(&target) {
                result += val;
            }
            *hasher.entry(acc).or_insert(0) += 1;
        }
        result
    }
}

fn main() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 4), 0);
    assert_eq!(Solution::subarray_sum(vec![1, -1, 0], 0), 3);
    assert_eq!(
        Solution::subarray_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0),
        55
    );
}
