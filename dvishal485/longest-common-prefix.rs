struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = &strs[0][..];
        for word in strs.iter().skip(1) {
            if word.len() < result.len() {
                result = &result[..word.len()];
            }
            for (i, (l1, l2)) in (word.bytes().zip(result.bytes())).enumerate() {
                if l1 != l2 && i < result.len() {
                    result = &word[..i];
                    break;
                }
            }
        }
        result.to_string()
    }
}

fn main() {
    assert!(
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]) == "fl"
    );
    assert!(
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]) == ""
    );
    assert!(
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "dog".to_string(),
            "dog".to_string()
        ]) == "dog"
    );
    assert!(Solution::longest_common_prefix(vec!["fog".to_string(), "dog".to_string()]) == "");
    assert!(Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()]) == "a");
}
