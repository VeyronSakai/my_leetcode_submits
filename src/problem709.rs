impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut s = s;
        let mut chars = s.chars().collect::<Vec<_>>();
        for i in 0..s.len() {
            chars[i] = chars[i].to_ascii_lowercase();
        }

        chars.iter().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello".to_string());
    }
}