struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut prev_elem = i32::MIN;
        // let mut prev_elem_ind = nums.len();
        let mut curr_ind = nums.len();
        for i in (0..nums.len()).rev() {
            curr_ind = i;
            let elem = nums[i];
            if elem > prev_elem {
                prev_elem = elem;
                continue;
            }
            // if elem == prev_elem {continue;}

            // Now found anomaly..

            // Find just_bigger.
            let mut min = std::i32::MAX;
            let mut min_ind = -1;
            for j in (i + 1)..nums.len() {
                let candidate = nums[j];
                if candidate < min && candidate > elem {
                    min = candidate;
                    min_ind = j as i32;
                }
            }
            // println!("{:?}", nums);
            // dbg!(min_ind);
            // dbg!(min);
            assert!(min_ind >= 0);
            // Now swap 'em.
            nums.swap(i, min_ind as usize);
            break;
        }

        // assert!(0 <= prev_elem_ind && (prev_elem_ind as usize) < nums.len());

        nums[curr_ind + 1 as usize..].reverse();
    }
}
fn main() {
    let mut a = vec![1, 3, 2];
    Solution::next_permutation(&mut a);
    dbg!(a);
}
