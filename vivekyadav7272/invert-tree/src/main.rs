use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let mut root_ = root.borrow_mut();
        let left = Self::invert_tree(root_.left.as_mut().map(|x| Rc::clone(x)));
        let right = Self::invert_tree(root_.right.as_mut().map(|x| Rc::clone(x)));
        root_.left = right;
        root_.right = left;
        drop(root_);
        Some(root)
    }
}

fn main() {
    println!("Hello, world!");
}
