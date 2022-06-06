use crate::problem1305::TreeNode;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Self::helper(&l1, &l2, 0);
    }

    fn helper(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        return match (l1, l2) {
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val + carry;
                if sum > 9 {
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Self::helper(&l1.next, &l2.next, 1),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Self::helper(&l1.next, &l2.next, 0),
                    }))
                }
            }
            (Some(l), None) => {
                let sum = l.val + carry;
                if sum > 9 {
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Self::helper(&l.next, &None, 1),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Self::helper(&l.next, &None, 0),
                    }))
                }
            }
            (None, Some(l)) => {
                let sum = l.val + carry;
                if sum > 9 {
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Self::helper(&None, &l.next, 1),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Self::helper(&None, &l.next, 0),
                    }))
                }
            }
            (None, None) => {
                if carry == 1 {
                    Some(Box::new(ListNode {
                        val: 1,
                        next: None,
                    }))
                } else {
                    None
                }
            }
        };
    }
}

struct Solution;