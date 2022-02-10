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

#[test]
pub fn it_works() {
    #[derive(Debug)]
    struct TestCase {
        args: i32,
        expected: bool,
    }

    let table = [
        TestCase {
            args: 1,
            expected: true,
        },
        TestCase {
            args: 121,
            expected: true,
        },
        TestCase {
            args: -121,
            expected: false,
        },
        TestCase {
            args: 234,
            expected: false,
        },
    ];

    for test_case in table {
        assert_eq!(Solution::is_palindrome(test_case.args), test_case.expected);
    }
}