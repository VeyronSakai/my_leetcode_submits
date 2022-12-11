struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n as usize];

        if n == 1 {
            return 1;
        }

        dp[0] = 1;
        dp[1] = 2;

        for i in 0..n - 2 {
            let i = i as usize;
            dp[i + 2] = dp[i] + dp[i + 1];
        }

        dp[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }
}
