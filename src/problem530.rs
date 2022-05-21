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

use std::i32::{MIN, MAX};
use std::rc::Rc;
use std::cell::RefCell;

// impl Solution {
//     pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         return Self::get_minimum_diff_internal(&root);
//     }
//
//     fn get_minimum_diff_internal(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut min_diff = i32::MAX;
//
//         match root {
//             None => {}
//             Some(n) => {
//                 let cur = n.borrow().val;
//
//                 if n.borrow().left.is_some() {
//                     min_diff = min_diff.min(cur - n.borrow().left.as_ref().unwrap().borrow().val);
//                 }
//
//                 if n.borrow().right.is_some() {
//                     min_diff = min_diff.min(n.borrow().right.as_ref().unwrap().borrow().val - cur);
//                 }
//
//                 min_diff = min_diff.min(Self::get_minimum_diff_internal(&n.borrow().left));
//                 min_diff = min_diff.min(Self::get_minimum_diff_internal(&n.borrow().right));
//             }
//         }
//
//         return min_diff;
//     }
// }

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::traverse(root, MIN, MAX).1;
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, last: i32, res: i32) -> (i32, i32) {
        if let Some(r) = root {
            let (mut last, mut res) = Self::traverse(r.borrow().left.clone(), last, res);
            res = res.min(r.borrow().val.saturating_sub(last));
            last = r.borrow().val;
            return Self::traverse(r.borrow().right.clone(), last, res);
        }
        return (last, res);
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
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
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

        assert_eq!(Solution::get_minimum_difference(root), 1);
    }
}
