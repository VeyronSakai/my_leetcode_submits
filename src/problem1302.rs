// // Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
//
// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::VecDeque;
//
// impl Solution {
//     pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
//         queue.push_front(root);
//
//         while !queue.is_empty() {
//
//         }
//
//         0
//     }
// }
//
// struct Solution;
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn example1() {
//         let root = Some(Rc::new(RefCell::new(TreeNode {
//             val: 1,
//             left: Some(Rc::new(RefCell::new(TreeNode {
//                 val: 2,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 4,
//                     left: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 7,
//                         left: None,
//                         right: None,
//                     }))),
//                     right: None,
//                 }))),
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 5,
//                     left: None,
//                     right: None,
//                 }))),
//             }))),
//             right: Some(Rc::new(RefCell::new(TreeNode {
//                 val: 3,
//                 left: None,
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 6,
//                     left: None,
//                     right: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 8,
//                         left: None,
//                         right: None,
//                     }))),
//                 }))),
//             }))),
//         })));
//
//         assert_eq!(Solution::deepest_leaves_sum(root), 15);
//     }
// }
