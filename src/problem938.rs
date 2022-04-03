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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Self::range_sum_bst_internal(&root, low, high)
    }

    fn range_sum_bst_internal(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            Some(node) => {
                let borrowed = node.borrow();
                let l = Self::range_sum_bst_internal(&borrowed.left, low, high);
                let r = Self::range_sum_bst_internal(&borrowed.right, low, high);

                let node_val = if low <= borrowed.val && borrowed.val <= high {
                    borrowed.val
                } else {
                    0
                };

                return node_val + l + r;
            }
            None => 0,
        }
    }
}