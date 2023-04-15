use rand::{rngs::ThreadRng, seq::SliceRandom};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {
    list_nodes: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut list_nodes = Vec::with_capacity(5000);
        let mut traversal = head;
        while let Some(node) = traversal {
            list_nodes.push(node.val);
            traversal = node.next;
        }
        Self {
            list_nodes,
            rng: rand::thread_rng(),
        }
    }

    fn get_random(&mut self) -> i32 {
        *self.list_nodes.choose(&mut self.rng).unwrap()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

fn main() {}
