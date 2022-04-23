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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut node_cache: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

        node_cache.push(root);

        while !node_cache.is_empty() {
            let mut tmp_node_cache: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
            let mut value_cache: Vec<Option<i32>> = Vec::new();

            for n in &node_cache {
                match n {
                    None => {
                        value_cache.push(None);
                    }
                    Some(x) => {
                        value_cache.push(Some(x.borrow().val));

                        tmp_node_cache.push(x.borrow().left.clone());
                        tmp_node_cache.push(x.borrow().right.clone());
                    }
                }
            }

            // is value_cache symmetry
            for i in 0..value_cache.len() / 2 {
                if value_cache[i] != value_cache[value_cache.len() - i - 1] {
                    return false;
                }
            }

            node_cache.clear();
            node_cache = tmp_node_cache.clone();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
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
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(Solution::is_symmetric(root), true);
    }

    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
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
        })));

        assert_eq!(Solution::is_symmetric(root), false);
    }
}