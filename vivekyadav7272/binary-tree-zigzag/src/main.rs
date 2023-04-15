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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(10); // chosen completely arbitrary, i just have a hunch
        let mut breadth_first = VecDeque::with_capacity(10); // " " " " " " " " " "
        if let Some(root) = root {
            breadth_first.push_back(root);
        }
        let mut is_even = true;
        while !breadth_first.is_empty() {
            let mut level_order: Vec<i32> = Vec::with_capacity(3); //  " " " " " " " " " "

            for _ in 0..breadth_first.len() {
                let node = if is_even {
                    breadth_first.pop_front().unwrap()
                } else {
                    breadth_first.pop_back().unwrap()
                };
                level_order.push(node.borrow().val);

                if is_even {
                    if let Some(left) = node.borrow().left.as_ref() {
                        breadth_first.push_back(Rc::clone(left));
                    }
                    if let Some(right) = node.borrow().right.as_ref() {
                        breadth_first.push_back(Rc::clone(right));
                    }
                } else {
                    if let Some(right) = node.borrow().right.as_ref() {
                        breadth_first.push_front(Rc::clone(right));
                    }
                    if let Some(left) = node.borrow().left.as_ref() {
                        breadth_first.push_front(Rc::clone(left));
                    }
                }
            }
            is_even = !is_even;
            result.push(level_order);
        }
        result
    }
}

fn main() {
    // Solution::zigzag_level_order()
}
