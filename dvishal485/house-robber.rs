struct Solution {}

impl Solution {
    fn steal(nums: &[i32], steal_from: usize, cache: &mut [i32]) -> i32 {
        match cache[steal_from] {
            -1 => {
                let mut neighbours_moni = 0;
                for new_house in (steal_from + 2..nums.len()).into_iter() {
                    neighbours_moni = neighbours_moni.max(Self::steal(nums, new_house, cache));
                }
                let val = nums[steal_from] + neighbours_moni;
                cache[steal_from] = val;
                val
            }
            val => val,
        }
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => nums[1].max(nums[0]),
            _ => {
                let mut moni = 0;
                let mut cache = [-1i32; 100];
                for thief in [0, 1] {
                    moni = moni.max(Self::steal(&nums, thief, &mut cache));
                }
                moni
            }
        }
    }
}

fn main() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(
        Solution::rob(vec![
            114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170, 110,
            236, 81, 90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64, 214, 228,
            78, 188, 67, 205, 94, 205, 169, 241, 202, 144, 240
        ]),
        4173
    );
}
