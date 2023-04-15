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

struct Solution;
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.val.cmp(&self.val))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

use std::collections::BinaryHeap;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut pq: BinaryHeap<Option<Box<ListNode>>> =
            lists.into_iter().filter(|x| x.is_some()).collect();

        let mut head = None;
        let mut tail = &mut head;

        while let Some(mut min_node) = pq.pop() {
            if let Some(min_node_next) = (&mut min_node).as_mut()?.next.take() {
                pq.push(Some(min_node_next));
            }
            *tail = min_node;
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}

fn main() {
    println!("Hello, world!");
}
