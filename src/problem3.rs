use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();

        let mut left = 0;

        let mut ret = 0;

        for (i, c) in s.chars().enumerate() {
            if map.contains_key(&c) {
                left = left.max(*map.get(&c).unwrap() as usize + 1);
            }

            map.insert(c, i);
            ret = ret.max(i - left + 1);
        }

        ret as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    test_macro::test_assert_eq!(example2, Solution::length_of_longest_substring("bbbbb".to_string()) => 1);
    test_macro::test_assert_eq!(example3, Solution::length_of_longest_substring("pwwkew".to_string()) => 3);
    test_macro::test_assert_eq!(test1, Solution::length_of_longest_substring("aab".to_string()) => 2);
    test_macro::test_assert_eq!(test2, Solution::length_of_longest_substring("dvdf".to_string()) => 3);
    test_macro::test_assert_eq!(test3, Solution::length_of_longest_substring("tmmzuxt".to_string()) => 5);
}