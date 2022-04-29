struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_stack: Vec<char> = Vec::new();
        let mut t_stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if c == '#' {
                s_stack.pop();
            } else {
                s_stack.push(c);
            }
        }

        for c in t.chars() {
            if c == '#' {
                t_stack.pop();
            } else {
                t_stack.push(c);
            }
        }

        return s_stack == t_stack;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()), true);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::backspace_compare("a#c".to_string(), "b".to_string()), false);
    }
}