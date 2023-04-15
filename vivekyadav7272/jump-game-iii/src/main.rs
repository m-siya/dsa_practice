struct Solution;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        const MAX_LEN: usize = 50000;
        let mut dp = vec![false; MAX_LEN];
        let mut q = VecDeque::new();
        q.push_back(start as usize);
        loop {
            let front = match q.pop_front() {
                Some(front) => front,
                None => return false,
            };
            if dp[front] {
                continue;
            }
            dp[front] = true;
            let jmp = arr[front];
            if jmp == 0 {
                return true;
            }

            if front >= jmp as usize {
                q.push_back(front - jmp as usize);
            }

            if (front + jmp as usize) < arr.len() {
                q.push_back(front + jmp as usize);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
