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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow_node = &head;
        let mut fast_node = &head;

        while fast_node.is_some() && slow_node.is_some() {
            match &fast_node.as_ref().unwrap().next {
                None => {
                    break;
                }
                Some(_) => {
                    slow_node = &slow_node.as_ref().unwrap().next;
                    fast_node = &fast_node.as_ref().unwrap().next.as_ref().unwrap().next
                }
            }
        }

        slow_node.clone()
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use crate::problem876::ListNode;
    use super::Solution;

    test_macro::test_assert_eq!(example1, Solution::middle_node(head(vec![1, 2, 3, 4, 5])) => head(vec![3,4,5]));
    test_macro::test_assert_eq!(example2, Solution::middle_node(head(vec![1,2,3,4,5,6])) => head(vec![4,5,6]));

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