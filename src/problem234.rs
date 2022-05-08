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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut array = Vec::new();

        let mut cur = head.as_ref();

        while let Some(x) = cur {
            array.push(x.val);
            cur = x.next.as_ref();
        }

        let (mut l_ptr, mut r_ptr) = (0, array.len() - 1);

        while l_ptr < r_ptr {
            if array[l_ptr] != array[r_ptr] {
                return false;
            }

            l_ptr += 1;
            r_ptr -= 1;
        }

        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: None,
                    })),
                })),
            })),
        }));

        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn example2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None,
            })),
        }));

        assert_eq!(Solution::is_palindrome(head), false);
    }
}