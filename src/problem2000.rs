impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut first: Vec<char> = Vec::new();
        let mut last: Vec<char> = Vec::new();

        for (i, c) in word.chars().enumerate() {
            first.push(c);

            if c == ch {
                first.reverse();

                if i < word.len() - 1 {
                    last = (&word.chars().collect::<Vec<char>>()[(i + 1)..]).to_vec();
                }

                first.append(&mut last);

                break;
            }
        }

        first.iter().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::reverse_prefix("abcdefd".to_string(), 'd'), "dcbaefd".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::reverse_prefix("xyxzxe".to_string(), 'z'), "zxyxxe".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::reverse_prefix("abcd".to_string(), 'z'), "abcd".to_string());
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse_prefix("abcd".to_string(), 'd'), "dcba".to_string());
    }
}