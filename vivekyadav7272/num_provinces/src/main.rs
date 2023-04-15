#[cfg(debug_assertions)]
macro_rules! dbg_print {
    ($garbage:expr) => {
        dbg!($garbage)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dbg_print {
    ($garbage:expr) => {};
}

macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        let expr = $left;
        let ans = $right;
        print!("`{left_stringified}` expected to be `{ans}`");
        assert_eq!(expr, ans);
    };
}

struct Solution;
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut province_count = 0i32;
        let mut digested = vec![false; is_connected.len()];

        for city in 0..is_connected.len() {
            if !digested[city] {
                Self::eat_city(city, &is_connected, &mut digested);
                province_count += 1;
            }
        }

        province_count
    }

    fn eat_city(curr_city: usize, is_connected: &Vec<Vec<i32>>, digested: &mut Vec<bool>) {
        digested[curr_city] = true;
        for (city, &is_neighbour) in is_connected[curr_city].iter().enumerate() {
            if digested[city] || is_neighbour == 0 {
                continue;
            }
            Self::eat_city(city as usize, is_connected, digested);
        }
    }
}

fn main() {
    dbg!(Solution::find_circle_num(vec![
        vec![1, 1, 0],
        vec![1, 1, 0],
        vec![0, 0, 1]
    ]));

    dbg!(Solution::find_circle_num(vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1]
    ]));
}
