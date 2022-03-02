use std::collections::HashSet;
use crate::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut left = 0;
        let mut right = 0;

        let mut set: HashSet<char> = HashSet::new();

        let s2_chars = s2.chars().collect::<Vec<char>>();

        for i in 0..s1.len() {
            set.insert(s2_chars[i]);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()) => true);
    test_macro::test_assert_eq!(example2, Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()) => false);
}