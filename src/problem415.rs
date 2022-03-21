struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut num1 = num1.bytes().rev();
        let mut num2 = num2.bytes().rev();
        let mut v = Vec::with_capacity(num1.len().max(num2.len()) + 1);
        let mut carry = false;
        loop {
            let n1 = num1.next().map(|u| u - b'0');
            let n2 = num2.next().map(|u| u - b'0');
            if n1.is_none() && n2.is_none() && !carry {
                break;
            }
            let d = if carry { 1 } else { 0 } + n1.unwrap_or_default() + n2.unwrap_or_default();
            carry = d > 9;
            v.push((b'0' + d % 10) as char);
        }
        v.iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::add_strings("11".to_string(), "123".to_string()), "134");
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::add_strings("456".to_string(), "77".to_string()), "533");
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::add_strings("0".to_string(), "0".to_string()), "0");
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::add_strings("52".to_string(), "63".to_string()), "115");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::add_strings("11".to_string(), "123".to_string()), "134");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::add_strings("9".to_string(), "99".to_string()), "108");
    }
}