impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_ptr = 0;
        let s_chars = s.chars().collect::<Vec<_>>();
        let mut t_ptr = 0;
        let t_chars = t.chars().collect::<Vec<_>>();

        while s_ptr < s.len() {
            let s_char = s_chars[s_ptr];
            while t_ptr < t.len() {
                if s_char == t_chars[t_ptr] {
                    s_ptr += 1;
                    t_ptr += 1;
                    break;
                }

                t_ptr += 1;
            }

            if s_ptr < s.len() && t_ptr == t.len() {
                return false;
            }
        }

        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()), false);
    }

    #[test]
    fn test() {
        assert_eq!(Solution::is_subsequence("aaaaaa".to_string(), "bbaaaa".to_string()), false);
    }
}