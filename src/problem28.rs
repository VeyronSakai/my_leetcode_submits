use crate::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.is_empty() {
            return if needle.is_empty() {
                0
            } else {
                -1
            };
        }

        if haystack == needle {
            return 0;
        }

        if haystack.len() < needle.len(){
            return -1;
        }

        for i in 0..(haystack.len() - needle.len() + 1) {
            let substring = &haystack[i..(i + needle.len())];
            if substring == needle {
                return i as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::str_str("hello.to_string()".to_string(), "ll".to_string()) => 2);
    test_macro::test_assert_eq!(example2, Solution::str_str("aaaaa".to_string(), "bba".to_string()) => -1);
    test_macro::test_assert_eq!(example3, Solution::str_str("".to_string(), "".to_string()) => 0);
    test_macro::test_assert_eq!(example4, Solution::str_str("aaaaa".to_string(), "".to_string()) => 0);
    test_macro::test_assert_eq!(example5, Solution::str_str("".to_string(), "aaa".to_string()) => -1);
    test_macro::test_assert_eq!(example6, Solution::str_str("a".to_string(), "a".to_string()) => 0);
    test_macro::test_assert_eq!(example7, Solution::str_str("abc".to_string(), "c".to_string()) => 2);
    test_macro::test_assert_eq!(example8, Solution::str_str("abb".to_string(), "abaaa".to_string()) => -1);
}