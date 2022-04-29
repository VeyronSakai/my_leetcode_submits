struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s;
        let (mut l, mut r) = (0, s.len() - 1);
        let mut chars = s.clone().into_bytes();
        while l < r {
            if Self::is_vowel(chars[l]) && Self::is_vowel(chars[r]) {
                chars.swap(l, r);

                if l < s.len() {
                    l += 1;
                }

                if r > 0 {
                    r -= 1;
                }
            }

            if !Self::is_vowel(chars[l]) {
                if l < s.len() {
                    l += 1;
                }
            }

            if !Self::is_vowel(chars[r]) {
                if r > 0 {
                    r -= 1;
                }
            }
        }

        String::from_utf8(chars).unwrap()
    }

    fn is_vowel(c: u8) -> bool {
        match c {
            b'a' | b'A' | b'i' | b'I' | b'u' | b'U' | b'e' | b'E' | b'o' | b'O' => {
                true
            }
            _ => {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
    }

    #[test]
    fn is_vowel_test1() {
        assert_eq!(Solution::is_vowel(b'a'), true);
    }

    #[test]
    fn is_vowel_test2() {
        assert_eq!(Solution::is_vowel(b's'), false);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse_vowels("a.".to_string()), "a.".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa".to_string());
    }
}