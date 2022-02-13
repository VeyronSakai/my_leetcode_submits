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

// macro_rules! test_macro {
//     ($name:ident, $arg:expr => $ret:expr) => {
//         #[test]
//         fn $name() {
//             assert_eq!($arg, $ret);
//         }
//     }
// }

// test_macro!(test1, Solution::is_palindrome(1) => true);
// test_macro!(test2, Solution::is_palindrome(121) => true);
// test_macro!(test3, Solution::is_palindrome(-121) => false);
// test_macro!(test4, Solution::is_palindrome(-121) => true);

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
