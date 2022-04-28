struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ptr1 = 0;
        let mut ptr2 = 0;

        let mut ret = String::new();

        let word1_chars = word1.chars().collect::<Vec<_>>();
        let word2_chars = word2.chars().collect::<Vec<_>>();

        while ptr1 < word1.len() || ptr2 < word2.len() {
            if ptr1 < word1.len() {
                ret.push(word1_chars[ptr1]);
                ptr1 += 1;
            }

            if ptr2 < word2.len() {
                ret.push(word2_chars[ptr2]);
                ptr2 += 1;
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
        assert_eq!(Solution::merge_alternately("abc".to_string(), "pqr".to_string()), "apbqcr".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::merge_alternately("ab".to_string(), "pqrs".to_string()), "apbqrs".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::merge_alternately("abcd".to_string(), "pq".to_string()), "apbqcd".to_string());
    }
}