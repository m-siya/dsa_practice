macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        println!("`{left_stringified}` expected to be `{}`", $right);
        let expr = $left;
        let ans = $right;
        assert_eq!(expr, ans);
    };
}

mod dp;
mod fib;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test1() {
        // use dp::Solution;
        use fib::Solution;
        check_eq!(Solution::climb_stairs(2), 2);
        check_eq!(Solution::climb_stairs(3), 3);
    }
}

fn main() {}
