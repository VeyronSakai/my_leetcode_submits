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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bst_from_preorder_internal(&preorder)
    }

    fn bst_from_preorder_internal(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        return match v.first() {
            None => {
                None
            }
            Some(first) => {
                let threshold_idx = v.iter().position(|&v| v > *first).unwrap_or(v.len());
                Some(Rc::new(RefCell::new(TreeNode {
                    val: *first,
                    left: Self::bst_from_preorder_internal(&v[1..threshold_idx]),
                    right: Self::bst_from_preorder_internal(&v[threshold_idx..]),
                })))
            }
        };
    }
}

struct Solution;