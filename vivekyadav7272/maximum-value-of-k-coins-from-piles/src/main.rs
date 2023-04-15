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
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut prev: Vec<i32> = std::iter::once(&0)
            .chain(piles[0].iter())
            .take(k + 1)
            .scan(0, |cumsum, curr_elem| {
                *cumsum = *cumsum + curr_elem;
                Some(*cumsum)
            })
            .collect();

        let last_val = prev[prev.len() - 1];
        for _ in 0..(k + 1 - prev.len()) {
            prev.push(last_val);
        }

        let mut curr = vec![0; k + 1];

        for pile in &piles[1..] {
            curr[0] = 0;
            for i in 1..=k {
                let mut max_profit = 0;
                let mut cumsum = 0;
                for (num_coin, &curr_coin) in std::iter::once(&0)
                    .chain(pile.iter())
                    .take(i + 1)
                    .enumerate()
                {
                    cumsum += curr_coin;
                    max_profit = max_profit.max(cumsum + prev[i - num_coin]);
                }
                curr[i] = max_profit;
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        prev[k]
    }
}
fn main() {
    check_eq!(
        Solution::max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2),
        101
    );
    check_eq!(
        Solution::max_value_of_coins(
            vec![
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![1, 1, 1, 1, 1, 1, 700]
            ],
            7
        ),
        706
    );
    check_eq!(
        Solution::max_value_of_coins(
            vec![vec![10, 3, 5, 2, 8], vec![3, 14, 2, 7], vec![4, 4, 4]],
            4
        ),
        31
    );
    check_eq!(
        Solution::max_value_of_coins(
            vec![vec![10, 3, 5, 2, 8], vec![3, 14, 2, 7], vec![4, 4, 4]],
            5
        ),
        36
    );
    check_eq!(
        Solution::max_value_of_coins(
            vec![vec![10, 3, 5, 2, 8], vec![3, 14, 2, 7], vec![4, 4, 4]],
            6
        ),
        40
    );
    check_eq!(
        Solution::max_value_of_coins(
            vec![vec![10, 3, 5, 2, 8], vec![3, 14, 2, 7], vec![4, 4, 4]],
            2
        ),
        17
    );
    check_eq!(
        Solution::max_value_of_coins(
            vec![vec![10, 3, 5, 2, 8], vec![3, 14, 2, 7], vec![4, 4, 4]],
            1
        ),
        10
    );
    check_eq!(
        Solution::max_value_of_coins(
            vec![vec![10, 3, 5, 2, 8], vec![3, 14, 2, 7], vec![4, 4, 4]],
            10
        ),
        58
    );
}
