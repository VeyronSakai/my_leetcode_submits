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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums)
    }

    fn helper(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }

        let mid = nums.len() / 2;

        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Self::helper(&nums[0..mid].to_vec()),
            right: Self::helper(&nums[mid + 1..].to_vec()),
        })))
    }
}

struct Solution;

