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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        let mut ret = root.as_ref().unwrap().borrow().val;
        Self::helper(&root, target, &mut ret);
        ret
    }

    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, target: f64, ret: &mut i32) {
        match root {
            None => {}
            Some(n) => {
                if (*ret as f64 - target).abs() > (n.borrow().val as f64 - target).abs() {
                    *ret = n.borrow().val;
                }

                Self::helper(&n.borrow().left, target, ret);
                Self::helper(&n.borrow().right, target, ret);
            }
        }
    }
}

struct Solution;