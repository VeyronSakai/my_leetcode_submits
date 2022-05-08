use std::collections::HashSet;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        for i in (1..=s.len()).rev() {
            for j in 0..s.len() - i {
                let mut set: HashSet<char> = HashSet::new();

                for c in s[j..=j + i].chars() {
                    set.insert(c);
                }

                let contains = set.iter().all(|x| (x.is_ascii_lowercase() && set.contains(&x.to_ascii_uppercase())) || (x.is_ascii_uppercase() && set.contains(&x.to_ascii_lowercase())));

                if contains {
                    return s[j..=j + i].to_string();
                }
            }
        }

        "".to_string()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::longest_nice_substring("YazaAay".to_string()), "aAa".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::longest_nice_substring("Bb".to_string()), "Bb".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::longest_nice_substring("c".to_string()), "".to_string());
    }
}