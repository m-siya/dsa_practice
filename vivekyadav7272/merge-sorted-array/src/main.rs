struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut nums1_ind = (m - 1) as isize;
        let mut nums2_ind = (n - 1) as isize;
        let mut fill_ind = nums1.len() - 1;
        while nums1_ind > -1 && nums2_ind > -1 {
            let num1 = nums1[nums1_ind as usize];
            let num2 = nums2[nums2_ind as usize];
            if num1 > num2 {
                nums1[fill_ind] = num1;
                nums1_ind -= 1;
            } else {
                nums1[fill_ind] = num2;
                nums2_ind -= 1;
            }
            fill_ind -= 1;
        }

        while nums2_ind > -1 {
            nums1[fill_ind] = nums2[nums2_ind as usize];
            nums2_ind -= 1;
            fill_ind -= 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
