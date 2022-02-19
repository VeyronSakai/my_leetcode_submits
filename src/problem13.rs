use crate::Solution;

impl Solution {
    fn get_num(c: &char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => {
                unreachable!()
            }
        }
    }

    fn subtracts(cur_char: &char, next_char: &char) -> bool {
        match cur_char {
            'I' => {
                if *next_char == 'V' || *next_char == 'X' {
                    return true;
                }
            }
            'X' => {
                if *next_char == 'L' || *next_char == 'C' {
                    return true;
                }
            }
            'C' => {
                if *next_char == 'D' || *next_char == 'M' {
                    return true;
                }
            }
            _ => {
                return false;
            }
        }

        false
    }

    fn calculate(s: &String, ret: &mut i32, pointer: &mut usize, cur_char: &char) {
        if *pointer + 1 < s.len() {
            let next_char = s.chars().nth(*pointer + 1).unwrap();
            if Solution::subtracts(&cur_char, &next_char) {
                *ret += Solution::get_num(&next_char) - Solution::get_num(&cur_char);
                *pointer += 2;
            } else {
                *ret += Solution::get_num(&cur_char);
                *pointer += 1;
            }
        } else {
            *ret += Solution::get_num(&cur_char);
            *pointer += 1;
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut ret = 0;

        // 何文字目まで読み進んだか
        let mut pointer = 0;

        while pointer < s.len() {
            let cur_char = s.chars().nth(pointer).unwrap();
            Self::calculate(&s, &mut ret, &mut pointer, &cur_char);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::*;

    test_macro::test_assert_eq!(test1, Solution::roman_to_int("I".to_string()) => 1);
    test_macro::test_assert_eq!(test2, Solution::roman_to_int("V".to_string()) => 5);
    test_macro::test_assert_eq!(test3, Solution::roman_to_int("X".to_string()) => 10);
    test_macro::test_assert_eq!(test4, Solution::roman_to_int("L".to_string()) => 50);
    test_macro::test_assert_eq!(test5, Solution::roman_to_int("C".to_string()) => 100);
    test_macro::test_assert_eq!(test6, Solution::roman_to_int("D".to_string()) => 500);
    test_macro::test_assert_eq!(test7, Solution::roman_to_int("M".to_string()) => 1000);

    test_macro::test_assert_eq!(test8, Solution::roman_to_int("II".to_string()) => 2);
    test_macro::test_assert_eq!(test9, Solution::roman_to_int("III".to_string()) => 3);
    test_macro::test_assert_eq!(test10, Solution::roman_to_int("IV".to_string()) => 4);
    test_macro::test_assert_eq!(test11, Solution::roman_to_int("IX".to_string()) => 9);
    test_macro::test_assert_eq!(test12, Solution::roman_to_int("IVI".to_string()) => 5);
    test_macro::test_assert_eq!(test13, Solution::roman_to_int("LVIII".to_string()) => 58);
    test_macro::test_assert_eq!(test14, Solution::roman_to_int("MCMXCIV".to_string()) => 1994);
}