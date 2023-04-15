use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<(i32, i32)>> = vec![vec![]; (n + 1) as usize];
        let mut roads = roads.iter();
        // Build the graph

        while let Some(&[a, b, edge_wt]) = roads.next().map(|x| x.as_slice()) {
            graph[a as usize].push((b, edge_wt));
            graph[b as usize].push((a, edge_wt));
        }

        // now start graph traversal from 1 and visit all the nodes, and return the min.

        Self::dfs(1, &mut graph)
    }
    fn dfs(root: i32, graph: &mut Vec<Vec<(i32, i32)>>) -> i32 {
        if graph[root as usize].len() == 0 {
            return i32::MAX;
        }
        std::mem::take(&mut graph[root as usize])
            .into_iter()
            .fold(i32::MAX, |acc, (babby, edge_wt)| {
                acc.min(edge_wt).min(Self::dfs(babby, graph))
            })
    }
}

fn main() {
    println!("Hello, world!");
}
