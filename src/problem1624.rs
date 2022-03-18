use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();

        let mut ret: i32 = -1;

        for (i, char) in s.chars().enumerate() {
            if map.contains_key(&char) {
                ret = max(ret, i as i32 - *map.get(&char).unwrap() - 1)
            } else {
                map.insert(char, i as i32);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_length_between_equal_characters(String::from("aa")), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_length_between_equal_characters(String::from("abca")), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_length_between_equal_characters(String::from("cbzxy")), -1);
    }
}