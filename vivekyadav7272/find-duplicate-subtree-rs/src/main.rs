struct Solution;

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

macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        let left_stringified = stringify!($left);
        println!("`{left_stringified}` expected to be `{}`", $right);
        let expr = $left;
        let ans = $right;
        assert_eq!(expr, ans);
    };
}

use std::cell::RefCell;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::Hasher;
use std::rc::Rc;
impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut visited_trees = HashMap::<u64, bool>::new();
        let mut subtrees = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();

        Self::_find_duplicate_subtrees(root, &mut subtrees, &mut visited_trees);

        subtrees
    }

    fn _find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
        subtrees: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        visited_trees: &mut HashMap<u64, bool>,
    ) -> u64 {
        if let Some(root_) = root {
            let root = root_.borrow();
            let left_tree =
                Self::_find_duplicate_subtrees(root.left.clone(), subtrees, visited_trees);

            let right_tree =
                Self::_find_duplicate_subtrees(root.right.clone(), subtrees, visited_trees);

            let mut hasher = DefaultHasher::new();
            hasher.write_u64(left_tree);
            hasher.write_i32(root.val);
            hasher.write_u64(right_tree);
            let tree_hash = hasher.finish();
            visited_trees
                .entry(tree_hash)
                .and_modify(|is_married| {
                    if *is_married {
                        return;
                    }
                    *is_married = true;
                    subtrees.push(Some(Rc::clone(&root_)));
                })
                .or_insert(false);
            tree_hash
        } else {
            0
        }
    }
}

fn main() {
    println!("Hello, world!");
}
