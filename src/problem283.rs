use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut ptr = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, ptr);
                ptr += 1;
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