

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; 5]; n];
        dp[0][0] = 1;
        dp[0][1] = 1;
        dp[0][2] = 1;
        dp[0][3] = 1;
        dp[0][4] = 1;

        for i in 1..n {
            dp[i][0] = dp[i - 1][0];
            dp[i][1] = dp[i - 1][0] + dp[i - 1][1];
            dp[i][2] = dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2];
            dp[i][3] = dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2] + dp[i - 1][3];
            dp[i][4] = dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][2] + dp[i - 1][3] + dp[i - 1][4];
        }

        dp[n - 1][0] + dp[n - 1][1] + dp[n - 1][2] + dp[n - 1][3] + dp[n - 1][4]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::count_vowel_strings(2), 15);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }
}
