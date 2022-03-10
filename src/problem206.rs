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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut vec = Vec::new();

        let mut head = head;

        while head.is_some() {
            vec.push(head.as_ref().unwrap().val);
            head = head.unwrap().next.clone();
        }

        vec.reverse();

        Self::get_head(vec)
    }

    fn get_head(list: Vec<i32>) -> Option<Box<ListNode>> {
        if list.is_empty() {
            return None;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::reverse_list(Solution::get_head(vec![1, 2, 3, 4, 5])), Solution::get_head(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::reverse_list(Solution::get_head(vec![1, 2])), Solution::get_head(vec![2, 1]));
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::reverse_list(Solution::get_head(vec![])), Solution::get_head(vec![]));
    }
}
