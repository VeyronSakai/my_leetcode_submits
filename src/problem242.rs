struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = s;
        let mut t = t;

        let mut s_chars = s.chars().collect::<Vec<_>>();
        s_chars.sort();
        s = s_chars.iter().collect();

        let mut t_chars = t.chars().collect::<Vec<_>>();
        t_chars.sort();
        t = t_chars.iter().collect();
        return s == t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_anagram(String::from("anagram"), String::from("nagaram")), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_anagram(String::from("rat"), String::from("car")), false);
    }
}