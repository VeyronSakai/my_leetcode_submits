impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![i32::MAX, i32::MAX, i32::MAX]; costs.len()];

        dp[0][0] = costs[0][0];
        dp[0][1] = costs[0][1];
        dp[0][2] = costs[0][2];

        for i in 1..costs.len() {
            dp[i][0] = dp[i - 1][1].min(dp[i - 1][2]) + costs[i][0];
            dp[i][1] = dp[i - 1][0].min(dp[i - 1][2]) + costs[i][1];
            dp[i][2] = dp[i - 1][0].min(dp[i - 1][1]) + costs[i][2];
        }

        *dp[costs.len() - 1].iter().min().unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_cost(vec![vec![7, 6, 2]]), 2);
    }
}