struct Solution;
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
        let expr = $left;
        let ans = $right;
        if expr != ans {
            println!("`{left_stringified}` was not `{:?}`", $right);
        }
    };
}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        dbg_print!((dividend, divisor));
        let divisor_abs = if let Some(x) = divisor.checked_abs() {
            x as u32
        } else {
            u32::MAX / 2 + 1
        };
        let dividend_abs = if let Some(x) = dividend.checked_abs() {
            x as u32
        } else {
            dbg_print!(u32::MAX / 2) + 1
        };

        let ans = dbg_print!(Self::_divide(dividend_abs, divisor_abs));
        if !((dividend < 0 && divisor < 0) || (dividend > 0 && divisor > 0)) {
            if ans == u32::MAX / 2 + 1 {
                i32::MIN
            } else {
                -(ans as i32)
            }
        } else {
            if ans == u32::MAX / 2 + 1 {
                i32::MAX
            } else {
                ans as i32
            }
        }
    }

    fn _divide(mut dividend: u32, divisor: u32) -> u32 {
        if divisor == 1 {
            return dividend;
        }
        let mut count = 0;
        while dividend >= divisor {
            let mut beast_mode_count = 0;
            while dividend >= (divisor << beast_mode_count) {
                beast_mode_count += 1;
                if (divisor << beast_mode_count) < divisor {
                    break;
                }
            }
            beast_mode_count -= 1;
            dividend -= divisor << beast_mode_count;
            count += 1 << beast_mode_count;
        }
        dbg_print!(count)
    }
}
fn fuzzer(test_cases: usize) {
    for _ in 0..test_cases {
        let dividend = fastrand::i32(..);
        let divisor = fastrand::i32(..);
        let ans = Solution::divide(dividend, divisor);
        let true_ans = dividend / divisor;
        if ans != true_ans {
            println!(
                "{dividend}/{divisor} should be {}, but your answer is {}",
                true_ans, ans
            );
        }
    }
}

fn main() {
    // fuzzer(u32::MAX as usize);
    assert_eq!(Solution::divide(-2147483648, 2), -2147483648 / 2);
    assert_eq!(Solution::divide(i32::MAX, 1), i32::MAX / 1);
    assert_eq!(Solution::divide(i32::MAX, -1), i32::MAX / -1);
    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN / 1);
}
