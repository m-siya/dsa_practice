use std::cell::RefCell;
use std::rc::Rc;

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
struct Solution {}
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MAX;
        Self::get_min_diff(&root.unwrap(), &mut res)
    }
    fn get_min_diff(root: &Rc<RefCell<TreeNode>>, current_min: &mut i32) -> i32 {
        let root_val = root.borrow().val;
        match (
            root.borrow().left.to_owned(),
            root.borrow().right.to_owned(),
        ) {
            (None, None) => *current_min,
            (None, Some(right)) => {
                *current_min = (Self::leftmost(&right) - root_val).min(*current_min);
                Self::get_min_diff(&right, current_min).min(*current_min)
            }
            (Some(left), None) => {
                *current_min = (root_val - Self::rightmost(&left)).min(*current_min);
                Self::get_min_diff(&left, current_min).min(*current_min)
            }
            (Some(left), Some(right)) => {
                *current_min = (*current_min)
                    .min(root_val - Self::rightmost(&left))
                    .min(Self::leftmost(&right) - root_val);
                *current_min = Self::get_min_diff(&left, current_min);
                Self::get_min_diff(&right, current_min).min(*current_min)
            }
        }
    }
    fn leftmost(root: &Rc<RefCell<TreeNode>>) -> i32 {
        match root.borrow().left.to_owned() {
            Some(left) => Self::leftmost(&left),
            None => root.borrow().val,
        }
    }
    fn rightmost(root: &Rc<RefCell<TreeNode>>) -> i32 {
        match root.borrow().right.to_owned() {
            Some(right) => Self::rightmost(&right),
            None => root.borrow().val,
        }
    }
}

fn main() {
    assert_eq!(
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        })))),
        1
    );

    assert_eq!(
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
        })))),
        1
    );

    assert_eq!(
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 90,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 69,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(89)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(91)))),
        })))),
        1
    );
}
