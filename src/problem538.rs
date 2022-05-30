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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        let mut sum = 0;
        Self::traverse(&mut root, &mut sum);
        root
    }

    fn traverse(root: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        match root {
            None => {}
            Some(node) => {
                let mut borrowed = node.borrow_mut();
                Self::traverse(&mut borrowed.right, sum);
                *sum += borrowed.val;
                borrowed.val = *sum;
                Self::traverse(&mut borrowed.left, sum);
            }
        }
    }
}

struct Solution;