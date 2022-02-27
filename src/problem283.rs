use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        for mut i in 0..nums.len() - 1 {
            for j in 0..nums.len() - 1 - i {
                if nums[j] == 0 {
                    nums[j] = nums[j + 1];
                    nums[j + 1] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::Solution;

    #[test]
    fn example1() {
        let mut nums = vec![0, 1, 0, 3, 12];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn example3() {
        let mut nums = vec![0, 0, 1];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, vec![1, 0, 0]);
    }
}