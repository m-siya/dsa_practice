use std::cmp::Ordering::*;
fn j(ch: char) -> usize {
    let ind = (ch as u32 - 'a' as u32) as usize;
    assert!(ind < 26);
    ind
}
struct Solution;
impl Solution {
    pub fn is_alien_sorted(q: Vec<String>, o: String) -> bool {
        let mut d = [0; 26];
        o.char_indices().for_each(|(i, c)| d[j(c)] = i);

        for t in q.windows(2) {
            let a = &t[0];
            let b = &t[1];

            let mut y = a.chars();
            let mut z = b.chars();

            loop {
                match (y.next(), z.next()) {
                    (Some(g), Some(h)) => match d[j(g)].cmp(&d[j(h)]) {
                        Less => break,
                        Equal => (),
                        Greater => return false,
                    },
                    (Some(_), None) => return false,
                    _ => break,
                }
            }
        }

        true
    }
}

fn main() {
    dbg!(Solution::is_alien_sorted(
        vec!["hl".into(), "h".into()],
        String::from("hlabcdefgijkmnopqrstuvwxyz")
    ));
    dbg!(Solution::is_alien_sorted(
        vec!["hello".into(), "leetcode".into()],
        String::from("hlabcdefgijkmnopqrstuvwxyz")
    ));
    dbg!(Solution::is_alien_sorted(
        vec!["word".into(), "world".into(), "row".into()],
        String::from("worldabcefghijkmnpqstuvxyz")
    ));
    dbg!(Solution::is_alien_sorted(
        vec!["apple".into(), "app".into()],
        String::from("abcdefghijklmnopqrstuvwxyz")
    ));
}
