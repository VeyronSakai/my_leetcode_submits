struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut cur_sum: i32 = 0;
        for num in &mut nums {
            cur_sum += *num;
            *num = cur_sum;
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    }
}