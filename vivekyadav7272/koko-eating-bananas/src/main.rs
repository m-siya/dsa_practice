struct Solution;
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
        } else {
            println!("`{left_stringified}` is correctly {expr:?}");
        }
    };
}
#[cfg(debug_assertions)]
macro_rules! dbg_print {
    ($garbage:expr) => {
        dbg!($garbage)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_print {
    ($garbage:expr) => {
        $garbage
    };
}
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let min_speed = (piles.iter().sum::<i32>() / h).max(1); // it would be sum(piles) / h but let's say it's 1 for now.
        let max_speed = piles.iter().max().cloned().unwrap();

        let mut min = min_speed;
        let mut max = max_speed;
        let mut middle = min_speed + (max_speed - min_speed) / 2;

        while min < max {
            dbg_print!((min, max));
            let hours = dbg_print!(Self::hours_taken(&piles, middle));
            if hours <= h {
                max = middle;
            } else {
                min = middle + 1;
            }
            middle = dbg_print!((min + max) / 2);
        }

        middle
    }

    fn hours_taken(piles: &[i32], eating_rate: i32) -> i32 {
        piles
            .iter()
            .map(|&x| (x / eating_rate) + ((x % eating_rate != 0) as i32))
            .sum()
    }
}

fn foo(prob: &[i32], h: i32) {
    println!("{prob:?}");
    let cons_estimate = (prob.iter().sum::<i32>() / h).max(1);
    println!("Conservative estimate: {cons_estimate}");
    let correction = prob.iter().map(|&x| x + x % cons_estimate).sum::<i32>() / h;
    println!("Corrected estimate: {correction}");
}

fn main() {
    let prob1 = vec![3, 6, 7, 11];
    foo(&prob1, 8);
    check_eq!(Solution::min_eating_speed(prob1.clone(), 8), 4);
    let prob2 = vec![30, 11, 23, 4, 20];
    foo(&prob2, 5);
    check_eq!(Solution::min_eating_speed(prob2, 5), 30);
    let prob3 = vec![30, 11, 23, 4, 20];
    foo(&prob3, 6);
    check_eq!(Solution::min_eating_speed(prob3, 6), 23);
    check_eq!(Solution::min_eating_speed(vec![312884470], 968709470), 1200);
}
