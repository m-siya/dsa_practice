struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.iter().sum::<i32>() < cost.iter().sum() {
            return -1;
        }
        // answer definitely exists.

        let mut net_fuel = 0;
        let mut viable_station = 0;
        for (i, (&fuel, &spend)) in gas.iter().zip(cost.iter()).enumerate() {
            net_fuel += fuel - spend;
            if net_fuel < 0 {
                viable_station = i + 1;
                net_fuel = 0;
            }
        }

        if viable_station >= gas.len() {
            return -1;
        }

        // for (&fuel, &spend) in gas.iter().zip(cost.iter()).take(viable_station) {
        //     net_fuel += fuel - spend;
        //     if net_fuel < 0 {
        //         return -1;
        //     }
        // }

        viable_station as i32
    }
}

fn main() {
    println!("Hello, world!");
}
