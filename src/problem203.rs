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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut list = head.clone();

        let mut cur_node = list.as_mut().unwrap();

        // while let Some(x) = cur_node.next.as_mut() {
        //     if x.val == val {
        //         cur_node.next = x.next.take();
        //     } else {
        //         // これだと何故かエラーにならない
        //         cur_node = cur_node.next.as_mut().unwrap();
        //
        //         // なぜこれだとエラーになる？
        //         // cur_node = x;
        //     }
        // }

        while cur_node.val == val {
            match cur_node.next.as_mut() {
                None => {
                    return None;
                }
                Some(n) => {
                    cur_node = n;
                }
            };
        }

        while let Some(next_node) = cur_node.next.clone() {
            if next_node.val == val {
                cur_node.next = next_node.next;
            } else {
                cur_node = cur_node.next.as_mut().unwrap();
            }
        }

        list
    }
}

struct Solution;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn example1() {
//         let head = Some(Box::new(ListNode {
//             val: 1,
//             next: Some(Box::new(ListNode {
//                 val: 2,
//                 next: Some(Box::new(ListNode {
//                     val: 6,
//                     next: Some(Box::new(ListNode {
//                         val: 3,
//                         next: Some(Box::new(ListNode {
//                             val: 4,
//                             next: Some(Box::new(ListNode {
//                                 val: 5,
//                                 next: Some(Box::new(ListNode {
//                                     val: 6,
//                                     next: None,
//                                 })),
//                             })),
//                         })),
//                     })),
//                 })),
//             })),
//         }));
//
//         let want = Some(Box::new(ListNode {
//             val: 1,
//             next: Some(Box::new(ListNode {
//                 val: 2,
//                 next: Some(Box::new(ListNode {
//                     val: 3,
//                     next: Some(Box::new(ListNode {
//                         val: 4,
//                         next: Some(Box::new(ListNode {
//                             val: 5,
//                             next: None,
//                         })),
//                     })),
//                 })),
//             })),
//         }));
//
//         assert_eq!(Solution::remove_elements(head, 6), want);
//     }
//
//     #[test]
//     fn test1() {
//         let head = Some(Box::new(ListNode {
//             val: 7,
//             next: Some(Box::new(ListNode {
//                 val: 7,
//                 next: Some(Box::new(ListNode {
//                     val: 7,
//                     next: Some(Box::new(ListNode {
//                         val: 7,
//                         next: None,
//                     })),
//                 })),
//             })),
//         }));
//
//         assert_eq!(Solution::remove_elements(head, 7), None);
//     }
//
//     #[test]
//     fn test2() {
//         let head = Some(Box::new(ListNode {
//             val: 1,
//             next: Some(Box::new(ListNode {
//                 val: 2,
//                 next: None,
//             })),
//         }));
//
//         let want = Some(Box::new(ListNode {
//             val: 2,
//             next: None,
//         }));
//
//         assert_eq!(Solution::remove_elements(head, 1), want);
//     }
// }