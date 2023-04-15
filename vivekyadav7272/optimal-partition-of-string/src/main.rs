struct Solution;

impl Solution {
    fn get(c: u8) -> usize {
        (c - b'a') as usize
    }

    pub fn partition_string(s: String) -> i32 {
        let mut visited = [false; 26];
        s.bytes().fold(1, |substrs, c| {
            let ans = if visited[Self::get(c)] {
                visited.fill(false);
                substrs + 1
            } else {
                substrs
            };
            visited[Self::get(c)] = true;
            ans
        })
    }
}
fn main() {
    println!("Hello, world!");
}
