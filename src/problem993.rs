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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        false
    }

    // // Are
    // fn is_cousins_by_node(root: Option<Rc<RefCell<TreeNode>>>, x: Option<Rc<RefCell<TreeNode>>>, y: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn example1() {
//         let root = Some(Rc::new(RefCell::new(
//             TreeNode {
//                 val: 1,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 2,
//                     left: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 4,
//                         left: None,
//                         right: None,
//                     }))),
//                     right: None,
//                 }))),
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 3,
//                     left: None,
//                     right: None,
//                 }))),
//             })));
//
//
//         assert_eq!(Solution::is_cousins(root, 4, 3), false);
//     }
//
//     #[test]
//     fn example2() {
//         let root = Some(Rc::new(RefCell::new(
//             TreeNode {
//                 val: 1,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 2,
//                     left: None,
//                     right: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 4,
//                         left: None,
//                         right: None,
//                     }))),
//                 }))),
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 3,
//                     left: None,
//                     right: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 5,
//                         left: None,
//                         right: None,
//                     }))),
//                 }))),
//             })));
//
//
//         assert_eq!(Solution::is_cousins(root, 5, 4), true);
//     }
//
//     #[test]
//     fn example3() {
//         let root = Some(Rc::new(RefCell::new(
//             TreeNode {
//                 val: 1,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 2,
//                     left: None,
//                     right: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 4,
//                         left: None,
//                         right: None,
//                     }))),
//                 }))),
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 3,
//                     left: None,
//                     right: None,
//                 }))),
//             })));
//
//
//         assert_eq!(Solution::is_cousins(root, 2, 3), false);
//     }
// }