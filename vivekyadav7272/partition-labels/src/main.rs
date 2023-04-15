fn lex_order(c: u8) -> usize {
    {
        let c = c as char;
        assert!('a' <= c && c <= 'z');
    }
    (c - 'a' as u8) as usize
}

pub fn partition_labels(s: String) -> Vec<i32> {
    
    let s = s.as_bytes();
    let mut end_idxs = [i32::MIN; 26];
    let mut res = vec![];
    for (i, &char) in s.iter().enumerate() {
        end_idxs[lex_order(char)] = i as i32;
    }

    let mut i = 0usize;
    
    while i < s.len() {

        let mut end_idx = end_idxs[lex_order(s[i])];
        let mut length = 0;
        while i <= end_idx as usize {
            end_idx = end_idx.max(end_idxs[lex_order(s[i])]);
            i += 1;
            length += 1;
        }
        res.push(length);
    }
    res
}

fn main() {
    println!("Hello, world!");
}