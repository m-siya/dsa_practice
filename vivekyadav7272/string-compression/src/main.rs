struct Solution;
macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        println!("`{left_stringified}` expected to be `{:?}`", $right);
        let expr = $left;
        let ans = $right;
        assert_eq!(expr, ans);
    };
}
// impl Solution {
//     // pub fn compress(chars: &mut Vec<char>) -> i32 {
//     //     let mut arr_count = 0;
//     //     let (_, last_chr_count) =
//     //         chars
//     //             .iter()
//     //             .skip(1)
//     //             .fold((chars[0], 1), |(prev_ch, chr_count), &curr_ch| {
//     //                 if curr_ch == prev_ch {
//     //                     (prev_ch, chr_count + 1)
//     //                 } else {
//     //                     arr_count += 1 + f32::log10(chr_count as f32).ceil() as i32;
//     //                     (curr_ch, 1)
//     //                 }
//     //             });
//     //     arr_count += 1 + f32::log10(last_chr_count as f32).ceil() as i32;

//     //     arr_count
//     // }
// }

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        chars.push('\0');
        let mut write_point = 0;
        let mut first_occur = chars[0];
        let mut cur_ch_count: u32 = 0;
        for curr_ch in 0..chars.len() {
            if chars[curr_ch] != first_occur {
                chars[write_point] = first_occur;
                write_point += 1;
                if cur_ch_count != 1 {
                    let last_write_point = write_point;
                    assert!(write_point < chars.len());
                    while cur_ch_count != 0 {
                        assert!(write_point < chars.len());
                        chars[write_point] = ((cur_ch_count % 10) as u8 + ('0' as u8)) as char;
                        cur_ch_count /= 10;
                        write_point += 1;
                    }
                    chars[last_write_point..write_point].reverse();
                }
                first_occur = chars[curr_ch];
                cur_ch_count = 1;
            } else {
                cur_ch_count += 1;
            }
        }

        write_point as i32
    }
}

fn main() {
    let mut a_1000: Vec<char> = std::iter::once('a').cycle().take(1000).collect();
    let len = Solution::compress(&mut a_1000);
    check_eq!(&a_1000[..len as usize], &['a', '1', '0', '0', '0']);
}
