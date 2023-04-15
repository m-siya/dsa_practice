struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;

        while low < high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid];
            if mid_num > target {
                high = mid;
            } else if mid_num < target {
                low = mid;
            } else {
                return mid as i32;
            }
        }

        -1
    }
}
fn main() {
    println!("Hello, world!");
}
