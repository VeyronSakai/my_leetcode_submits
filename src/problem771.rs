use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut set: HashSet<char> = HashSet::new();

        for c in jewels.chars() {
            set.insert(c);
        }

        stones.chars().into_iter().filter(|x| set.contains(x)).count() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }
}