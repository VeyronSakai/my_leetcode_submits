struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut vec = Vec::new();

        for char in s.chars() {
            vec.push(char);

            while vec.len() > 1 && vec[vec.len() - 1] == vec[vec.len() - 2] {
                vec.pop();
                vec.pop();
            }
        }

        let ret = vec.iter().collect();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::remove_duplicates("abbaca".to_string()), "ca".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::remove_duplicates("azxxzy".to_string()), "ay".to_string());
    }
}