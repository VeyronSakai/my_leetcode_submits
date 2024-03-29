use crate::Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // nums = 1 2 3 4 5 6 7, k = 2
        // 6 7 1 2 3 4 5 (nums.len() - k がスタート。 nums.len() - k - 1 がゴール。)

        // nums = 1 2 3 4 5 6 7, k = 10
        // 5 6 7 1 2 3 4 (nums.len() - k % nums.len() がスタート。nums.len() - k % nums.len() - 1 がゴール)

        let mut ret: Vec<i32> = Vec::new();

        let start = nums.len() - k as usize % nums.len();

        let end = if start == 0 {
            nums.len() - 1
        } else {
            start
        };

        for i in start..nums.len() {
            ret.push(nums[i]);
        }

        for i in 0..end {
            ret.push(nums[i]);
        }

        *nums = ret;
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::Solution;

    #[test]
    fn example1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];

        Solution::rotate(&mut nums, 3);

        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![-1, -100, 3, 99];

        Solution::rotate(&mut nums, 2);

        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn example3() {
        let mut nums = vec![-1];

        Solution::rotate(&mut nums, 2);

        assert_eq!(nums, vec![-1]);
    }
}