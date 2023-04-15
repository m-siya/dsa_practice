struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = [0; 3];

        for &num in nums.iter() {
            counts[num as usize] += 1;
        }

        let mut prev_count = 0;
        for (col, &count) in counts.iter().enumerate() {
            nums[prev_count..prev_count + count].fill(col as i32);
            prev_count = prev_count + count;
        }
    }
}

fn main() {
    println!("Hello, World!");
}
