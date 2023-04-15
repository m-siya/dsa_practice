use easy_io::OutputWriter;
use easy_io::InputReader;

fn iseven(num: u32) -> bool {
    num & 1 == 0
}

fn solution(nums: &[u64]) {
    let mut subarr_sum = 0i128;
    let mut coeff = -((nums.len() - 2) as i64);
    for (i,&elem) in nums.iter().enumerate() {
        subarr_sum += (coeff * (elem as i64)) as i128;
        coeff += 2;
        dbg!(subarr_sum);
    }

}

fn main() {
    let mut input = InputReader::from_file("input.txt");
    let test_cases = input.next::<u16>();
    for _ in 0..test_cases {
        let n: u32 = input.next();
        let mut nums = Vec::with_capacity(n as usize);
        let mut acc = 0u64;
        for _ in 0..n {
            let elem = input.next::<u32>();
            if iseven(elem) {
                acc += 1;
            }
            nums.push(acc);
        }

        solution(&*nums);
    }
}