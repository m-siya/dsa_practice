struct Solution;
impl Solution {
    fn helper(digits: &str, res: &mut Vec<String>, message: String) {
        if digits.is_empty() {
            return res.push(message);
        }
        let char = match digits.bytes().next().unwrap() {
            b'2' => "abc",
            b'3' => "def",
            b'4' => "ghi",
            b'5' => "jkl",
            b'6' => "mno",
            b'7' => "pqrs",
            b'8' => "tuv",
            b'9' => "wxyz",
            _ => "",
        };
        for c in char.chars() {
            Self::helper(&digits[1..], res, message.clone() + &c.to_string());
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = vec![];
        if digits.is_empty() {
            return res;
        }
        Self::helper(&digits, &mut res, "".to_string());
        res
    }
}

fn main() {
    assert!(
        Solution::letter_combinations("23".to_string())
            == vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}
