struct Solution;
impl Solution {
    fn map(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut total_number = 0i32;
        let mut prev = i32::MAX;
        for c in s.chars() {
            let c = Self::map(c);
            if prev < c {
                total_number = total_number - prev + (c - prev);
            }
            prev = c;
            total_number += c;
        }
        total_number
    }
}
fn main() {
    println!("Hello, world!");
}
