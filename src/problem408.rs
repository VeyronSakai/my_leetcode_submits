struct Solution;

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let W: Vec<char> = word.as_bytes().iter().map(|x| *x as char).collect();
        let A: Vec<char> = abbr.as_bytes().iter().map(|x| *x as char).collect();
        let mut w: usize = 0;
        let mut a: usize = 0;
        while a < A.len() && w < W.len() {
            if A[a].is_digit(10) && A[a] != '0' {
                let s: usize = a;
                while a < A.len() && A[a].is_digit(10) {
                    a += 1;
                }
                let D: i32 = A[s..a].iter().collect::<String>().parse::<i32>().unwrap();
                let mut d: i32 = D;
                let q = w;
                while w < W.len() && d > 0 {
                    d -= 1;
                    w += 1;
                }
                if w - q != D as usize {
                    return false;
                }
            } else {
                if A[a] != W[w] {
                    return false;
                }
                a += 1;
                w += 1;
            }
        }
        if a < A.len() || w < W.len() {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::valid_word_abbreviation("internationalization".to_string(), "i12iz4n".to_string()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::valid_word_abbreviation("apple".to_string(), "a2e".to_string()), false);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::valid_word_abbreviation("a".to_string(), "2".to_string()), false);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::valid_word_abbreviation("internationalization".to_string(), "i5a11o1".to_string()), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::valid_word_abbreviation("hi".to_string(), "hi1".to_string()), false);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::valid_word_abbreviation("hi".to_string(), "2i".to_string()), false);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::valid_word_abbreviation("a".to_string(), "01".to_string()), false);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::valid_word_abbreviation("a".to_string(), "ab".to_string()), false);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::valid_word_abbreviation("ab".to_string(), "a".to_string()), false);
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::valid_word_abbreviation("aa".to_string(), "a".to_string()), false);
    }
}