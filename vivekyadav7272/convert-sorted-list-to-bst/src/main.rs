// Definition for singly-linked list.
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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut head = head;
        let mut cheatsheet = Vec::with_capacity(10_000);
        while let Some(node) = head {
            cheatsheet.push(node.val);
            head = node.next;
        }
        Self::vec_to_bst(&cheatsheet)
    }

    fn vec_to_bst(cheatsheet: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if cheatsheet.is_empty() {
            return None;
        }

        let middle_idx = cheatsheet.len() / 2;
        let middle = cheatsheet[middle_idx];
        let papa = Rc::new(RefCell::new(TreeNode::new(middle)));
        {
            let mut borrowed_papa = papa.borrow_mut();
            borrowed_papa.left = Self::vec_to_bst(&cheatsheet[..middle_idx]);
            borrowed_papa.right = Self::vec_to_bst(&cheatsheet[middle_idx + 1..]);
        }
        Some(papa)
    }
}

fn main() {
    println!("Hello, world!");
}
