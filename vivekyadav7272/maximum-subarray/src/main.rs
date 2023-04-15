pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    let mut sum = 0;
    for &num in nums.iter().cycle().take(nums.len()*2) {
        sum += num;
        max_sum = max_sum.max(sum);
        if sum < 0 {
            sum = 0;
        }
    }
    max_sum
}

fn main() {
    dbg!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
    dbg!(max_sub_array(vec![1]));
    dbg!(max_sub_array(vec![5,4,-1,7,8]));
}
