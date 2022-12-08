use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let vec: Vec<char> = x.to_string().chars().collect();

        for i in 0..vec.len() {
            if i > vec.len() - 1 {
                break;
            }

            if vec[i] != vec[vec.len() - 1 - i] {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_macro::*;
    test_assert_eq!(test1, Solution::is_palindrome(1) => true);
    test_assert_eq!(test2, Solution::is_palindrome(121) => true);
    test_assert_eq!(test3, Solution::is_palindrome(-121) => false);
}
