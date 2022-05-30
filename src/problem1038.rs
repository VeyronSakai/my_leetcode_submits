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
use std::collections::HashMap;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        let want = Some(Rc::new(RefCell::new(TreeNode {
            val: 30,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 36,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 36,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 35,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 33,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 21,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 26,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        assert_eq!(Solution::bst_to_gst(root), want);
    }

    #[test]
    fn example2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        })));

        let want = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        })));

        assert_eq!(Solution::bst_to_gst(root), want);
    }
}