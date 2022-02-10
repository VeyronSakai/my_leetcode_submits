// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode {
//             next: None,
//             val,
//         }
//     }
// }
//
// struct Solution;
//
// impl Solution {
//     pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut node1: Box<ListNode> = Box::new(ListNode::new(0));
//         let mut node2: Box<ListNode> = Box::new(ListNode::new(0));
//
//         match l1 {
//             Some(val) => {
//                 node1 = val;
//             }
//             None => ()
//         }
//
//         match l2 {
//             Some(val) => {
//                 node2 = val;
//             }
//             None => ()
//         }
//
//         let mut num1 = 0;
//         let mut num2 = 0;
//
//         let mut ten = 1;
//
//         loop {
//             num1 += node1.val * ten;
//             ten *= 10;
//
//             match node1.next {
//                 Some(next) => {
//                     node1 = next;
//                 }
//                 None => {
//                     break;
//                 }
//             }
//         }
//
//         ten = 1;
//
//         loop {
//             num2 += node2.val * ten;
//             ten *= 10;
//
//             match node2.next {
//                 Some(next) => {
//                     node2 = next;
//                 }
//                 None => {
//                     break;
//                 }
//             }
//         }
//
//         // let mut ans : Option<Box<ListNode>> = Some()
//
//         return Some(Box::new(ListNode::new(0)));
//     }
// }
//
// #[test]
// fn add_two_numbers_test() {
//     let vec = vec![2, 4, 3];
//     let mut node = ListNode::new(*vec.first().unwrap());
//
//     for v in &vec {
//
//     }
//
//     assert_eq!(vec![0, 1], Solution::add_two_numbers());
// }