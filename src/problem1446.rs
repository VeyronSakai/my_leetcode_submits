use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut pre_char = '_';
        let mut cur_power = 0;
        let mut max_power = 0;

        let vec = s.chars().collect::<Vec<_>>();

        for char in vec {
            if pre_char == char {
                cur_power += 1;
            } else {
                pre_char = char;
                cur_power = 1;
            }

            max_power = max(max_power, cur_power);
        }

        max_power
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_power(String::from("leetcode")), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_power(String::from("abbcccddddeeeeedcba")), 5);
    }
}

