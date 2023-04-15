struct Solution;
use std::collections::HashSet;
fn index(c: char) -> usize {
    (c as u32 - 'a' as u32) as usize
}
impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut total_count = 0i64;
        let old_names: HashSet<(char, &str)> = HashSet::from_iter(
            ideas
                .iter()
                .map(|idea| (idea.as_bytes()[0] as char, &idea[1..])),
        );
        let mut sex_appeal: [[i64; 26]; 26] = [[0; 26]; 26];

        for idea in ideas.iter() {
            assert!(idea.len() > 0);
            let cc = index(idea.as_bytes()[0] as char);
            for c in 'a'..='z' {
                if !old_names.contains(&(c, &idea[1..])) {
                    sex_appeal[index(c)][cc] += 1;
                }
            }
        }

        for i in 0..26 {
            for j in 0..26 {
                total_count += sex_appeal[i][j] * sex_appeal[j][i];
            }
        }
        total_count as i64
    }
}
fn main() {
    for c in 'a'..'z' {
        println!("{c}");
    }
}
