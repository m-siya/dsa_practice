struct Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut is_destroyed = vec![false; is_connected.len()];
        let mut army_required = 0;
        for city in (0..is_connected.len()).rev() {
            if !is_destroyed[city] {
                army_required += 1;
                Self::destroy_city(city, &is_connected, &mut is_destroyed);
            }
        }
        army_required
    }

    fn destroy_city(city: usize, is_connected: &Vec<Vec<i32>>, is_destroyed: &mut Vec<bool>) {
        is_destroyed[city] = true;
        for (city_id, &connected) in is_connected[city].iter().enumerate() {
            if connected == 1 && !is_destroyed[city_id] {
                Self::destroy_city(city_id, is_connected, is_destroyed);
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
        3
    );
    assert_eq!(
        Solution::find_circle_num(vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1]
        ]),
        1
    );
}
