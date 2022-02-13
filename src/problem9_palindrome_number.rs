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
    use test_macro::*;
    use super::*;
    test_eq!(test1, Solution::is_palindrome(1) => true);
    test_eq!(test2, Solution::is_palindrome(121) => true);
    test_eq!(test3, Solution::is_palindrome(-121) => false);
    // test_eq!(test4, Solution::is_palindrome(-121) => false);
}

#[test]
pub fn it_works() {
    #[derive(Debug)]
    struct TestCase {
        args: i32,
        expected: bool,
        name: String,
    }

    let table = [
        TestCase {
            args: 1,
            expected: true,
            name: String::from("テスト1"),
        },
        TestCase {
            args: 121,
            expected: true,
            name: String::from("テスト2"),
        },
        TestCase {
            args: -121,
            expected: false,
            name: String::from("テスト3"),
        },
        TestCase {
            args: 234,
            expected: false,
            name: String::from("テスト4"),
        },
    ];

    for test_case in table {
        assert_eq!(Solution::is_palindrome(test_case.args), test_case.expected);
    }
}
