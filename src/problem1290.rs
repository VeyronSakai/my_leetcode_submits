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

struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut cur_node = head;
        let mut ret = 0;

        while cur_node.is_some() {
            ret = ret << 1;
            ret += cur_node.as_ref().unwrap().val;
            cur_node = cur_node.unwrap().next;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: None,
                })),
            })),
        }));

        assert_eq!(Solution::get_decimal_value(head), 5);
    }
}