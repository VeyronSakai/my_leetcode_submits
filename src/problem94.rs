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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        Self::inorder_traversal_internal(&root, &mut ret);

        ret
    }

    fn inorder_traversal_internal(root: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        match root {
            None => {
                return;
            }
            Some(node) => {
                Self::inorder_traversal_internal(&node.borrow().left, list);
                list.push(node.borrow().val);
                Self::inorder_traversal_internal(&node.borrow().right, list);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));

        let ret = vec![1, 3, 2];

        assert_eq!(Solution::inorder_traversal(root), ret);
    }
}