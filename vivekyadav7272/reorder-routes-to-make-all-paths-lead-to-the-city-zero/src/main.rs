struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<(i32, bool)>> = vec![vec![]; n as usize];
        let mut connections = connections.iter();

        while let Some(&[a, b]) = connections.next().map(|conn| conn.as_slice()) {
            graph[a as usize].push((b, true));
            graph[b as usize].push((a, false));
        }

        Self::num_wrong_paths(0, i32::MAX, &graph)
    }

    fn num_wrong_paths(root: i32, parent: i32, graph: &Vec<Vec<(i32, bool)>>) -> i32 {
        graph[root as usize]
            .iter()
            .fold(0, |num_wrong_paths, &(child, is_original_path)| {
                num_wrong_paths
                    + if child == parent {
                        !is_original_path as i32
                    } else {
                        Self::num_wrong_paths(child, root, graph)
                    }
            })
    }
}

fn main() {
    println!("Hello, world!");
}
