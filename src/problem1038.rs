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
        let mut vec: Vec<i32> = Vec::new();
        Self::traverse(&root, &mut vec);
        let mut mp = Self::build_map(&vec);
        let mut root = root.clone();
        Self::update_tree(&mut root, &mp);
        root
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        match root {
            None => {}
            Some(node) => {
                vec.push(node.borrow().val);
                Self::traverse(&node.borrow().left, vec);
                Self::traverse(&node.borrow().right, vec);
            }
        }
    }

    fn update_tree(root: &mut Option<Rc<RefCell<TreeNode>>>, mp: &HashMap<i32, i32>) {
        match root {
            None => {}
            Some(node) => {
                let mut borrowed = node.borrow_mut();
                borrowed.val = *mp.get(&borrowed.val).unwrap();
                Self::update_tree(&mut borrowed.left, &mp);
                Self::update_tree(&mut borrowed.right, &mp);
            }
        }
    }

    fn build_map(input: &Vec<i32>) -> HashMap<i32, i32> {
        let mut input = input.clone();
        input.sort_unstable();
        let mut sum = 0;
        let mut ret: HashMap<i32, i32> = HashMap::new();

        for val in input.iter().rev() {
            sum += val;
            ret.insert(*val, sum);
        }

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_map_test() {
        let mut mp = HashMap::new();
        mp.insert(4, 30);
        mp.insert(1, 36);
        mp.insert(6, 21);
        mp.insert(0, 36);
        mp.insert(2, 35);
        mp.insert(3, 33);
        mp.insert(5, 26);
        mp.insert(7, 15);
        mp.insert(8, 8);

        assert_eq!(Solution::build_map(&vec![4, 1, 6, 0, 2, 5, 7, 3, 8]), mp);
    }

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