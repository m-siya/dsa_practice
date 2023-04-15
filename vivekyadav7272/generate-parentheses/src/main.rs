use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut final_ans: HashSet<String> = HashSet::new();
        Self::_generate_parenthesis(&mut final_ans, n)
    }

    fn _generate_parenthesis(hs: &mut HashSet<String>, n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_string()];
        }

        let prev_ans = Self::_generate_parenthesis(hs, n - 1);
        let mut currans = Vec::with_capacity(prev_ans.len() * 3);
        for oldans in prev_ans.into_iter() {
            let new_answers = [
                "(".to_string() + &*oldans + ")",
                oldans.clone() + "()",
                "()".to_string() + &*oldans,
            ];
            for new_ans in new_answers {
                if !hs.contains(&new_ans) {
                    hs.insert(new_ans.clone());
                    currans.push(new_ans);
                }
            }
        }

        currans
    }
}

fn main() {
    let root_path = env!("CARGO_MANIFEST_DIR");
    println!("{root_path}");
    for i in 1..=5 {
        println!(
            "Your solution: {:?}\nActual Solution: {}",
            Solution::generate_parenthesis(i),
            std::fs::read_to_string(format!("{root_path}/src/testcase{i}.txt")).unwrap()
        );
    }
}
