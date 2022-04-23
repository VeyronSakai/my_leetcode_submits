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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut left_vec: Vec<i32> = Vec::new();

        Self::dfs(&root, &mut left_vec);

        left_vec.iter().sum()
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        match root {
            None => {
                return;
            }
            Some(node) => {
                let l = &node.borrow().left;
                if l.is_some() && l.as_ref().unwrap().borrow().left.is_none() && l.as_ref().unwrap().borrow().right.is_none() {
                    vec.push(node.borrow().left.as_ref().unwrap().borrow().val);
                }


                Self::dfs(&node.borrow().left, vec);
                Self::dfs(&node.borrow().right, vec);
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

        assert_eq!(Solution::sum_of_left_leaves(root), 24);
    }

    #[test]
    fn example2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));

        assert_eq!(Solution::sum_of_left_leaves(root), 0);
    }
}