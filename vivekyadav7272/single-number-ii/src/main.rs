struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bitcount = [0usize; 32];
        let mut single_num = 0i32;

        for i in 0..32 {
            for &num in nums.iter() {
                bitcount[i] += ((num >> i) & 1) as usize;
            }
        }

        for i in 0..32 {
            if bitcount[i] % 3 != 0 {
                single_num += 1 << i;
            }
        }
        single_num
    }

    pub fn single_number2(nums: Vec<i32>) -> i32 {
        let mut ones = 0i32;
        let mut twos = 0i32;

        for num in nums {
            twos = twos | (ones & num);
            ones = ones ^ num;

            let threes = ones & twos;

            ones = ones ^ threes;
            twos = twos ^ threes;
        }

        ones
    }
}

fn main() {
    dbg!(Solution::single_number2(vec![2, 2, 3, 2]));
    dbg!(Solution::single_number2(vec![0, 1, 0, 1, 0, 1, 99]));
}
