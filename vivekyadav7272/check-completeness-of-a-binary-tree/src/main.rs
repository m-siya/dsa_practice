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
      right: None
    }
  }
}
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
// #[allow(unused_mut, unused_variables)]
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut parents: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        let mut children: Vec<Option<Rc<RefCell<TreeNode>>>>  = Vec::new();
        // let mut parents = &mut list1;
        // let mut children = &mut list2;
        let mut null_encountered = false;
        parents.push(root);
        while !parents.is_empty() {            
            for parent in parents.iter() {
                if let Some(parent) = parent {
                    if null_encountered {
                        return false;
                    }
                    let parent = parent.borrow();
                    children.push(parent.left.clone());
                    children.push(parent.right.clone());
                } else {
                    null_encountered = true;
                }
                
            }
            parents.clear();
            std::mem::swap(&mut parents, &mut children);
            println!("{:?}", parents.iter().map(|node| node.as_ref().map(|some_node| some_node.borrow().val)).collect::<Vec<Option<i32>>>());
        }
        true
    }
}

fn main() {
    
}
