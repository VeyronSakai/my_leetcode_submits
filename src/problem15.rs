use std::env::var;
use crate::Solution;

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode {
            val: 0,
            next: head,
        }));
        let mut len = 0;

        {
            let mut p = dummy_head.as_ref();
            while p.unwrap().next.is_some() {
                len += 1;
                p = p.unwrap().next.as_ref();
            }
        }

        let idx = len - n;
        {
            let mut p = dummy_head.as_mut();
            for _ in 0..(idx) {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = next;
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::remove_nth_from_end(head(vec![1,2,3,4,5]), 2) => head(vec![1,2,3,5]));

    fn head(list: Vec<i32>) -> Option<Box<ListNode>> {
        let mut cur: Option<Box<ListNode>> = None;

        cur = Some(Box::new(ListNode::new(*list.last().unwrap())));
        cur.as_mut().unwrap().next = None;

        for i in 1..list.len() {
            let mut node = ListNode {
                next: cur.clone(),
                val: list[list.len() - i - 1],
            };

            cur = Some(Box::new(node));
        }

        cur
    }
}