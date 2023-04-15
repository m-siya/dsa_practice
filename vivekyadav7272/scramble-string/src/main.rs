macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        let expr = $left;
        let ans = $right;
        if expr != ans {
            println!(
                "`{left_stringified}` expected to be `{:?}`, but was {:?}",
                $right, expr
            );
        }
    };
}
struct Solution;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::_is_scramble(&s1, &s2, &mut HashMap::new())
    }

    fn _is_scramble<'a, 'b>(
        s1: &'a str,
        s2: &'a str,
        dp: &'b mut HashMap<(&'a str, &'a str), bool>,
    ) -> bool {
        if s1.len() <= 1 || s2.len() <= 1 {
            return s1 == s2;
        }

        let mut char_frequencies_1 = [0; 26];
        let mut char_frequencies_2 = [0; 26];

        for c in s1.bytes() {
            char_frequencies_1[(c - b'a') as usize] += 1;
        }

        for c in s2.bytes() {
            char_frequencies_2[(c - b'a') as usize] += 1;
        }

        if !char_frequencies_1
            .iter()
            .zip(char_frequencies_2.iter())
            .all(|(x, y)| x == y)
        {
            return false;
        }
        if let Some(&is_scramble) = dp.get(&(s1, s2)) {
            return is_scramble;
        }
        for i in 1..s1.len() {
            let (s1_left, s1_right) = s1.split_at(i);
            let (s2_left, s2_right) = s2.split_at(i);
            let (s2_rev_left, s2_rev_right) = s2.split_at(s1.len() - i);
            if (Self::_is_scramble(s1_left, s2_left, dp)
                && Self::_is_scramble(s1_right, s2_right, dp))
                || (Self::_is_scramble(s1_left, s2_rev_right, dp)
                    && Self::_is_scramble(s1_right, s2_rev_left, dp))
            {
                dp.insert((s1, s2), true);
                return true;
            }
        }
        dp.insert((s1, s2), false);
        false
    }
}
fn main() {
    check_eq!(Solution::is_scramble("great".into(), "rgeat".into()), true);
}
