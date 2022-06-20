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
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // match (list1, list2) {
        //     (None, None) => None,
        //     (Some(node), None) | (None, Some(node)) => Some(node),
        //     (Some(node1), Some(node2)) => {
        //         if node1.val >= node2.val {
        //             Some(Box::new(ListNode {
        //                 val: node2.val,
        //                 next: Solution::merge_two_lists(Some(node1), node2.next),
        //             }))
        //         } else {
        //             Some(Box::new(ListNode {
        //                 val: node1.val,
        //                 next: Solution::merge_two_lists(node1.next, Some(node2)),
        //             }))
        //         }
        //     }
        // }

        let (mut l1, mut l2) = (list1, list2);
        let mut head = Box::new(ListNode::new(0));
        let mut tail = Some(&mut head);

        while let Some(mut cur) = tail {
            match (l1, l2) {
                (Some(mut n1), Some(mut n2)) => {
                    if n1.val < n2.val {
                        l1 = n1.next.take(); // get the ownership back for l1
                        l2 = Some(n2); // get the ownership back for l2
                        cur.next = Some(n1);
                    } else {
                        l1 = Some(n1); // same as above
                        l2 = n2.next.take(); // same as above
                        cur.next = Some(n2);
                    }
                }
                (Some(n1), None) => {
                    cur.next = Some(n1);
                    break;
                }
                (None, Some(n2)) => {
                    cur.next = Some(n2);
                    break;
                }
                (None, None) => {
                    cur.next = None;
                    break;
                }
            }
            tail = cur.next.as_mut();
        }

        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_macro::*;

    #[test]
    fn example1() {
        let mut arg1_1 = ListNode::new(1);
        let mut arg1_2 = ListNode::new(2);
        let mut arg1_3 = ListNode::new(4);

        arg1_3.next = None;
        arg1_2.next = Some(Box::new(arg1_3));
        arg1_1.next = Some(Box::new(arg1_2));

        let mut arg2_1 = ListNode::new(1);
        let mut arg2_2 = ListNode::new(3);
        let mut arg2_3 = ListNode::new(4);

        arg2_3.next = None;
        arg2_2.next = Some(Box::new(arg2_3));
        arg2_1.next = Some(Box::new(arg2_2));

        let mut ret1 = ListNode::new(1);
        let mut ret2 = ListNode::new(1);
        let mut ret3 = ListNode::new(2);
        let mut ret4 = ListNode::new(3);
        let mut ret5 = ListNode::new(4);
        let mut ret6 = ListNode::new(4);

        ret6.next = None;
        ret5.next = Some(Box::new(ret6));
        ret4.next = Some(Box::new(ret5));
        ret3.next = Some(Box::new(ret4));
        ret2.next = Some(Box::new(ret3));
        ret1.next = Some(Box::new(ret2));

        assert_eq!(
            Solution::merge_two_lists(Some(Box::new(arg1_1)), Some(Box::new(arg2_1))),
            Some(Box::new(ret1))
        );
    }

    test_macro::test_assert_eq!(example2, Solution::merge_two_lists(None, None) => None);
    test_macro::test_assert_eq!(example3, Solution::merge_two_lists(None, Some(Box::new(ListNode::new(0)))) => Some(Box::new(ListNode::new(0))));
}
