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

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut jmp_table: HashMap<i32, Vec<isize>> = HashMap::with_capacity(arr.len() / 3);
        for (i, &num) in arr.iter().enumerate() {
            jmp_table.entry(num).or_insert(vec![]).push(i as isize);
        }
        Self::_min_jumps(&arr, jmp_table)
    }

    fn _min_jumps(arr: &[i32], mut jmp_table: HashMap<i32, Vec<isize>>) -> i32 {
        let mut queue = VecDeque::with_capacity(arr.len() / 2);
        queue.push_back(0);
        let mut visited = HashSet::with_capacity(arr.len());
        let mut curr_dist = 0;
        loop {
            let curr_len = queue.len();
            for _ in 0..curr_len {
                let curr_ind = queue.pop_front().unwrap();
                if curr_ind == (arr.len() - 1) as isize {
                    return curr_dist;
                }
                for &ind in [curr_ind - 1, curr_ind + 1].iter() {
                    if ind < 0 || ind >= arr.len() as isize || visited.contains(&ind) {
                        continue;
                    }
                    visited.insert(curr_ind);
                    queue.push_back(ind);
                }
                if let Some(comrades) = jmp_table.remove(&arr[curr_ind as usize]) {
                    // We can directly put into the queue without putting in checks
                    // like we did above because, if I have my comrades,
                    // that means they didn't visit before me (loyal ain't they)
                    // so no "visited.contains" check required.
                    // Also all indices are valid :)
                    // We can avoid putting in the curr_ind redundancy check as this node is visited already
                    // so only a bit of space will be wasted.
                    queue.push_back(comrades[0]);
                    for consecutive_comrades in comrades.windows(2) {
                        let prev_comrade = consecutive_comrades[0];
                        let curr_comrade = consecutive_comrades[1];
                        if curr_comrade != prev_comrade + 1 {
                            queue.push_back(curr_comrade);
                        }
                    }
                    queue.push_back(comrades[comrades.len() - 1]);

                    // We can avoid putting in the curr_ind redundancy check as that will be handled
                    // by set itself.
                    visited.extend(comrades);
                }
            }
            curr_dist += 1;
        }
    }
}

fn main() {
    check_eq!(
        Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
        3
    );
    check_eq!(Solution::min_jumps(vec![7]), 0);
    check_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
}
