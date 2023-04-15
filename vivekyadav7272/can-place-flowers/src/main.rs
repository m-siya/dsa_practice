macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        let expr = $left;
        let ans = $right;
        if expr != ans {
            println!(
                "`{left_stringified}` expected to be `{:?}`, but was {:?}",
                $right, expr
            );
        }
    };
}

struct Solution;

impl Solution {
    fn get(flowerbed: &Vec<i32>, i: isize) -> i32 {
        if i < 0 || (i as usize) >= flowerbed.len() {
            return 0;
        }
        flowerbed[i as usize]
    }
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed;
        let mut flowers_left = n;

        for i in 0..flowerbed.len() as isize {
            if flowers_left == 0 {
                break;
            }
            if Self::get(&flowerbed, i - 1) == 0
                && Self::get(&flowerbed, i + 1) == 0
                && flowerbed[i as usize] == 0
            {
                flowerbed[i as usize] = 1;
                flowers_left -= 1;
            }
        }

        flowers_left == 0
    }
}

fn main() {
    check_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    check_eq!(
        Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
        false
    );
    check_eq!(
        Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 0, 1], 2),
        true
    );
    check_eq!(
        Solution::can_place_flowers(vec![0, 0, 0, 0, 1, 0, 1], 2),
        true
    );
    check_eq!(
        Solution::can_place_flowers(vec![0, 0, 0, 1, 0, 0, 0, 1], 2),
        true
    );
    check_eq!(
        Solution::can_place_flowers(vec![0, 0, 1, 0, 0, 0, 1], 2),
        true
    );
}
