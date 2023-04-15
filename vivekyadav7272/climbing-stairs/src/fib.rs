pub struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let temp = b;
            b = a + b;
            a = temp;
        }
        b
    }
}
