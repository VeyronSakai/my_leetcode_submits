use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' => {
                    vec.push('(');
                }
                ')' => {
                    match vec.pop() {
                        None => {
                            return false;
                        }
                        Some(c) => {
                            if c != '(' {
                                return false;
                            }
                        }
                    }
                }
                '[' => {
                    vec.push('[');
                }
                ']' => {
                    match vec.pop() {
                        None => {
                            return false;
                        }
                        Some(c) => {
                            if c != '[' {
                                return false;
                            }
                        }
                    }
                }
                '{' => {
                    vec.push('{');
                }
                '}' => {
                    match vec.pop() {
                        None => {
                            return false;
                        }
                        Some(c) => {
                            if c != '{' {
                                return false;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        return vec.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::*;

    test_macro::test_assert_eq!(test1, Solution::is_valid("()".to_string()) => true);
    test_macro::test_assert_eq!(test2, Solution::is_valid("()[]{}".to_string()) => true);
    test_macro::test_assert_eq!(test3, Solution::is_valid("(]".to_string()) => false);
    test_macro::test_assert_eq!(test4, Solution::is_valid("{[]}".to_string()) => true);
    test_macro::test_assert_eq!(test5, Solution::is_valid("[}".to_string()) => false);
    test_macro::test_assert_eq!(test6, Solution::is_valid("[".to_string()) => false);
}