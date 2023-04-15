struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let graph = Self::build_graph(roads);
        Self::dfs(&graph, usize::MAX, 0, seats).1
    }

    fn dfs(graph: &Vec<Vec<i32>>, parent_node: usize, curr_node: usize, seats: i32) -> (i64, i64) {
        let ans = graph[curr_node]
            .iter()
            .fold((1, 0), |(total_ppl, total_cost), &next_node| {
                if next_node as usize == parent_node {
                    return (total_ppl, total_cost); // Graph is bidirectional, avoid touching your parents as that is illegal.
                }
                let (ppl_it_got, its_cost_in_its_tree) =
                    Self::dfs(&graph, curr_node, next_node as usize, seats);
                (
                    total_ppl + ppl_it_got,
                    total_cost + its_cost_in_its_tree + Self::cost_of_carryin(ppl_it_got, seats),
                )
            });
        ans
    }

    fn cost_of_carryin(homies: i64, seats: i32) -> i64 {
        (homies - 1) / (seats as i64) + 1
    }

    fn build_graph(roads: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![vec![]; roads.len() + 1];
        for road in roads {
            assert!(road.len() == 2);
            assert!((road[0] as usize) < graph.len());
            assert!((road[1] as usize) < graph.len());
            graph[road[1] as usize].push(road[0]);
            graph[road[0] as usize].push(road[1]);
        }
        graph
    }
}

macro_rules! vec_vec {
    [$([$a:expr, $b:expr]),*] => {
        vec![$(vec![$a, $b]),*]
    }
}

fn main() {
    dbg!(Solution::minimum_fuel_cost(
        vec_vec![[0, 1], [0, 2], [0, 3]],
        5
    ));
    dbg!(Solution::minimum_fuel_cost(
        vec_vec![[3, 1], [3, 2], [1, 0], [0, 4], [0, 5], [4, 6]],
        2
    ));
}
