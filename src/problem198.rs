impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];

        dp[0] = nums[0];

        if nums.len() == 1 {
            return nums[0];
        }

        dp[1] = nums[1];
        if nums.len() == 2 {
            return dp[0].max(dp[1]);
        }

        dp[2] = dp[0] + nums[2];
        if nums.len() == 3 {
            return dp[2].max(dp[1]);
        }

        for i in 3..nums.len() {
            dp[i] = (dp[i - 2] + nums[i]).max(dp[i - 3] + nums[i]);
        }

        dp[nums.len() - 1].max(dp[nums.len() - 2])
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::rob(vec![1]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::rob(vec![1, 3]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }
}