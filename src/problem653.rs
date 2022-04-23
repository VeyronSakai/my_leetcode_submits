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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        // root -> Vec
        let mut vec: Vec<i32> = Vec::new();

        Self::build_vec(&root, &mut vec);

        for x in vec {
            if Self::find(&root, k - x, x) {
                return true;
            }
        }

        return false;
    }

    fn build_vec(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        match root {
            None => {
                return;
            }
            Some(node) => {
                vec.push(node.borrow().val);
                Self::build_vec(&node.borrow().left, vec);
                Self::build_vec(&node.borrow().right, vec);
            }
        }
    }

    fn find(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, ignore_v: i32) -> bool {
        return match root {
            None => {
                false
            }
            Some(node) => {
                if node.borrow().val == v && node.borrow().val != ignore_v {
                    return true;
                }

                if node.borrow().val > v {
                    Self::find(&node.borrow().left, v, ignore_v)
                } else {
                    Self::find(&node.borrow().right, v, ignore_v)
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(Solution::find_target(root, 9), true);
    }
}