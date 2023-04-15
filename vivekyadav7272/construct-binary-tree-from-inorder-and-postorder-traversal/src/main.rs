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

use std::ops::Range;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn get(x: i32) -> usize {
        (x + 3000) as usize
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut hashmapuh: Vec<usize> = vec![usize::MAX; 6002];
        inorder.iter().enumerate().for_each(|(ind, &x)| hashmapuh[Self::get(x)] = ind);
        Self::_build_tree(0..inorder.len(), &postorder, &hashmapuh)
    }
    
    fn _build_tree(inorder: Range<usize>, postorder: &[i32], hashmapuh: &[usize]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root: Option<(usize, i32)> = None;
        let mut postorder = postorder;
        while let Some((&node, next_posorder)) = postorder.split_last() {
            let ind = hashmapuh[Self::get(node)];
            if inorder.contains(&ind) {
                root = Some((ind, node));
                break;
            }
            postorder = next_posorder;
        }
        
        root.and_then(|(ind, root)| 
            Some(Rc::new(RefCell::new(TreeNode {
                val: root,
                left: Self::_build_tree(inorder.start..ind, postorder, hashmapuh),
                right: Self::_build_tree(ind+1..inorder.end, postorder, hashmapuh),
            }))))
    }
}

fn main() {
    println!("Hello, world!");
}
