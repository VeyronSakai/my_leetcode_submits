use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut mp: HashMap<char, usize> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            mp.insert(c, i);
        }

        let mut start = 0;
        let mut end = 0;
        let mut ret: Vec<i32> = Vec::new();

        for (i, c) in s.chars().enumerate() {
            end = end.max(mp.get(&c).unwrap().to_owned());

            if i == end {
                ret.push((end - start + 1) as i32);
                start = i + 1;
            }
        }

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9, 7, 8]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::partition_labels("eccbbbbdec".to_string()), vec![10]);
    }
}
