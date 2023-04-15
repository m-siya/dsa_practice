struct Solution;
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
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut start = 0usize;
        let mut end = arr.len() - 1;
        let mut mid = (start + end) / 2;
        if k < arr[0] as usize {
            return k as i32;
        }

        while start <= end {
            let missing_nums = Self::missing_nums(&arr, mid);
            if missing_nums >= k {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
            mid = (start + end) / 2;
        }

        let missing_in_mid = Self::missing_nums(&arr, mid);
        if missing_in_mid < k {
            (arr[mid] as usize + (k - missing_in_mid)) as i32
        } else {
            (arr[mid - 1] as usize + (k - Self::missing_nums(&arr, mid - 1))) as i32
        }
    }

    fn missing_nums(arr: &[i32], k: usize) -> usize {
        (arr[k] as usize - k - 1) as usize
    }
}

fn main() {
    // println!("{}", &[2, 3, 4, 6, 11].partition_point(|&x| x <= 5));
    check_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    check_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    check_eq!(Solution::find_kth_positive(vec![2, 3], 1), 1);
    check_eq!(Solution::find_kth_positive(vec![2, 3], 4), 6);
    check_eq!(Solution::find_kth_positive(vec![2], 2), 3);
    check_eq!(Solution::find_kth_positive(vec![1, 3, 5], 2), 4);
}
