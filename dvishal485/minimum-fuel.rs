struct Solution;
impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        if roads.is_empty() {
            return 0;
        }
        let mut fuel = 0;
        let mut graph = vec![vec![]; roads.len() + 1];
        for road in roads {
            assert!(road.len() == 2);
            graph[road[0] as usize].push(road[1]);
            graph[road[1] as usize].push(road[0]);
        }

        Self::helper(&graph, 0, -1, seats, &mut fuel);
        fuel
    }
    fn helper(graph: &Vec<Vec<i32>>, curr: i32, prev: i32, seats: i32, fuel: &mut i64) -> i32 {
        let mut travellers = 0;
        for &next in graph[curr as usize].iter() {
            if next != prev {
                travellers += Self::helper(graph, next, curr, seats, fuel);
            }
        }
        if curr > 0 {
            *fuel += (travellers / seats) as i64 + 1;
        }
        travellers + 1
    }
}

fn main() {
    assert!(Solution::minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5) == 3);
    assert!(
        Solution::minimum_fuel_cost(
            vec![
                vec![3, 1],
                vec![3, 2],
                vec![1, 0],
                vec![0, 4],
                vec![0, 5],
                vec![4, 6]
            ],
            2
        ) == 7
    );
    assert!(Solution::minimum_fuel_cost(vec![], 1) == 0);
}
