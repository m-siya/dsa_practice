use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums_vec = nums;
        let mut nums = HashSet::<i32>::with_capacity(nums_vec.len());
        for num in nums_vec {
            nums.insert(num);
        }

        let mut max_len = 0i32;
        for elem in nums.iter() {
            if nums.contains(&(elem-1)) {continue;}

            let mut len = 1;
            while nums.contains(&(elem+len)) {
                len += 1;
            }

            max_len = max_len.max(len);
        }

        max_len
    }
}

fn main() {
    let lcs = Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]);
    dbg!(lcs);
}
