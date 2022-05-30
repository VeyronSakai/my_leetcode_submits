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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(&root, 0).1
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> (i32, bool) {
        match root {
            None => {
                return (sum, true);
            }
            Some(node) => {
                let s = sum + 1;
                let (s1, is_ok1) = Self::helper(&node.borrow().right, s);
                let (s2, is_ok2) = Self::helper(&node.borrow().left, s);

                if !is_ok1 || !is_ok2 {
                    return (0, false);
                }

                if (s1 - s2).abs() > 1 {
                    return (0, false);
                }

                return (s.max(s1.max(s2)), true);
            }
        }
    }
}

struct Solution;


#[test]
fn example1() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    assert_eq!(Solution::is_balanced(root), true);
}

#[test]
fn example2() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: None,
                left: None,
            }))),
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: None,
                }))),
            }))),
        }))),
    })));

    assert_eq!(Solution::is_balanced(root), false);
}

#[test]
fn example3() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            right: None,
            left: None,
        }))),
        left: None,
    })));

    assert_eq!(Solution::is_balanced(root), true);
}