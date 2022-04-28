struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut l = 0;
        let mut h = s.len() as i32;
        let mut ret: Vec<i32> = Vec::new();

        for c in s.chars() {
            match c {
                'I' => {
                    ret.push(l);
                    l += 1;
                }
                'D' => {
                    ret.push(h);
                    h -= 1;
                }
                _ => {
                    unreachable!()
                }
            }
        }

        ret.push(l);

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::di_string_match("IDID".to_string()), vec![0, 4, 1, 3, 2]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::di_string_match("III".to_string()), vec![0, 1, 2, 3]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::di_string_match("DDI".to_string()), vec![3, 2, 0, 1]);
    }
}