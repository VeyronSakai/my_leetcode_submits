use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let str_len = if s.len() == t.len() {
            s.len()
        } else {
            return false;
        };

        let mut mp: HashMap<char, char> = HashMap::new();

        let mut s_chars = s.chars().collect::<Vec<char>>();
        let mut t_chars = t.chars().collect::<Vec<char>>();

        for i in 0..str_len {
            if let Some(c) = mp.get(&(s_chars[i])) {
                if *c != t_chars[i] {
                    return false;
                }
            } else {
                mp.insert(s_chars[i], t_chars[i]);
            }
        }

        let mut set: HashSet<char> = HashSet::new();

        for value in mp.values() {
            if set.contains(value) {
                return false;
            } else {
                set.insert(*value);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_isomorphic("egg".to_string(), "add".to_string()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_isomorphic("foo".to_string(), "bar".to_string()), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::is_isomorphic("paper".to_string(), "title".to_string()), true);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::is_isomorphic("badc".to_string(), "bada".to_string()), false);
    }
}