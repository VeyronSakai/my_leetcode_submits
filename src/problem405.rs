use std::collections::VecDeque;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut num: u32 = if num < 0 {
            (2u64.pow(32) - (-num) as u64) as u32
        } else {
            num as u32
        };

        let mut queue: VecDeque<char> = VecDeque::new();

        while num > 0 {
            queue.push_front(Self::to_char((num % 16) as u8));
            num /= 16;
        }

        queue.iter().collect()
    }

    fn to_char(num: u8) -> char {
        if num < 10 {
            return char::from_digit(num as u32, 10).unwrap();
        }

        (b'a' + num - 10) as char
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_char_test1() {
        assert_eq!(Solution::to_char(10), 'a');
    }

    #[test]
    fn to_char_test2() {
        assert_eq!(Solution::to_char(11), 'b');
    }

    #[test]
    fn example1() {
        assert_eq!(Solution::to_hex(26), "1a".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::to_hex(-1), "ffffffff".to_string());
    }
}
