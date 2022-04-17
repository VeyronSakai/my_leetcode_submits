struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        if n == 1 {
            return 1;
        }

        Self::fib(n - 1) + Self::fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::fib(4), 3);
    }
}