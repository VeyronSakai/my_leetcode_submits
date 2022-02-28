// use std::cmp::max;
// use std::collections::HashSet;
// use crate::Solution;
//
// impl Solution {
//     pub fn length_of_longest_substring(s: String) -> i32 {
//         let mut set: HashSet<char> = HashSet::new();
//
//         let mut ret = 0;
//         let mut tmp = 0;
//
//         for char in s.chars() {
//             if set.contains(&char) {
//                 ret = max(ret, tmp);
//                 set.clear();
//                 set.insert(char);
//                 tmp = 1;
//             } else {
//                 set.insert(char);
//                 tmp += 1;
//             }
//         }
//
//         ret = max(ret, tmp);
//
//         ret
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use test_macro::*;
//     use super::*;
//
//     test_macro::test_assert_eq!(example1, Solution::length_of_longest_substring("abcabcbb".to_string()) => 3);
//     test_macro::test_assert_eq!(example2, Solution::length_of_longest_substring("bbbbb".to_string()) => 1);
//     test_macro::test_assert_eq!(example3, Solution::length_of_longest_substring("pwwkew".to_string()) => 3);
//     test_macro::test_assert_eq!(test1, Solution::length_of_longest_substring("aab".to_string()) => 2);
//     test_macro::test_assert_eq!(test2, Solution::length_of_longest_substring("dvdf".to_string()) => 3);
// }