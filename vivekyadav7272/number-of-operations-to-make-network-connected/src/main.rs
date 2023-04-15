struct Solution;

#[derive(Clone)]
struct Node {
    is_master: bool,
    data: usize, // could either be the index of the master if slave, or the number of slaves if master.
}

impl Node {
    fn new() -> Node {
        Node {
            is_master: true,
            data: 1,
        }
    }
}

struct DSU {
    nodes: Vec<Node>,
    num_sets: usize,
}

impl DSU {
    fn union(&mut self, node1: usize, node2: usize) {
        assert!(node1 < self.nodes.len());
        assert!(node2 < self.nodes.len());
        let master1 = self.get_master(node1);
        let master2 = self.get_master(node2);

        if master1 == master2 {
            return;
        }

        let bigger = self.bigger_master(master1, master2);
        let smaller = if bigger == master1 { master2 } else { master1 };
        let smaller = &mut self.nodes[smaller];
        smaller.is_master = false;
        let smaller_slave_count = smaller.data;
        smaller.data = bigger;
        self.nodes[bigger].data += smaller_slave_count;

        self.num_sets -= 1;
    }

    fn get_master(&mut self, node: usize) -> usize {
        if self.nodes[node].is_master {
            node
        } else {
            let true_master = self.get_master(self.nodes[node].data);
            self.nodes[node].data = true_master;
            true_master
        }
    }

    fn bigger_master(&self, master1: usize, master2: usize) -> usize {
        let master1_node = &self.nodes[master1];
        let master2_node = &self.nodes[master2];

        if master1_node.data > master2_node.data {
            master1
        } else {
            master2
        }
    }

    fn new(n: usize) -> Self {
        Self {
            nodes: vec![Node::new(); n],
            num_sets: n,
        }
    }

    fn len(&self) -> usize {
        self.num_sets
    }
}

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let num_cables = connections.len();
        if num_cables < n - 1 {
            return -1;
        }

        let mut dsu = DSU::new(n as usize);
        for connection in connections.into_iter() {
            if let &[a, b] = connection.as_slice() {
                dsu.union(a as usize, b as usize);
            }
        }
        (dsu.len() - 1) as i32
    }
}

fn main() {
    println!("Hello, world!");
}
