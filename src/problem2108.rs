struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in &words {
            if Self::is_palindrome(word.clone()) {
                return word.clone();
            }
        }

        "".to_string()
    }

    fn is_palindrome(str: String) -> bool {
        let mut i = 0;
        let mut j = str.len() - 1;

        let chars = str.chars().collect::<Vec<_>>();

        while i < j {
            if chars[i] != chars[j] {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::first_palindrome(vec!["abc".to_string(), "car".to_string(), "ada".to_string(), "racecar".to_string(), "cool".to_string()]), "ada".to_string());
    }
}
