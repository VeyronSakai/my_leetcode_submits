impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize + 1;
        let mut before_dp: Vec<i32> = Vec::new();
        let mut cur_dp: Vec<i32> = Vec::new();

        before_dp.push(1);

        for i in 1..row_index {
            cur_dp = vec![1; i + 1];
            for j in 1..i {
                cur_dp[j] = before_dp[j - 1] + before_dp[j];
            }

            before_dp = cur_dp.clone();
        }

        before_dp
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }
}