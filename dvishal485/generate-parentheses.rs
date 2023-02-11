struct Solution;
impl Solution {
    fn _generate_parenthesis(opening: i32, closing: i32, s: String, res: &mut Vec<String>) {
        if opening == 0 && closing == 0 {
            return res.push(s);
        }
        if opening == 0 {
            return Self::_generate_parenthesis(opening, closing - 1, s + ")", res);
        }
        if opening == closing {
            return Self::_generate_parenthesis(opening - 1, closing, s + "(", res);
        }
        Self::_generate_parenthesis(opening - 1, closing, s.clone() + "(", res);
        Self::_generate_parenthesis(opening, closing - 1, s + ")", res);
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Self::_generate_parenthesis(n, n, String::new(), &mut result);
        result
    }
}

fn main() {
    assert!(Solution::generate_parenthesis(1) == vec!["()"]);
    assert!(Solution::generate_parenthesis(2) == vec!["(())", "()()"]);
    assert!(
        Solution::generate_parenthesis(3) == vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
}
