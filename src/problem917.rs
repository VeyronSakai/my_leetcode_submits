struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut s = s;
        let mut l = 0;
        let mut r = s.len() - 1;

        let mut chars = s.chars().collect::<Vec<_>>();

        while l < r {
            if chars[l].is_alphabetic() && chars[r].is_alphabetic() {
                chars.swap(l, r);
                l += 1;
                r -= 1;
            }

            if !chars[l].is_alphabetic() {
                l += 1;
            }

            if !chars[r].is_alphabetic() {
                r -= 1;
            }
        }

        chars.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::reverse_only_letters("ab-cd".to_string()), "dc-ba".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()), "j-Ih-gfE-dCba".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()), "Qedo1ct-eeLg=ntse-T!".to_string());
    }
}

