struct Solution;

impl Solution {
    fn is_vowel(c: u8) -> bool {
        match c {
            b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => true,
            _ => false,
        }
    }
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.bytes().collect::<Vec<u8>>();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if !Self::is_vowel(s[i]) {
                i += 1;
                continue;
            }
            if !Self::is_vowel(s[j]) {
                j -= 1;
                continue;
            }
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
        return String::from_utf8(s).unwrap();
    }
}

fn main() {
    assert!(Solution::reverse_vowels("hello".to_string()) == "holle");
    assert!(Solution::reverse_vowels("leetcode".to_string()) == "leotcede");
    assert!(Solution::reverse_vowels("vishal".to_string()) == "vashil");
}
