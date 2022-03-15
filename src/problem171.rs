struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut ret = 0;

        let char_vec = column_title.chars().rev().collect::<Vec<_>>();

        for i in 0..char_vec.len() {
            ret += Self::get_alphabet_num(char_vec[i]) * 26_i32.pow(i as u32);
        }

        ret
    }

    fn get_alphabet_num(alphabet: char) -> i32 {
        alphabet as i32 - ('A' as i32) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_alphabet_num_test1() {
        assert_eq!(Solution::get_alphabet_num('A'), 1);
    }

    #[test]
    fn get_alphabet_num_test2() {
        assert_eq!(Solution::get_alphabet_num('B'), 2);
    }

    #[test]
    fn example1() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    }


    #[test]
    fn example3() {
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }
}