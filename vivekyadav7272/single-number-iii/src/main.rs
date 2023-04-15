struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let all_xor = nums.iter().fold(0, |acc, x| acc ^ x);
        // Now we have x^y.

        let mask = all_xor & -all_xor;
        // This mask gives us the last one bit as mask, which we need to check for all the
        // numbers with that bit set.

        let x = nums
            .iter()
            .fold(0, |acc, &x| if (x & mask) != 0 { acc ^ x } else { x });
        // This gives us x, because y doesn't have that bit set fo'sure (otherwise that bit wouldn't be set)
        // in x^y. The other numbers are in pairs, so they would cancel out.

        // Now we have x, just xor it in x^y to get y.
        let y = x ^ all_xor;

        vec![x, y]
    }
}

fn main() {
    println!("Hello, world!");
}
