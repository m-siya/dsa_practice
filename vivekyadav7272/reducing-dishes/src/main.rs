struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable();
        let pivot = satisfaction.partition_point(|&x| x < 0);
        let (negative_part, positive_part) = satisfaction.split_at(pivot);
        let mut neg_acc = 0;
        let mut neg_sum = 0;
        let mut max_satisfaction = i32::MIN;
        let pos_sum: i32 = positive_part.iter().sum();
        let mut pos_acc = Self::sum_part(1, positive_part);
        for &neg in [0].iter().chain(negative_part.iter().rev()) {
            neg_acc += neg + neg_sum;
            neg_sum += neg;

            let satisfaction = pos_acc + neg_acc;
            if satisfaction < max_satisfaction {
                break;
            }
            pos_acc += pos_sum;
            max_satisfaction = satisfaction;
        }

        max_satisfaction
    }

    fn sum_part(i: usize, part: &[i32]) -> i32 {
        part.iter()
            .fold((i, 0i32), |(curr_i, curr_sum), &num| {
                (curr_i + 1, curr_sum + curr_i as i32 * num)
            })
            .1
    }
}
fn main() {
    println!("Hello, world!");
}
