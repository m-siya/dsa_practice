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

impl Solution {
    // Subarray as if the array was non-secular.
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut sum = 0;
        for &num in nums.iter() {
            sum += num;
            max_sum = max_sum.max(sum);
            if sum < 0 {
                sum = 0;
            }
        }
        max_sum
    }

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut postfix_sum: Vec<i32> = vec![i32::MIN; nums.len() + 1];
        // Populate the postfix sum.
        let mut max_postfix_sum = i32::MIN;
        let mut sum = 0;
        for (postfix_val, &num) in postfix_sum.iter_mut().rev().zip(nums.iter().rev()) {
            *postfix_val = max_postfix_sum;
            sum += num;
            max_postfix_sum = max_postfix_sum.max(sum);
        }
        postfix_sum[0] = max_postfix_sum;
        // println!("Back sum = {:?}", postfix_sum);
        // Now get the best circular subarray sum by trying out all the combinations of subslices from beginning and from end.
        let mut overall_max = i32::MIN as i64;
        let mut prefix_sum = i32::MIN as i64;
        let mut sum = 0;
        for (i, &val) in nums.iter().enumerate() {
            // println!("{i}");
            sum += val;
            prefix_sum = prefix_sum.max(sum as i64);
            let postfix_val = postfix_sum[i + 1];
            overall_max = overall_max.max(prefix_sum + postfix_val as i64);
        }
        let overall_max = overall_max as i32;
        overall_max.max(Self::max_sub_array(nums))
    }
}

fn main() {
    check_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    check_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
    check_eq!(Solution::max_subarray_sum_circular(vec![5, 1, -3, 5]), 11);
    check_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
}
