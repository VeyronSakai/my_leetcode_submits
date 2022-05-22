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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vec: Vec<i32> = Vec::new();
        Self::helper(&root, &mut vec);

        let mut ret = i32::MAX;

        vec.sort_unstable();

        for i in 1..vec.len() {
            ret = ret.min(vec[i] - vec[i - 1]);
        }

        ret
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        match root {
            None => {}
            Some(node) => {
                vec.push(node.borrow().val);
                Self::helper(&node.borrow().left, vec);
                Self::helper(&node.borrow().right, vec);
            }
        }
    }
}

struct Solution;