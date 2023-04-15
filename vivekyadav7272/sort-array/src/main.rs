macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        println!("`{left_stringified}` expected to be `{:?}`", $right);
        let expr = $left;
        let ans = $right;
        assert_eq!(expr, ans);
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

struct Solution;
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        Self::_sort_array(&mut nums, 12);
        nums
    }
    fn _sort_array(nums: &mut [i32], magic_num: usize) {
        if nums.len() == 0 {
            return;
        }
        let new_pivot = Self::partition(nums, magic_num);
        Self::_sort_array(&mut nums[..new_pivot], new_pivot + magic_num);
        Self::_sort_array(&mut nums[new_pivot + 1..], new_pivot + magic_num);
    }
    fn partition(nums: &mut [i32], pivot: usize) -> usize {
        let pivot = pivot % nums.len();
        // protecc'ing
        nums.swap(nums.len() - 1, pivot);
        let pivot = nums.len() - 1;
        let pivot_elem = nums[pivot];
        let mut he_who_remains = -1isize;
        // dbg_print!(&nums);
        for kraken in 0..nums.len() {
            if nums[kraken] <= pivot_elem {
                he_who_remains += 1;
                nums.swap(he_who_remains as usize, kraken);
            }
        }
        // dbg_print!(&nums);
        he_who_remains as usize
    }
}

fn test(len: usize) {
    for _ in 0..100 {
        let arr = (0..len).map(|_| fastrand::i32(..)).collect::<Vec<i32>>();
        let mut sorted = arr.clone();
        sorted.sort_unstable();
        check_eq!(Solution::sort_array(arr), &*sorted);
    }
}

fn main() {
    for i in 0..100 {
        test(i);
    }
    // let arr = vec![1302415089, -2318977, -455165501];
    // let mut sorted = arr.clone();
    // sorted.sort_unstable();
    // check_eq!(Solution::sort_array(arr), &*sorted);
}
