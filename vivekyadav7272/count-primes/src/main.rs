struct Solution;
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 1 {
            return 0;
        }
        let n = n as usize;
        let mut killed = vec![false; n];
        assert!(killed.len() == n);
        for i in 2..(((n as f32).sqrt() + 1 as f32) as usize) {
            if killed[i] {
                continue;
            }
            let mut j = i * 2;
            while j < n {
                killed[j] = true;
                j += i;
            }
        }

        killed[2..n]
            .into_iter()
            .filter(|&&isKilled| !isKilled)
            .count() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
