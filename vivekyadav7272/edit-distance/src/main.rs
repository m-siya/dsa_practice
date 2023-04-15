#[cfg(debug_assertions)]
macro_rules! dbg_print {
    ($garbage:expr) => {
        dbg!($garbage)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_print {
    ($garbage:expr) => {
        $garbage
    };
}

macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        println!("`{left_stringified}` expected to be `{}`", $right);
        let expr = $left;
        let ans = $right;
        assert_eq!(expr, ans);
    };
}

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![-1; word2.len()]; word1.len()];
        Self::_min_distance(&mut dp, word1.as_bytes(), word2.as_bytes())
    }

    fn _min_distance(dp: &mut Vec<Vec<i32>>, word1: &[u8], word2: &[u8]) -> i32 {
        let dist = dp[word1.len()][word2.len()];
        if dist != -1 {
            return dist;
        }

        let mut i = 0;
        for (c1, c2) in word1.iter().zip(word2.iter()) {
            if c1 != c2 {
                break;
            }
            i += 1;
        }

        if i == word1.len().min(word2.len()) {
            let ans = (word1.len() as i32 - word2.len() as i32).abs() as i32;
            dp[word1.len()][word2.len()] = ans;
            return ans;
        }

        let ans = 1
            + (Self::_min_distance(dp, &word1[i + 1..], &word2[i + 1..])
                .min(Self::_min_distance(dp, &word1[i + 1..], &word2[i..]))
                .min(Self::_min_distance(dp, &word1[i..], &word2[i + 1..])));

        dp[word1.len()][word2.len()] = ans;

        ans
    }
}

fn main() {
    check_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
    check_eq!(
        Solution::min_distance("intention".into(), "execution".into()),
        5
    );
}
