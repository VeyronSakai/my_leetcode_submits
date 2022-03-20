use std::mem::swap;

struct Solution;


#[derive(Debug, Clone, PartialEq)]
struct StrobogrammaticError {
    pub number: char,
}

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let mut tmp: Vec<char> = num.clone().chars().collect();
        for i in 0..num.len() / 2 {
            // i文字目とnum.len() - i 文字目を回転させて交換する

            let mut rotated_char_1 = match Solution::rotate_single_num(&tmp[i]) {
                Ok(x) => { x }
                Err(_) => { return false; }
            };

            let mut rotated_char_2 = match Solution::rotate_single_num(&tmp[num.len() - i - 1]) {
                Ok(x) => { x }
                Err(_) => { return false; }
            };

            tmp[num.len() - i - 1] = rotated_char_1;
            tmp[i] = rotated_char_2;
        }

        if num.len() % 2 == 1 {
            tmp[num.len() / 2] = match Solution::rotate_single_num(&tmp[num.len() / 2]) {
                Ok(x) => x,
                Err(_) => { return false; }
            };
        }

        let str = tmp.iter().collect::<String>();

        return str == num;
    }

    fn rotate_single_num(num: &char) -> Result<char, StrobogrammaticError> {
        let ret = match *num {
            '0' => '0',
            '1' => '1',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => {
                return Err(StrobogrammaticError { number: *num });
            }
        };

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_num_test1() {
        assert_eq!(Solution::rotate_single_num(&'0'), Ok('0'));
    }

    #[test]
    fn rotate_num_test2() {
        assert_eq!(Solution::rotate_single_num(&'1'), Ok('1'));
    }

    #[test]
    fn rotate_num_test3() {
        assert_eq!(Solution::rotate_single_num(&'6'), Ok('9'));
    }

    #[test]
    fn rotate_num_test4() {
        assert_eq!(Solution::rotate_single_num(&'4'), Err(StrobogrammaticError { number: '4' }));
    }

    #[test]
    fn example1() {
        assert_eq!(Solution::is_strobogrammatic("69".to_string()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_strobogrammatic("88".to_string()), true);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::is_strobogrammatic("962".to_string()), false);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::is_strobogrammatic("2".to_string()), false);
    }
}