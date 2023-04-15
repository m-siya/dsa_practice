struct Solution;

use std::collections::HashMap;

fn get(c: u8) -> usize {
    (c - b'a') as usize
}
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = HashMap::<(usize, usize), i32>::new();
        let mut occurences: Vec<Vec<usize>> = vec![vec![]; 26];

        for (i, t2_i) in text2.bytes().enumerate() {
            occurences[get(t2_i)].push(i);
        }

        Self::_longest_common_subsequence(
            text1.as_bytes(),
            text2.as_bytes(),
            0,
            &mut dp,
            &occurences,
        )
    }

    fn _longest_common_subsequence(
        text1: &[u8],
        text2: &[u8],
        text2_begin: usize,
        dp: &mut HashMap<(usize, usize), i32>,
        occurences: &Vec<Vec<usize>>,
    ) -> i32 {
        if text1.len() == 0 || text2.len() == 0 {
            return 0;
        }

        if let Some(&ans) = dp.get(&(text1.len(), text2.len())) {
            return ans;
        }

        let mut matches = 0usize;
        for (t1_i, t2_i) in text1.into_iter().zip(text2.into_iter()) {
            if t1_i == t2_i {
                matches += 1;
            } else {
                break;
            }
        }

        if matches != 0 {
            let ans = Self::_longest_common_subsequence(
                &text1[matches..],
                &text2[matches..],
                text2_begin + matches,
                dp,
                occurences,
            );
            let ans = matches as i32 + ans;
            dp.insert((text1.len(), text2.len()), ans);
            // dp[text1.len() - 1][text2.len() - 1] = Some(ans);
            ans
        } else {
            let mut ans = 0;
            for (i, &t1_i) in text1.iter().enumerate() {
                let pos_in_table = occurences[get(t1_i)].partition_point(|&x| x < text2_begin);
                if let Some(&pos) = occurences[get(t1_i)].get(pos_in_table) {
                    ans = ans.max(Self::_longest_common_subsequence(
                        &text1[i..],
                        &text2[pos - text2_begin..],
                        text2_begin + pos,
                        dp,
                        occurences,
                    ));
                }
            }
            dp.insert((text1.len(), text2.len()), ans);
            // dp[text1.len() - 1][text2.len() - 1] = Some(ans);
            ans
        }
    }
}
fn main() {
    println!("Hello, world!");
}
