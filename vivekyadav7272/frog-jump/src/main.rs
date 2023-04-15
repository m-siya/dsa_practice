mod lc_soln;
struct Solution;
use std::collections::HashSet;
impl Solution {
    /*
    B |B| |B| |B|B| |B| | | |B| | | | |B| | | | | |
    */

    pub fn can_cross(stones: Vec<i32>) -> bool {
        assert!(stones.len() > 1);
        // Early exit condition if a stone is impossibly far (i.e more far than the
        // previous stone is from the first stone, i.e twice as far as previous stone)
        for two_stones in stones.windows(2) {
            if two_stones[1] > 2 * two_stones[0] + 1 {
                return false;
            }
        }

        let mut failures = HashSet::<(i32, i32)>::with_capacity(stones.len());
        if stones[1] != 1 {
            return false;
        }
        Self::_can_cross(&mut failures, &stones[1..], 1)
    }

    fn _can_cross(failures: &mut HashSet<(i32, i32)>, stones: &[i32], last_jump: i32) -> bool {
        assert!(stones.len() > 0);
        if stones.len() <= 1 {
            return true;
        }

        for jump in [last_jump - 1, last_jump, last_jump + 1] {
            if jump == 0 || failures.contains(&(stones[0], last_jump)) {
                continue;
            }
            if let Ok(next_stone_ind) = stones.binary_search(&(stones[0] + jump)) {
                if Self::_can_cross(failures, &stones[next_stone_ind..], jump) {
                    return true;
                }
            }
        }

        failures.insert((stones[0], last_jump));

        false
    }
}

fn main() {
    dbg!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]));
    dbg!(Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]));
}
