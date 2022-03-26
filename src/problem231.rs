struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let n = n as i64;
        let mut tmp = 1 as i64;
        while tmp <= n {
            if tmp == n {
                return true;
            }

            tmp = 2 * tmp;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_power_of_two(1), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_power_of_two(16), true);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::is_power_of_two(3), false);
    }
}