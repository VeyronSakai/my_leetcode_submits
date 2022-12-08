struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x: i64 = x as i64;
        let mut t = 0i64;
        while (t + 1) * (t + 1) <= x {
            t = t + 1;
        }

        t as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::my_sqrt(1), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::my_sqrt(0), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::my_sqrt(2147395600), 46340);
    }
}
