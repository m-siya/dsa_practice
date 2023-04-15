struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut valid_path: Vec<&str> = Vec::with_capacity(path.len() / 10);

        let subpaths = path.split('/').skip(1);
        for path in subpaths {
            match path {
                "." | "" => continue,
                ".." => {
                    valid_path.pop();
                }
                _ => valid_path.push(path),
            }
        }

        let mut valid_path_str = String::from('/');
        let mut dabbled_in_the_dark_arts = false;
        for valid_p in valid_path {
            dabbled_in_the_dark_arts = true;
            valid_path_str.push_str(valid_p);
            valid_path_str.push('/');
        }

        if dabbled_in_the_dark_arts {
            valid_path_str.pop();
        }

        valid_path_str
    }
}
fn main() {
    println!(
        "{}",
        Solution::simplify_path("/home/foo/bar/../../puss".to_string())
    );
}
