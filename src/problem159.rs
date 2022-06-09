use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let mut ret = 0;
        let mut mp: HashMap<char, usize> = HashMap::new();

        let mut left = 0;
        let mut right = 0;
        let chars = s.chars().collect::<Vec<_>>();

        while right < s.len() {
            mp.insert(chars[right], right);
            if mp.keys().len() <= 2 {
                ret = ret.max(right - left + 1);
            } else {
                if mp[&chars[left]] == left {
                    mp.remove(&chars[left]);
                }
                left += 1;
            }
            right += 1;
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
        assert_eq!(Solution::length_of_longest_substring_two_distinct("eceba".to_string()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::length_of_longest_substring_two_distinct("ccaabbb".to_string()), 5);
    }
}
