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
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        Self::helper(&root1, &mut ret);
        Self::helper(&root2, &mut ret);

        ret.sort_unstable();
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
