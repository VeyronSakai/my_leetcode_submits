impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let column_size = matrix[0].len();

        let mut dp = vec![vec![i32::MAX; column_size]; matrix.len()];

        for i in 0..column_size {
            dp[0][i] = matrix[0][i];
        }

        for i in 1..matrix.len() {
            dp[i][0] = dp[i - 1][0].min(dp[i - 1][1]) + matrix[i][0];

            for j in 1..column_size - 1 {
                dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j + 1]) + matrix[i][j];
            }

            dp[i][column_size - 1] = dp[i - 1][column_size - 2].min(dp[i - 1][column_size - 1]) + matrix[i][column_size - 1];
        }

        *dp[matrix.len() - 1].iter().min().unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]), -59);
    }
}