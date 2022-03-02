use std::cmp::max;
use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();

        let mut left = 0;

        let mut ret = 0;

        let chars = s.chars().collect::<Vec<char>>();
        let chars_len = chars.len();

        for i in 0..chars_len {
            if map.contains_key(&chars[i]) {
                left = max(*map.get(&chars[i]).unwrap() as usize + 1, left);
            }

            map.insert(chars[i], i);
            ret = max(ret, i - left + 1);
        }

        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::length_of_longest_substring("abcabcbb".to_string()) => 3);
    test_macro::test_assert_eq!(example2, Solution::length_of_longest_substring("bbbbb".to_string()) => 1);
    test_macro::test_assert_eq!(example3, Solution::length_of_longest_substring("pwwkew".to_string()) => 3);
    test_macro::test_assert_eq!(test1, Solution::length_of_longest_substring("aab".to_string()) => 2);
    test_macro::test_assert_eq!(test2, Solution::length_of_longest_substring("dvdf".to_string()) => 3);
    test_macro::test_assert_eq!(test3, Solution::length_of_longest_substring("tmmzuxt".to_string()) => 5);
}