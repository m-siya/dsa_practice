use std::collections::{HashMap, HashSet, VecDeque};
pub struct Solution2;
impl Solution2 {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let indices: HashMap<_, _> = stones
            .iter()
            .enumerate()
            .map(|(i, &s)| (s as usize, i))
            .collect();
        let mut sets: VecDeque<HashSet<_>> = (0..stones.len()).map(|_| HashSet::new()).collect();
        sets[0].insert(1);

        let mut last = HashSet::new();
        for (i, &s) in stones.iter().enumerate() {
            last = sets.pop_front().unwrap();
            for &k in &last {
                if let Some(&idx) = indices.get(&(s as usize + k)) {
                    sets[idx - i - 1].insert((k - 1).max(1));
                    sets[idx - i - 1].insert(k);
                    sets[idx - i - 1].insert(k + 1);
                }
            }
        }

        return last.len() > 0;
    }
}
