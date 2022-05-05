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
    pub fn delete_nodes(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        Self::delete_nodes_internal(&head, m, n, 1)
    }

    fn delete_nodes_internal(head: &Option<Box<ListNode>>, m: i32, n: i32, cur_count: i32) -> Option<Box<ListNode>> {
        return match head {
            None => {
                None
            }
            Some(node) => {
                if cur_count < m {
                    Some(Box::new(ListNode {
                        val: node.val,
                        next: Self::delete_nodes_internal(&node.next, m, n, cur_count + 1),
                    }))
                } else {
                    let mut next = node.next.clone();

                    for _ in 0..n {
                        match next {
                            None => {
                                next = None;
                            }
                            Some(x) => {
                                next = x.next;
                            }
                        }
                    }
                    Some(Box::new(ListNode {
                        val: node.val,
                        next: Self::delete_nodes_internal(&next, m, n, 1),
                    }))
                }
            }
        };
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
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode {
                                    val: 7,
                                    next: Some(Box::new(ListNode {
                                        val: 8,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode {
                                                val: 10,
                                                next: Some(Box::new(ListNode {
                                                    val: 11,
                                                    next: Some(Box::new(ListNode {
                                                        val: 12,
                                                        next: Some(Box::new(ListNode {
                                                            val: 13,
                                                            next: None,
                                                        })),
                                                    })),
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let want = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 7,
                        next: Some(Box::new(ListNode {
                            val: 11,
                            next: Some(Box::new(ListNode {
                                val: 12,
                                next: None,
                            })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(Solution::delete_nodes(head, 2, 3), want);
    }
}