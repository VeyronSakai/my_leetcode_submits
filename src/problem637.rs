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
use std::collections::HashMap;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut val_map: HashMap<usize, Vec<i32>> = HashMap::new();

        let mut cur_level = 0;

        Self::build(&root, &mut val_map, cur_level);

        let mut ret: Vec<f64> = Vec::new();

        let mut vec: Vec<(&usize, &Vec<i32>)> = val_map.iter().collect();
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        vec.iter().for_each(|(_, x)| ret.push(x.iter().map(|&x| x as i64).sum::<i64>() as f64 / x.len() as f64));

        ret
    }

    fn build(root: &Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<usize, Vec<i32>>, cur_level: usize) {
        match root {
            None => {
                return;
            }
            Some(node) => {
                let mut vec = map.entry(cur_level).or_insert(vec![]);
                vec.push(node.borrow().val);

                Self::build(&node.borrow().left, map, cur_level + 1);
                Self::build(&node.borrow().right, map, cur_level + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(Solution::average_of_levels(root), vec![3.00000, 14.50000, 11.00000]);
    }
}