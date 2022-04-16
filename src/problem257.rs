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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        todo!()
    }
    //
    // fn binary_tree_paths_internal(root: &Option<Rc<RefCell<TreeNode>>>, ){
    //
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn example1() {
    //     let root = Some(Rc::new(RefCell::new(TreeNode {
    //         val: 1,
    //         left: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 2,
    //             left: None,
    //             right: Some(Rc::new(RefCell::new(TreeNode {
    //                 val: 5,
    //                 left: None,
    //                 right: None,
    //             }))),
    //         }))),
    //         right: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 3,
    //             left: None,
    //             right: None,
    //         }))),
    //     })));
    //     assert_eq!(Solution::binary_tree_paths(root), vec!["1->2->5", "1->3"]);
    // }
}