struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for i in 0..1 << nums.len() {
            let mut local = vec![];
            for bit in 0..nums.len() {
                if i & (1 << bit) != 0 {
                    local.push(nums[bit]);
                }
            }
            res.push(local);
        }
        res
    }
}

fn main() {
    assert_eq!(
        Solution::subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ]
    );

    assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]]);

    assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
}
