use core::num;

struct Solution;
impl Solution {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answers = Vec::<i32>::with_capacity(queries.len());
        let mut queries = queries.iter();
        while let Some(&[a, b]) = queries.next().map(|x| x.as_slice()) {
            let mut parent_a = a;
            let mut parent_b = b;
            let mut num_edges = 0;

            while parent_a != parent_b {
                while parent_a > parent_b {
                    parent_a = Self::parent(parent_a);
                    num_edges += 1;
                }
                while parent_b > parent_a {
                    parent_b = Self::parent(parent_b);
                    num_edges += 1;
                }
            }

            answers.push(num_edges + 1);
        }

        answers
    }

    fn parent(child: i32) -> i32 {
        child / 2
    }
}

fn main() {
    println!("Hello, world!");
}
