struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        // return vec!["".to_string(); n as usize];

        let mut ret = vec!["".to_string(); n as usize];

        for (i, val) in ret.iter_mut().enumerate() {
            let j = i + 1;
            if j % 15 == 0 {
                *val = "FizzBuzz".to_string();
            } else if j % 3 == 0 {
                *val = "Fizz".to_string();
            } else if j % 5 == 0 {
                *val = "Buzz".to_string();
            } else {
                *val = j.to_string();
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
