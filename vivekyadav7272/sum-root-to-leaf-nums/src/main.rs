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

/*class Solution {
public:
    int sumNumbers(TreeNode* root) {
        return _sumNumbers(root, 0);
    }

    int _sumNumbers(TreeNode* root, int curr_sum) {
        if (root->left == nullptr && root->right == nullptr)
            return curr_sum*10 + root->val;

        int leftSum = 0;
        if (root->left)
            leftSum = _sumNumbers(root->left, curr_sum * 10 + root->val);
        int rightSum = 0;
        if (root->right)
            rightSum = _sumNumbers(root->right, curr_sum * 10 + root->val);

        return leftSum + rightSum;
    }
}; */
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::_sum_numbers(root.unwrap(), 0)
    }

    fn _sum_numbers(root: Rc<RefCell<TreeNode>>, curr_sum: i32) -> i32 {
        let root = root.borrow();
        if root.left.is_none() && root.right.is_none() {
            return curr_sum * 10 + root.val;
        }
        let left_val = if let Some(left) = &root.left {
            Self::_sum_numbers(Rc::clone(left), curr_sum * 10 + root.val)
        } else {
            0
        };
        let right_val = if let Some(right) = &root.right {
            Self::_sum_numbers(Rc::clone(right), curr_sum * 10 + root.val)
        } else {
            0
        };

        left_val + right_val
    }
}

fn main() {}
