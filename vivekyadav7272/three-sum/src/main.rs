struct Solution;
impl Solution {
    fn two_sum(soln_set: &mut Vec<Vec<i32>>, target: i32, start_ind: usize, list: &[i32]) {
        println!("Currently on target: {target}");
        let mut i = start_ind;
        let mut j = list.len() - 1;

        while i < j {
            while i < j && list[i] + list[j] < target {
                i += 1;
            }

            while i < j && list[i] + list[j] > target {
                j -= 1;
            }

            if i < j && list[i] + list[j] == target {
                soln_set.push(vec![-target, list[i], list[j]]);
            }

            i += 1;
            j -= 1;
        }
    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        println!("Starting {nums:?}...");
        nums.sort_unstable();
        let mut soln_set: Vec<Vec<i32>> = Vec::with_capacity(nums.len() / 3);
        // Only yield the first non-duplicate value.
        let mut i = 0;
        while i < nums.len() {
            while (1..nums.len()).contains(&i) && nums[i] == nums[i - 1] {
                i += 1;
            }
            Self::two_sum(&mut soln_set, -nums[i], i + 1, &nums);
            i += 1;
        }

        println!("Finished {nums:?}..");

        soln_set
    }
}
// [-4, -1, -1, 0, 1, 2]
// [0, 1, 1]
fn main() {
    dbg!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    dbg!(Solution::three_sum(vec![0, 1, 1]));
    dbg!(Solution::three_sum(vec![0, 0, 0]));
}
