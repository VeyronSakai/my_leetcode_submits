struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s_chars = s.chars().collect::<Vec<_>>();
        s_chars.sort();

        let mut t_chars = t.chars().collect::<Vec<_>>();
        t_chars.sort();


        for i in 0..s.len() {
            if s_chars[i] != t_chars[i] {
                return t_chars[i];
            }
        }

        return *t_chars.last().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_the_difference(String::from("abcd"), String::from("abcde")), 'e');
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_the_difference(String::from(""), String::from("y")), 'y');
    }
}