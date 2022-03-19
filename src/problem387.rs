use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();

        for char in s.chars() {
            let count = map.entry(char).or_insert(0);
            *count += 1;
        }

        for (i, char) in s.chars().enumerate() {
            if *map.get(&char).unwrap() == 1 {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::first_uniq_char(String::from("leetcode")), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::first_uniq_char(String::from("loveleetcode")), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::first_uniq_char(String::from("aabb")), -1);
    }
}