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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut list = Self::build_list(&root);

        Solution::build_ans_node(&mut list)
    }

    fn build_ans_node(list: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let val = match list.pop() {
            None => {
                return None;
            }
            Some(val) => val,
        };

        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: Solution::build_ans_node(list),
        })))
    }

    fn build_list(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut list: Vec<i32> = Vec::new();

        Solution::build_list_internal(&root, &mut list);

        list.sort();
        list.reverse();

        list
    }

    fn build_list_internal(root: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        match root {
            None => { return; }
            Some(node) => {
                list.push(node.borrow().val);
                Solution::build_list_internal(&node.borrow().left, list);
                Solution::build_list_internal(&node.borrow().right, list);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_list_test() {
        let root = get_mock_root();

        assert_eq!(Solution::build_list(&root), vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn example1() {
        let root = get_mock_root();

        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 6,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                    val: 7,
                                    left: None,
                                    right: Some(Rc::new(RefCell::new(TreeNode {
                                        val: 8,
                                        left: None,
                                        right: Some(Rc::new(RefCell::new(TreeNode {
                                            val: 9,
                                            left: None,
                                            right: None,
                                        }))),
                                    }))),
                                }))),
                            }))),
                        }))),
                    }))),
                }))),
            }))),
        })));

        assert_eq!(Solution::increasing_bst(root), result);
    }

    fn get_mock_root() -> Option<Rc<RefCell<TreeNode>>> {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
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
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        root
    }
}