pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut rem_counts = vec![0usize; k as usize];
    rem_counts[0] = 1;
    let cumsum = nums.iter().scan(0, |acc, &num| {
        *acc += num;
        Some(*acc)
    });
    println!("{cumsum:?}");

    let mut count = 0usize;

    for cum in cumsum {
        let mut rem = cum%k;
        if rem < 0 {
            rem = k + rem;
        }
        assert!(rem < k);
        count += rem_counts[rem as usize];
        rem_counts[rem as usize] += 1;
    }

    count as i32
}

fn main() {
    dbg!(subarrays_div_by_k(vec![4,5,0,-2,-3,1], 5));
    dbg!(subarrays_div_by_k(vec![5], 9));
}