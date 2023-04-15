struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut s_reversed = s.clone().into_bytes();
        s_reversed.reverse();
        Self::_longest_common_subsequence(
            s.as_bytes(),
            &s_reversed,
            &mut vec![vec![None::<i32>; s.len()]; s.len()],
        )
    }

    fn _longest_common_subsequence(
        text1: &[u8],
        text2: &[u8],
        dp: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        if text1.len() == 0 || text2.len() == 0 {
            return 0;
        }

        if let Some(ans) = dp[text1.len() - 1][text2.len() - 1] {
            return ans;
        }

        let mut matches = 0i32;
        for (t1_i, t2_i) in text1.into_iter().zip(text2.into_iter()) {
            if t1_i == t2_i {
                matches += 1;
            } else {
                break;
            }
        }
        if matches != 0 {
            let ans = Self::_longest_common_subsequence(
                &text1[matches as usize..],
                &text2[matches as usize..],
                dp,
            );
            let ans = matches + ans;
            dp[text1.len() - 1][text2.len() - 1] = Some(ans);
            ans
        } else {
            let possibility_1 = Self::_longest_common_subsequence(&text1[1..], text2, dp);
            let possibility_2 = Self::_longest_common_subsequence(text1, &text2[1..], dp);
            let ans = possibility_1.max(possibility_2);
            dp[text1.len() - 1][text2.len() - 1] = Some(ans);
            ans
        }
    }
}
fn main() {
    println!("Hello, world!");
}
