struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut visited_by = vec![-1; edges.len()];
        let mut my_visited: Vec<i32> = vec![-1; edges.len()];

        (0..edges.len() as i32).fold(-1, |max_cycle, node| {
            max_cycle.max(Self::longest_cycle_subgraph(
                &mut visited_by,
                node,
                &edges,
                &mut my_visited,
            ))
        })
    }

    fn longest_cycle_subgraph(
        visited_by: &mut [i32],
        node: i32,
        edges: &[i32],
        my_visited: &mut [i32],
    ) -> i32 {
        let mut curr_node = node;
        let master_node = node;
        let mut depth = 0;

        while curr_node != -1 {
            let visited_from = &mut visited_by[curr_node as usize];
            if *visited_from == -1 {
                *visited_from = master_node;
            } else {
                if *visited_from != master_node {
                    return -1;
                }
                // found a cycle in our subgraph
                return depth - my_visited[curr_node as usize];
            }

            my_visited[curr_node as usize] = depth;
            depth += 1;
            curr_node = edges[curr_node as usize];
        }

        -1i32
    }
}

fn main() {
    println!("Hello, world!");
}
