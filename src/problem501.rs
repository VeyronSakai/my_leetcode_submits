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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut root = root;
        let mut mp: HashMap<i32, usize> = HashMap::new();
        Self::helper(&mut root, &mut mp);
        let max_val = *mp.values().max().unwrap();
        mp.iter().filter(|&(&key, &val)| val == max_val).map(|x| *x.0).collect()
    }

    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, mp: &mut HashMap<i32, usize>) {
        match node {
            None => {}
            Some(n) => {
                let mut count = mp.entry(n.borrow().val).or_insert(0);
                *count += 1;

                Self::helper(&n.borrow().left, mp);
                Self::helper(&n.borrow().right, mp);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));

        assert_eq!(Solution::find_mode(root), vec![2]);
    }

    #[test]
    fn example2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));

        assert_eq!(Solution::find_mode(root), vec![0]);
    }
}