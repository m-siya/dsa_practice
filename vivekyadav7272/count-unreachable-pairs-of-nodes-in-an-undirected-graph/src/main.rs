struct Solution;

use std::collections::HashMap;
struct DSU {
    rank: Vec<usize>,
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            rank: vec![0; n],
            parent: (0..n).collect(),
        }
    }

    fn get_parent(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.get_parent(self.parent[x]);
            self.parent[x]
        }
    }

    fn union(&mut self, a: usize, b: usize) {
        let up_a = self.get_parent(a);
        let up_b = self.get_parent(b);

        if up_a == up_b {
            return;
        }

        let rank_a = self.rank[up_a];
        let rank_b = self.rank[up_b];
        if rank_a > rank_b {
            self.parent[up_b] = up_a;
        } else if rank_a < rank_b {
            self.parent[up_a] = up_b;
        } else {
            self.parent[up_a] = up_b;
            self.rank[up_b] += 1;
        }
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut dsu = DSU::new(n);
        let mut edges = edges.iter();
        while let Some(&[a, b]) = edges.next().map(|vec| vec.as_slice()) {
            dsu.union(a as usize, b as usize);
        }

        let mut up_slave_count = HashMap::<usize, usize>::with_capacity(n / 4);
        (0..n).for_each(|slave| {
            let up = dsu.get_parent(slave);
            *up_slave_count.entry(up).or_default() += 1;
        });

        (up_slave_count.values().fold(0, |judai_pairs, &guccha| {
            judai_pairs + guccha * (n - guccha)
        }) / 2) as i64

        // (dsu.ultimate_parents
        //     .iter()
        //     .fold(0, |judai_pairs, (_, &guccha)| {
        //         judai_pairs + guccha * (n - guccha)
        //     })
        //     / 2) as i64
    }
}

fn main() {}
