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

#[derive(Clone, Copy)]
enum Day {
    RelaxDay,
    TravelDay,
    TravelledDay(i32),
}
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let costs = [costs[0], costs[1], costs[2]];
        let mut dp = [Day::RelaxDay; 366];
        for &day in days.iter() {
            dp[day as usize] = Day::TravelDay;
        }

        Self::_mincost_tickets(&costs, 0, *days.last().unwrap(), &mut dp)
    }

    fn _mincost_tickets(
        costs: &[i32; 3],
        current_day: i32,
        last_day: i32,
        dp: &mut [Day; 366],
    ) -> i32 {
        let days_provided = [1, 7, 30];

        if current_day > last_day {
            return 0;
        }

        match dp[current_day as usize] {
            Day::RelaxDay => {
                let ans = Self::_mincost_tickets(costs, current_day + 1, last_day, dp);
                dp[current_day as usize] = Day::TravelledDay(ans);
                ans
            }
            Day::TravelledDay(ans) => ans,
            Day::TravelDay => {
                let ans = (0..3).fold(i32::MAX, |min_cost, i| {
                    let cost = costs[i]
                        + Self::_mincost_tickets(
                            costs,
                            current_day + days_provided[i],
                            last_day,
                            dp,
                        );
                    min_cost.min(cost)
                });
                dp[current_day as usize] = Day::TravelledDay(ans);

                ans
            }
        }
    }
}
fn main() {
    check_eq!(
        Solution::mincost_tickets(
            vec![4, 5, 9, 11, 14, 16, 17, 19, 21, 22, 24],
            vec![1, 4, 18]
        ),
        10
    );
}
