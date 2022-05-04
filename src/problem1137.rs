impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = vec![0; 38];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;

        for i in 3..38 {
            dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
        }

        dp[n as usize]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}