use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        let k = k as usize;
        if s.len() < k {
            return s.len() as i32;
        }
        let mut ret = 0;
        let mut mp: HashMap<char, usize> = HashMap::new();

        let mut left = 0;
        let mut right = 0;
        let chars = s.chars().collect::<Vec<_>>();

        while right < s.len() {
            mp.insert(chars[right], right);
            right += 1;
            if mp.keys().len() > k {
                let (&del_key, &del_val) = mp.iter().min_by_key(|&(&c, &pos)| pos).unwrap();
                mp.remove(&del_key);
                left = del_val + 1;
            }

            ret = ret.max(right - left);
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
        assert_eq!(Solution::length_of_longest_substring_k_distinct("eceba".to_string(), 2), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::length_of_longest_substring_k_distinct("aa".to_string(), 1), 2);
    }
}