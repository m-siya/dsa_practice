macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        let expr = $left;
        let ans = $right;
        if expr != ans {
            println!(
                "`{left_stringified}` expected to be `{:?}`, but was {:?}",
                $right, expr
            );
        }
    };
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let invalid_node = Rc::new(RefCell::new(TreeNode::new(i32::MAX)));
        let mut level_order: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::with_capacity(500);
        level_order.push_back(root.expect("Root should be non-null"));

        while !level_order.is_empty() {
            let curr_level_len = level_order.len();
            println!(
                "{:?}",
                level_order
                    .iter()
                    .map(|node| node.borrow().val)
                    .collect::<Vec<i32>>()
            );
            if !Self::is_palindrome(&level_order) {
                return false;
            }
            for _ in 0..curr_level_len {
                let node = level_order.pop_front()
                            .expect("I should only try to pop when curr_level_len > 0 and only until curr_level_len");

                let node = node.borrow();
                if let Some(left) = &node.left {
                    level_order.push_back(Rc::clone(left));
                } else if node.val != i32::MAX {
                    level_order.push_back(Rc::clone(&invalid_node));
                }
                if let Some(right) = &node.right {
                    level_order.push_back(Rc::clone(right));
                } else if node.val != i32::MAX {
                    level_order.push_back(Rc::clone(&invalid_node));
                }
            }
        }
        true
    }

    pub fn is_palindrome(level_order: &VecDeque<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack: Vec<i32> = level_order.iter().map(|node| node.borrow().val).collect();
        if stack.len() == 0 {
            return true;
        }
        let mut i = 0;
        let mut j = stack.len() - 1;

        while i < j {
            if stack[i] == stack[j] {
                i += 1;
                j -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

pub fn is_vec_palindrome(stack: &[i32]) -> bool {
    // let mut stack: Vec<i32> = level_order.iter().map(|node| node.borrow().val).collect();
    if stack.len() == 0 {
        return true;
    }
    let mut i = 0;
    let mut j = stack.len() - 1;

    while i < j {
        if stack[i] == stack[j] {
            i += 1;
            j -= 1;
        } else {
            return false;
        }
    }

    true
}

fn main() {
    check_eq!(is_vec_palindrome(&[1]), true);
    check_eq!(is_vec_palindrome(&[1, 1]), true);
    check_eq!(is_vec_palindrome(&[1, 2, 1]), true);
    check_eq!(is_vec_palindrome(&[1, 2, 2, 1]), true);
    check_eq!(is_vec_palindrome(&[1, 2, 3]), false);
    check_eq!(is_vec_palindrome(&[1, 2]), false);
}
