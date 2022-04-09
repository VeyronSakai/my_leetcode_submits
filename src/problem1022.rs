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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = 0;

        let mut vec = Self::get_binary_list(&root);

        for val in &mut vec {
            ret += Self::calculate_binary(val);
        }

        ret
    }

    fn calculate_binary(binary: &Vec<i32>) -> i32 {
        let mut coefficient = 1;
        let mut ret = 0;

        for i in binary.iter() {
            ret += i * coefficient;
            coefficient *= 2;
        }

        ret
    }

    fn get_binary_list(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        return match root {
            None => {
                vec![vec![]]
            }
            Some(node) => {
                match (&node.borrow().left, &node.borrow().right) {
                    (Some(l), Some(r)) => {
                        let mut left = Solution::get_binary_list(&node.borrow().left);
                        for val in &mut left {
                            val.push(node.borrow().val)
                        }

                        let mut right = Solution::get_binary_list(&node.borrow().right);
                        for val in &mut right {
                            val.push(node.borrow().val)
                        }
                        [left, right].concat()
                    }
                    (Some(val), None) | (None, Some(val)) => {
                        let mut list = Solution::get_binary_list(&Some(val.clone()));
                        for v in &mut list {
                            v.push(node.borrow().val)
                        }

                        list
                    }
                    (None, None) => {
                        vec![vec![node.borrow().val]]
                    }
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_binary_list_test() {
        let root = get_mock_root();

        assert_eq!(Solution::get_binary_list(&root), vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 1, 1], vec![1, 1, 1]])
    }

    #[test]
    fn calculate_binary_test() {
        assert_eq!(Solution::calculate_binary(&vec![1, 0, 1]), 5);
        assert_eq!(Solution::calculate_binary(&vec![0, 0, 1]), 4);
        assert_eq!(Solution::calculate_binary(&vec![1, 0, 0]), 1);
    }

    #[test]
    fn example1() {
        let root = get_mock_root();

        assert_eq!(Solution::sum_root_to_leaf(root), 22);
    }

    fn get_mock_root() -> Option<Rc<RefCell<TreeNode>>> {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        root
    }
}