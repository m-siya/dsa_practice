struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = Vec::with_capacity(s.len() / 2);
        for c in s.bytes() {
            match c {
                b')' | b']' | b'}' => {
                    if (st.pop().unwrap_or(0) ^ c) >> 4 != 0 {
                        return false;
                    }
                }
                _ => st.push(c),
            }
        }

        st.is_empty()
    }
}
fn main() {
    println!("Hello, world!");
}
