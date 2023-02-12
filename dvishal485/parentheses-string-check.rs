struct Solution;
impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() & 1 == 1 {
            return false;
        }
        let (mut potential_open, mut potential_close) = (0, 0);
        for (c, lock) in s.bytes().zip(locked.bytes()) {
            if c == b'(' || lock == b'0' {
                potential_open += 1;
            } else {
                potential_close += 1;
            }
            if potential_close > potential_open {
                return false;
            }
        }
        (potential_open, potential_close) = (0, 0);
        for (c, lock) in s.bytes().rev().zip(locked.bytes().rev()) {
            if c != b'(' || lock == b'0' {
                potential_close += 1;
            } else {
                potential_open += 1;
            }
            if potential_open > potential_close {
                return false;
            }
        }
        true
    }
}

fn main() {
    assert!(Solution::can_be_valid("))()))".to_string(), "010100".to_string()) == true);
    assert!(Solution::can_be_valid("()()".to_string(), "0000".to_string()) == true);
    assert!(
        Solution::can_be_valid(
            "())(()(()(())()())(())((())(()())((())))))(((((((())(()))))(".to_string(),
            "100011110110011011010111100111011101111110000101001101001111".to_string()
        ) == false
    );
    assert!(Solution::can_be_valid("((())".to_string(), "00000".to_string()) == false);
    assert!(Solution::can_be_valid(")".to_string(), "0".to_string()) == false);
    assert!(
        Solution::can_be_valid(
            "()()()(()(())()())(())((())(()())((())))))(((((((())(()))))(".to_string(),
            "10()11110110011011010111100111011101111110000101001101001111".to_string()
        ) == false
    );
    assert!(Solution::can_be_valid("()".to_string(), "11".to_string()) == true);
}
