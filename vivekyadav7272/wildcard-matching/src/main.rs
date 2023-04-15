struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = HashSet::<(usize, usize)>::new();
        let mut p = p.into_bytes();
        p.dedup_by(|x, y| x == y && *y == b'*');
        Self::_is_match(s.as_bytes(), p.as_slice(), &mut dp)
    }

    fn _is_match(s: &[u8], p: &[u8], dp: &mut HashSet<(usize, usize)>) -> bool {
        if p.len() == 0 && s.len() != 0 {
            return false;
        }

        if !dp.insert((s.len(), p.len())) {
            return false;
        }

        if s.len() == 0 {
            let ans = p.iter().all(|&x| x == b'*');
            return ans;
        }

        let p_first = p[0];
        let mut matching_pairs = 0;
        for (&s_i, &p_i) in s.into_iter().zip(p.into_iter()) {
            if s_i == p_i || p_i == b'?' {
                matching_pairs += 1;
            } else {
                break;
            }
        }

        if matching_pairs != 0 {
            return Self::_is_match(&s[matching_pairs..], &p[matching_pairs..], dp);
        }

        if p_first != b'*' {
            return false;
        }

        let possibility_1 = Self::_is_match(&s[1..], p, dp);
        if possibility_1 {
            return true;
        }

        let possibility_2 = Self::_is_match(s, &p[1..], dp);
        possibility_2
    }
}
fn main() {
    println!("Hello, world!");
}
