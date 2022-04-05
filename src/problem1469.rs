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
    pub fn get_lonely_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        stack.push(root);

        while !stack.is_empty() {
            if let Some(value) = stack.pop() {
                if let Some(node) = value {
                    let borrowed = node.borrow();
                    match (&borrowed.left, &borrowed.right) {
                        (Some(left), Some(right)) => {
                            stack.push(Some(left.clone()));
                            stack.push(Some(right.clone()));
                        }
                        (None, Some(value)) | (Some(value), None) => {
                            let v = value.borrow();
                            ret.push(v.val);
                            stack.push(Some(value.clone()));
                        }
                        (None, None) => continue
                    }
                }
            }
        }

        ret
    }
}