struct Solution;

fn ord(c: u8) -> u8 {
    c - b'a'
}

struct Soln {
    colors: String,
    graph: Vec<Vec<i32>>,
    dp: Vec<Option<[i32; 26]>>,
    global_visited: Vec<bool>,
    local_visited: Vec<bool>,
}

impl Soln {
    pub fn new()
}

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();

        // build out the graph
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n];
        for edge in edges {
            if let &[a, b] = edge.as_slice() {
                graph[a as usize].push(b);
            }
        }

        let mut dp = vec![None::<[i32; 26]>; n];
        let mut global_visited = vec![false; n];
        let mut local_visited = vec![false; n];

        let mut max_count = [0i32; 26];

        for i in 0..n {
            if !global_visited[i] {
                Self::_largest_path_value(
                    colors.as_bytes(),
                    &graph,
                    &mut dp,
                    &mut global_visited,
                    &mut local_visited,
                    i as i32,
                );

                if let Some(ref max_color_freq) = dp[i] {
                    for (freq_i, &freq_j) in max_count.iter_mut().zip(max_color_freq) {
                        *freq_i = (*freq_i).max(freq_j);
                    }
                } else {
                    return -1;
                }
            }
        }

        *max_count.iter().max().unwrap()
    }

    fn _largest_path_value(
        colors: &[u8],
        graph: &Vec<Vec<i32>>,
        dp: &mut Vec<Option<[i32; 26]>>,
        global_visited: &mut Vec<bool>,
        local_visited: &mut Vec<bool>,
        curr_node: i32,
    ) {
        if local_visited[curr_node as usize] {
            // oops cycle detected. sed
            dp[curr_node as usize] = None;
            return;
        }

        if dp[curr_node as usize].is_some() {
            return;
        }

        let neighbours = &graph[curr_node as usize];

        let my_color = ord(colors[curr_node as usize]);

        let mut max_freqs = [0i32; 26];

        global_visited[curr_node as usize] = true;
        local_visited[curr_node as usize] = true;

        for &neighbour in neighbours {
            Self::_largest_path_value(colors, graph, dp, global_visited, local_visited, neighbour);

            if let Some(ref padosi_max_freq) = dp[neighbour as usize] {
                for (max_freq_i, &max_freq_j) in max_freqs.iter_mut().zip(padosi_max_freq) {
                    *max_freq_i = (*max_freq_i).max(max_freq_j);
                }
            } else {
                dp[curr_node as usize] = None;
                return;
            }
        }

        max_freqs[my_color as usize] += 1;

        local_visited[curr_node as usize] = false;
        dp[curr_node as usize] = Some(max_freqs);
    }
}

fn main() {}
